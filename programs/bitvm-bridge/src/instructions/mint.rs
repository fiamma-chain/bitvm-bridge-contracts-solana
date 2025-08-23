use crate::{
    errors::BitvmBridgeError,
    events::MintEvent,
    state::{BridgeState, TxMintedState},
};
use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use btc_light_client::state::TxVerifiedState;
use btc_light_client::ID as BTC_LIGHT_CLIENT_PROGRAM_ID;

#[derive(Accounts)]
#[instruction(tx_id: [u8; 32])]
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
    )]
    pub bridge_state: Account<'info, BridgeState>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    #[account(
        init_if_needed,
        payer = mint_authority,
        space = TxMintedState::SPACE,
        seeds = [b"tx_minted_state".as_ref(), tx_id.as_ref()],
        bump,
    )]
    pub tx_minted_state: Account<'info, TxMintedState>,

    #[account(
        mut,
        seeds = [b"tx_verified_state", tx_id.as_ref()],
        seeds::program = BTC_LIGHT_CLIENT_PROGRAM_ID,
        bump,
    )]
    pub tx_verified_state: Option<Account<'info, TxVerifiedState>>,
}

pub fn mint_token(ctx: Context<MintToken>, _tx_id: [u8; 32], amount: u64) -> Result<()> {
    let bridge_state = &ctx.accounts.bridge_state;
    let tx_verified_state = &ctx.accounts.tx_verified_state;
    let tx_minted_state = &mut ctx.accounts.tx_minted_state;

    // Check if the mint authority is the bridge owner or the mint authority is set
    let mint_info = &ctx.accounts.mint_account;
    require!(
        bridge_state.owner == ctx.accounts.mint_authority.key()
            || mint_info.mint_authority == COption::Some(ctx.accounts.mint_authority.key()),
        BitvmBridgeError::UnauthorizedMinter
    );

    // Verify amount limits
    require!(
        amount >= bridge_state.min_btc_per_mint && amount <= bridge_state.max_btc_per_mint,
        BitvmBridgeError::InvalidPeginAmount
    );

    require!(
        bridge_state.skip_tx_verification
            || (tx_verified_state.is_some() && tx_verified_state.as_ref().unwrap().is_verified),
        BitvmBridgeError::TxNotVerified
    );

    require!(
        !tx_minted_state.is_minted,
        BitvmBridgeError::TxAlreadyMinted
    );

    // Mint tokens
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

    tx_minted_state.is_minted = true;

    emit!(MintEvent {
        to: ctx.accounts.recipient.key(),
        value: amount,
    });

    Ok(())
}
