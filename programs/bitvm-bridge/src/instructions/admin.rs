use crate::errors::BitvmBridgeError;
use crate::events::{LPWithdrawTimeoutUpdated, OwnershipTransferred};
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

#[derive(Accounts)]
pub struct ToggleBurnPause<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub owner: Signer<'info>,
}

pub fn pause_burn(ctx: Context<ToggleBurnPause>) -> Result<()> {
    require!(
        !ctx.accounts.bridge_state.burn_paused,
        BitvmBridgeError::BurnAlreadyPaused
    );

    ctx.accounts.bridge_state.burn_paused = true;
    Ok(())
}

pub fn unpause_burn(ctx: Context<ToggleBurnPause>) -> Result<()> {
    require!(
        ctx.accounts.bridge_state.burn_paused,
        BitvmBridgeError::BurnNotPaused
    );

    ctx.accounts.bridge_state.burn_paused = false;
    Ok(())
}

#[derive(Accounts)]
pub struct SetMaxFeeRate<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub owner: Signer<'info>,
}

pub fn set_max_fee_rate(ctx: Context<SetMaxFeeRate>, max_fee_rate: u64) -> Result<()> {
    ctx.accounts.bridge_state.max_fee_rate = max_fee_rate;
    Ok(())
}

#[derive(Accounts)]
pub struct SetLPWithdrawTimeout<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub owner: Signer<'info>,
}

pub fn set_lp_withdraw_timeout(ctx: Context<SetLPWithdrawTimeout>, timeout: u64) -> Result<()> {
    ctx.accounts.bridge_state.lp_withdraw_timeout = timeout;

    emit!(LPWithdrawTimeoutUpdated {
        new_timeout: timeout,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == current_owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub current_owner: Signer<'info>,

    /// CHECK: New owner can be any pubkey (including multisig)
    pub new_owner: UncheckedAccount<'info>,
}

pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    let previous_owner = bridge_state.owner;
    let new_owner = ctx.accounts.new_owner.key();

    // Update the owner
    bridge_state.owner = new_owner;

    // Emit ownership transfer event
    emit!(OwnershipTransferred {
        previous_owner,
        new_owner,
    });

    Ok(())
}
