use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::errors::BitvmBridgeError;
use crate::events::BurnEvent;
use crate::state::BridgeState;
#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = authority,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"bridge_state"],
        bump,
        constraint = !bridge_state.burn_paused @ BitvmBridgeError::BurnPaused
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub token_program: Program<'info, Token>,
}

pub fn burn_token(
    ctx: Context<BurnToken>,
    amount: u64,
    btc_addr: String,
    operator_id: u64,
) -> Result<()> {
    let bridge_state = &ctx.accounts.bridge_state;

    require!(
        amount >= bridge_state.min_btc_per_burn && amount <= bridge_state.max_btc_per_burn,
        BitvmBridgeError::InvalidPegoutAmount
    );
    let mint = &ctx.accounts.mint_account;
    let adjusted_amount = amount * 10u64.pow(mint.decimals as u32);

    let cpi_accounts = Burn {
        mint: ctx.accounts.mint_account.to_account_info(),
        from: ctx.accounts.associated_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    token::burn(cpi_ctx, adjusted_amount)?;

    emit!(BurnEvent {
        from: ctx.accounts.associated_token_account.key(),
        btc_addr: btc_addr,
        value: adjusted_amount,
        operator_id: operator_id,
    });

    Ok(())
}
