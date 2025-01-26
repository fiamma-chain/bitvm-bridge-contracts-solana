use crate::errors::BitvmBridgeError;
use crate::events::MintEvent;
use crate::state::BridgeState;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = mint_authority,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    // recipient is the owner of the associated token account
    // This is a custom account type that is used to store the token account data
    pub associated_token_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == mint_authority.key() @ BitvmBridgeError::UnauthorizedMinter
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    let bridge_state = &ctx.accounts.bridge_state;

    require!(
        amount >= bridge_state.min_btc_per_mint && amount <= bridge_state.max_btc_per_mint,
        BitvmBridgeError::InvalidPeginAmount
    );

    msg!("Mint: {}", &ctx.accounts.mint_account.key());
    msg!(
        "Token Address: {}",
        &ctx.accounts.associated_token_account.key()
    );

    // Invoke the mint_to instruction on the token program
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        ),
        amount,
    )?;

    emit!(MintEvent {
        to: ctx.accounts.recipient.key(),
        value: amount,
    });

    msg!("Token minted successfully.");

    Ok(())
}
