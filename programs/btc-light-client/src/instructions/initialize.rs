use crate::state::*;
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
    let block_hash_entry = &mut ctx.accounts.block_hash;
    let period_target = &mut ctx.accounts.period_target;

    state.latest_block_height = block_height;
    state.latest_block_time = block_time;
    state.is_testnet = is_testnet;
    state.min_confirmations = 1;

    block_hash_entry.height = block_height;
    block_hash_entry.hash = block_hash;

    period_target.period = block_height / 2016;
    period_target.target = expected_target;

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
        space = 8 + 8 + 32,  // discriminator + height + hash
        seeds = [b"block_hash".as_ref(), block_height.to_le_bytes().as_ref()],
        bump
    )]
    pub block_hash: Account<'info, BlockHashEntry>,

    #[account(
        init,
        payer = payer,
        space = 8 + 8 + 32,  // discriminator + period + target
        seeds = [b"period_target".as_ref(), (block_height / 2016).to_le_bytes().as_ref()],
        bump
    )]
    pub period_target: Account<'info, PeriodTargetEntry>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
