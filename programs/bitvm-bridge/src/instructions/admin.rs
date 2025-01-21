use crate::error::ErrorCode;
use crate::BridgeState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateBridgeParams<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ ErrorCode::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub owner: Signer<'info>,
}

pub fn update_bridge_params(
    ctx: Context<UpdateBridgeParams>,
    max_btc_per_mint: u64,
    min_btc_per_mint: u64,
    max_btc_per_burn: u64,
    min_btc_per_burn: u64,
) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;

    bridge_state.max_btc_per_mint = max_btc_per_mint;
    bridge_state.min_btc_per_mint = min_btc_per_mint;
    bridge_state.max_btc_per_burn = max_btc_per_burn;
    bridge_state.min_btc_per_burn = min_btc_per_burn;

    Ok(())
}

#[derive(Accounts)]
pub struct ToggleBurnPause<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ ErrorCode::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub owner: Signer<'info>,
}

pub fn pause_burn(ctx: Context<ToggleBurnPause>) -> Result<()> {
    ctx.accounts.bridge_state.burn_paused = true;
    Ok(())
}

pub fn unpause_burn(ctx: Context<ToggleBurnPause>) -> Result<()> {
    ctx.accounts.bridge_state.burn_paused = false;
    Ok(())
}
