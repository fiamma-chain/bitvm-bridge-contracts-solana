use crate::{
    errors::BtcLightClientError,
    events::{ChainReorg, NewTip},
    state::*,
    utils::{
        get_and_verify_block_hash_account, get_and_verify_period_target_account, mul_in_place,
    },
};
use anchor_lang::prelude::*;
use bitcoin::{
    block::Header as BlockHeader, consensus::encode::deserialize, hash_types::BlockHash,
    hashes::Hash,
};

pub fn submit_block_headers(
    ctx: Context<SubmitBlockHeaders>,
    block_height: u64,
    headers: Vec<u8>,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let headers: Vec<BlockHeader> =
        deserialize(&headers).map_err(|_| BtcLightClientError::InvalidHeaderLength)?;

    require!(!headers.is_empty(), BtcLightClientError::NoHeaders);

    let old_period = state.latest_block_height / 2016;
    let new_height = block_height + headers.len() as u64 - 1;
    let new_period = new_height / 2016;

    require!(
        new_period >= old_period,
        BtcLightClientError::OldDifficultyPeriod
    );

    let mut num_reorged = 0;

    for (i, header) in headers.iter().enumerate() {
        let current_height = block_height + i as u64;
        let hash = header.block_hash();
        let new_hash = hash.to_byte_array();

        // Get or create block hash account
        let block_hash_entry = get_and_verify_block_hash_account(
            &ctx.remaining_accounts[i + 1],
            current_height,
            ctx.program_id,
        )?;

        if block_hash_entry.hash != [0; 32] && block_hash_entry.hash != new_hash {
            num_reorged += 1;
        }

        // Update block hash
        let block_hash_entry = BlockHashEntry {
            height: current_height,
            hash: new_hash,
        };
        block_hash_entry
            .serialize(&mut &mut ctx.remaining_accounts[i + 1].data.borrow_mut()[..])?;

        // Verify previous block hash
        let expected_prev_hash = if i == 0 {
            let prev_block_hash_entry = get_and_verify_block_hash_account(
                &ctx.remaining_accounts[0],
                block_height - 1,
                ctx.program_id,
            )?;
            require!(
                prev_block_hash_entry.hash != [0; 32],
                BtcLightClientError::BlockNotFound
            );
            BlockHash::from_byte_array(prev_block_hash_entry.hash)
        } else {
            headers[i - 1].block_hash()
        };

        require!(
            header.prev_blockhash == expected_prev_hash,
            BtcLightClientError::InvalidPrevHash
        );

        // Verify PoW and difficulty
        let target = header.target();
        require!(
            target.is_met_by(hash),
            BtcLightClientError::InvalidProofOfWork
        );

        let current_period = current_height / 2016;
        let new_target = header.target().to_be_bytes();

        if current_height % 2016 == 0 {
            // Get previous period target
            let period_target_entry = get_and_verify_period_target_account(
                &ctx.remaining_accounts[headers.len() + 1],
                current_period - 1,
                ctx.program_id,
            )?;

            if !state.is_testnet {
                let mut t = period_target_entry.target;
                mul_in_place(&mut t, 4);
                require!(
                    new_target < t,
                    BtcLightClientError::InvalidDifficultyAdjustment
                );
            }

            // Create new period target account
            let period_target_entry = PeriodTargetEntry {
                period: current_period,
                target: new_target,
            };
            period_target_entry.serialize(
                &mut &mut ctx.remaining_accounts[headers.len() + 2].data.borrow_mut()[..],
            )?;
        } else if !state.is_testnet {
            let period_target_entry = get_and_verify_period_target_account(
                &ctx.remaining_accounts[headers.len() + 1],
                current_period,
                ctx.program_id,
            )?;
            require!(
                new_target == period_target_entry.target,
                BtcLightClientError::InvalidDifficultyAdjustment
            );
        }
    }

    // Get old tip
    let old_tip_entry = get_and_verify_block_hash_account(
        &ctx.remaining_accounts[1],
        block_height,
        ctx.program_id,
    )?;

    let new_tip = headers.last().unwrap().block_hash().to_byte_array();

    if num_reorged > 0 {
        emit!(ChainReorg {
            reorg_count: num_reorged,
            old_tip: old_tip_entry.hash,
            new_tip,
        });
    }

    state.latest_block_height = new_height;
    state.latest_block_time = headers.last().unwrap().time;

    emit!(NewTip {
        block_height: new_height,
        block_time: state.latest_block_time,
        block_hash: new_tip,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SubmitBlockHeaders<'info> {
    #[account(mut, seeds = [b"btc_light_client"], bump)]
    pub state: Account<'info, BtcLightClientState>,

    #[account(mut)]
    pub submitter: Signer<'info>,
    pub system_program: Program<'info, System>,
}
