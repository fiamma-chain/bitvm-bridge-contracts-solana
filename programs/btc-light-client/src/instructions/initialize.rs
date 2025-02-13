use crate::state::*;
use anchor_lang::prelude::*;

pub fn initialize(
    ctx: Context<Initialize>,
    block_height: u64,
    block_hash: [u8; 32],
    block_time: u32,
    expected_target: [u8; 32],
    is_testnet: bool,
    min_confirmations: u64,
) -> Result<()> {
    let state = &mut ctx.accounts.state;

    state.latest_block_height = block_height;
    state.latest_block_hash = block_hash;
    state.latest_block_time = block_time;
    state.latest_period_target = expected_target;
    state.is_testnet = is_testnet;
    state.min_confirmations = min_confirmations;
    state.owner = ctx.accounts.payer.key();

    let block_hash_entry = &mut ctx.accounts.block_hash_entry;
    block_hash_entry.height = block_height;
    block_hash_entry.hash = block_hash;

    Ok(())
}

#[derive(Accounts)]
#[instruction(block_height: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = BtcLightClientState::SPACE,
        seeds = [b"btc_light_client"],
        bump
    )]
    pub state: Account<'info, BtcLightClientState>,

    #[account(
        init,
        payer = payer,
        space = 8 + 8 + 32,
        seeds = [b"block_hash_entry".as_ref(), block_height.to_le_bytes().as_ref()],
        bump
    )]
    pub block_hash_entry: Account<'info, BlockHashEntry>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
