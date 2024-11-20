use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::events::BurnEvent;

#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn burn_token(ctx: Context<BurnToken>, amount: u64, btc_addr: String, operator_id: u64) -> Result<()> {
    let mint = &ctx.accounts.mint_account;
    let adjusted_amount = amount * 10u64.pow(mint.decimals as u32);

    let cpi_accounts = Burn {
        mint: ctx.accounts.mint_account.to_account_info(),
        from: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    token::burn(cpi_ctx, adjusted_amount)?;

    emit!(BurnEvent {
        from: ctx.accounts.token_account.key(),
        btc_addr: btc_addr,
        value: adjusted_amount,
        operator_id: operator_id,
    });

    Ok(())
}
