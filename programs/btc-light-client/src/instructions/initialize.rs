use crate::state::BtcLightClientState;
use anchor_lang::prelude::*;

pub fn initialize(
    ctx: Context<Initialize>,
    block_height: u64,
    block_hash: [u8; 32],
    block_time: u32,
    expected_target: [u8; 32],
    is_testnet: bool,
) -> Result<()> {
    let state = &mut ctx.accounts.state;

    state.latest_block_height = block_height;
    state.latest_block_time = block_time;
    state.block_hashes.push((block_height, block_hash));
    state
        .period_targets
        .push((block_height / 2016, expected_target));
    state.is_testnet = is_testnet;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + BtcLightClientState::SPACE)]
    pub state: Account<'info, BtcLightClientState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
} 