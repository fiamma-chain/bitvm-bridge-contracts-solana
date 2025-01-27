use crate::{
    errors::BtcLightClientError,
    events::{ChainReorg, NewTip, NewTotalDifficultySinceRetarget},
    state::BtcLightClientState,
    utils::{get_work_in_period, mul_in_place},
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

    let parent_period = (block_height - 1) / 2016;
    let mut old_work = bitcoin::Work::from_be_bytes([0; 32]);

    if new_period > parent_period {
        require!(
            new_period == parent_period + 1,
            BtcLightClientError::InvalidDifficultyTransition
        );

        if new_period == old_period {
            old_work = get_work_in_period(state, old_period, state.latest_block_height)?;
        } else {
            require!(
                old_period == parent_period,
                BtcLightClientError::InvalidDifficultyTransition
            );
        }
    }

    let mut num_reorged = 0;

    for (i, header) in headers.iter().enumerate() {
        let current_height = block_height + i as u64;
        let hash = header.block_hash();
        let new_hash = hash.to_byte_array();
        let old_hash = state.get_block_hash(current_height).unwrap_or([0; 32]);

        if old_hash != [0; 32] && old_hash != new_hash {
            num_reorged += 1;
        }

        state.add_block_hash(current_height, new_hash)?;
        let expected_prev_hash = if i == 0 {
            BlockHash::from_byte_array(state.get_block_hash(block_height - 1)?)
        } else {
            headers[i - 1].block_hash()
        };

        require!(
            header.prev_blockhash == expected_prev_hash,
            BtcLightClientError::InvalidPrevHash
        );

        let target = header.target();
        require!(
            target.is_met_by(hash),
            BtcLightClientError::InvalidProofOfWork
        );

        let current_period = current_height / 2016;
        let new_target = header.target().to_be_bytes();

        if current_height % 2016 == 0 {
            let last_target = state.get_period_target(current_period - 1)?;

            if !state.is_testnet {
                let mut t = last_target;
                mul_in_place(&mut t, 4);
                require!(
                    new_target < t,
                    BtcLightClientError::InvalidDifficultyAdjustment
                );
            }

            state.period_targets.push((current_period, new_target));
        } else if !state.is_testnet {
            let period_target = state.get_period_target(current_period)?;
            require!(
                new_target == period_target,
                BtcLightClientError::InvalidDifficultyAdjustment
            );
        }
    }

    let old_tip = state.get_block_hash(block_height)?;
    let new_tip = headers.last().unwrap().block_hash().to_byte_array();

    if num_reorged > 0 {
        emit!(ChainReorg {
            reorg_count: num_reorged,
            old_tip,
            new_tip,
        });
    }

    if new_period > parent_period {
        let new_work = get_work_in_period(state, new_period, new_height)?;
        require!(
            new_work.gt(&old_work),
            BtcLightClientError::InsufficientChainWork
        );
        // delete all block hashes above the new height
        // (in case we just accepted a shorter, heavier chain.)
        let (_, max_height) = state.get_block_range();
        if let Some(max_height) = max_height {
            if max_height > new_height {
                state
                    .block_hashes
                    .retain(|(height, _)| *height <= new_height);
            }
        }
        emit!(NewTotalDifficultySinceRetarget {
            new_height: new_height,
            new_work: new_work.to_be_bytes(),
        });
    } else {
        assert!(new_period == old_period);
        assert!(new_period == parent_period);
        require!(
            new_height > state.latest_block_height,
            BtcLightClientError::InsufficientChainWork
        );
    }

    state.latest_block_height = new_height;
    let last_block_time = headers.last().unwrap().time;
    state.latest_block_time = last_block_time;

    emit!(NewTip {
        block_height: new_height,
        block_time: last_block_time,
        block_hash: new_tip,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SubmitBlockHeaders<'info> {
    #[account(mut, seeds = [b"btc_light_client"], bump)]
    pub state: Account<'info, BtcLightClientState>,
    pub submitter: Signer<'info>,
}
