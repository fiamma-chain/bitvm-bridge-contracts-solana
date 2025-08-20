use crate::errors::BitvmBridgeError;
use crate::state::BridgeState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateBridgeParams<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
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
pub struct ToggleSkipTxVerification<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub owner: Signer<'info>,
}

pub fn toggle_skip_tx_verification(ctx: Context<ToggleSkipTxVerification>) -> Result<()> {
    ctx.accounts.bridge_state.skip_tx_verification =
        !ctx.accounts.bridge_state.skip_tx_verification;
    Ok(())
}
