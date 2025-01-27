use crate::{
    errors::BtcLightClientError,
    events::{ChainReorg, NewTip},
    state::*,
    utils::mul_in_place,
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
        let (block_hash_pda, _) =
            BtcLightClientState::get_block_hash_pda(current_height, ctx.program_id);
        let block_hash_account = &ctx.remaining_accounts[i + 1];
        require!(
            block_hash_account.key() == block_hash_pda,
            BtcLightClientError::InvalidPdaAccount
        );

        let old_hash = if let Ok(data) =
            BlockHashEntry::try_deserialize(&mut &block_hash_account.data.borrow()[..])
        {
            data.hash
        } else {
            [0; 32]
        };

        if old_hash != [0; 32] && old_hash != new_hash {
            num_reorged += 1;
        }

        // Update block hash
        let block_hash_entry = BlockHashEntry {
            height: current_height,
            hash: new_hash,
        };
        block_hash_entry.serialize(&mut &mut block_hash_account.data.borrow_mut()[..])?;

        // Verify previous block hash
        let expected_prev_hash = if i == 0 {
            let (prev_block_hash_pda, _) =
                BtcLightClientState::get_block_hash_pda(block_height - 1, ctx.program_id);
            let prev_block_hash_account = &ctx.remaining_accounts[0];
            require!(
                prev_block_hash_account.key() == prev_block_hash_pda,
                BtcLightClientError::InvalidPdaAccount
            );
            let prev_entry =
                BlockHashEntry::try_deserialize(&mut &prev_block_hash_account.data.borrow()[..])?;
            require!(
                prev_entry.hash != [0; 32],
                BtcLightClientError::BlockNotFound
            );
            BlockHash::from_byte_array(prev_entry.hash)
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
            let (prev_period_target_pda, _) =
                BtcLightClientState::get_period_target_pda(current_period - 1, ctx.program_id);
            let prev_period_target_account = &ctx.remaining_accounts[headers.len() + 1];
            require!(
                prev_period_target_account.key() == prev_period_target_pda,
                BtcLightClientError::InvalidPdaAccount
            );
            let prev_period_entry = PeriodTargetEntry::try_deserialize(
                &mut &prev_period_target_account.data.borrow()[..],
            )?;

            if !state.is_testnet {
                let mut t = prev_period_entry.target;
                mul_in_place(&mut t, 4);
                require!(
                    new_target < t,
                    BtcLightClientError::InvalidDifficultyAdjustment
                );
            }

            // Create new period target account
            let (period_target_pda, _) =
                BtcLightClientState::get_period_target_pda(current_period, ctx.program_id);
            let period_target_account = &ctx.remaining_accounts[headers.len() + 2];
            require!(
                period_target_account.key() == period_target_pda,
                BtcLightClientError::InvalidPdaAccount
            );

            let period_target_entry = PeriodTargetEntry {
                period: current_period,
                target: new_target,
            };
            period_target_entry.serialize(&mut &mut period_target_account.data.borrow_mut()[..])?;
        } else if !state.is_testnet {
            let (period_target_pda, _) =
                BtcLightClientState::get_period_target_pda(current_period, ctx.program_id);
            let period_target_account = &ctx.remaining_accounts[headers.len() + 1];
            require!(
                period_target_account.key() == period_target_pda,
                BtcLightClientError::InvalidPdaAccount
            );
            let period_target_entry =
                PeriodTargetEntry::try_deserialize(&mut &period_target_account.data.borrow()[..])?;
            require!(
                new_target == period_target_entry.target,
                BtcLightClientError::InvalidDifficultyAdjustment
            );
        }
    }

    // Get old tip
    let (old_tip_pda, _) = BtcLightClientState::get_block_hash_pda(block_height, ctx.program_id);
    let old_tip_account = &ctx.remaining_accounts[1]; // first block hash account
    require!(
        old_tip_account.key() == old_tip_pda,
        BtcLightClientError::InvalidPdaAccount
    );
    let old_tip_entry = BlockHashEntry::try_deserialize(&mut &old_tip_account.data.borrow()[..])?;
    let old_tip = old_tip_entry.hash;

    let new_tip = headers.last().unwrap().block_hash().to_byte_array();

    if num_reorged > 0 {
        emit!(ChainReorg {
            reorg_count: num_reorged,
            old_tip,
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
#[instruction(block_height: u64, headers: Vec<u8>)]
pub struct SubmitBlockHeaders<'info> {
    #[account(mut, seeds = [b"btc_light_client"], bump)]
    pub state: Account<'info, BtcLightClientState>,

    #[account(mut)]
    pub submitter: Signer<'info>,
    pub system_program: Program<'info, System>,
}
