use crate::{
    errors::BtcLightClientError,
    events::{ChainReorg, NewTip},
    state::*,
    utils::{get_and_verify_block_hash_account, mul_in_place},
};
use anchor_lang::prelude::*;
use bitcoin::{block::Header as BlockHeader, consensus::deserialize, hashes::Hash};

pub fn submit_block_headers(
    ctx: Context<SubmitBlockHeaders>,
    block_height: u64,
    headers: Vec<u8>,
) -> Result<()> {
    let headers: Vec<BlockHeader> = headers
        .chunks(80)
        .map(|chunk| deserialize(chunk))
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| BtcLightClientError::InvalidHeaderLength)?;

    let state = &mut ctx.accounts.state;

    require!(!headers.is_empty(), BtcLightClientError::NoHeaders);

    require!(
        ctx.remaining_accounts.len() >= headers.len(),
        BtcLightClientError::InvalidAccountNumber
    );

    let old_period = state.latest_block_height / 2016;
    let new_height = block_height + headers.len() as u64 - 1;
    let new_period = new_height / 2016;

    require!(
        new_period >= old_period,
        BtcLightClientError::OldDifficultyPeriod
    );

    let mut num_reorged = 0;

    let mut prev_hash = state.latest_block_hash;

    for (i, header) in headers.iter().enumerate() {
        let current_height = block_height + i as u64;
        let hash = header.block_hash();
        let hash_bytes = header.block_hash().to_byte_array();

        // Get or create block hash account
        let block_hash_entry = get_and_verify_block_hash_account(
            &ctx.remaining_accounts[i],
            current_height,
            ctx.program_id,
        )?;

        if block_hash_entry.hash != [0; 32] && block_hash_entry.hash != hash_bytes {
            num_reorged += 1;
        }

        // Update block hash
        let block_hash_entry = BlockHashEntry {
            height: current_height,
            hash: hash_bytes,
        };
        block_hash_entry.serialize(&mut &mut ctx.remaining_accounts[i].data.borrow_mut()[..])?;

        // Verify previous block hash
        require!(
            prev_hash != [0; 32],
            BtcLightClientError::ParentBlockNotYetSubmitted
        );

        if num_reorged == 0 {
            require!(
                header.prev_blockhash.to_byte_array() == prev_hash,
                BtcLightClientError::InvalidPrevHash
            );
        }
        prev_hash = hash_bytes;

        // Verify PoW and difficulty
        let target = header.target();
        require!(
            target.is_met_by(hash),
            BtcLightClientError::InvalidProofOfWork
        );

        let new_target = target.to_be_bytes();
        if current_height % 2016 == 0 {
            if !state.is_testnet {
                let mut prev_target = state.latest_period_target;
                mul_in_place(&mut prev_target, 4);
                require!(
                    new_target < prev_target,
                    BtcLightClientError::InvalidDifficultyAdjustment
                );
            }
            state.latest_period_target = new_target;
        } else if !state.is_testnet {
            require!(
                new_target == state.latest_period_target,
                BtcLightClientError::InvalidDifficultyAdjustment
            );
        }
    }
    let last_header = headers.last().unwrap();
    let new_tip = last_header.block_hash().to_byte_array();
    if num_reorged > 0 {
        emit!(ChainReorg {
            reorg_count: num_reorged,
            old_tip: state.latest_block_hash,
            new_tip,
        });
    }

    state.latest_block_height = new_height;
    state.latest_block_hash = new_tip;
    state.latest_block_time = last_header.time;

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
