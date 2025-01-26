use crate::{
    errors::BitvmBridgeError,
    events::MintEvent,
    state::{BridgeState, MintedTx},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use btc_light_client::{
    cpi::accounts::VerifyTransaction, cpi::verify_transaction, instructions::BtcTxProof,
    program::BtcLightClient, state::BtcLightClientState,
};

#[derive(Accounts)]
#[instruction(tx_id: [u8; 32])]
pub struct MintToken<'info> {
    #[account(
        init_if_needed,
        payer = mint_authority,
        space = MintedTx::SPACE,
        seeds = [b"minted_tx".as_ref(), tx_id.as_ref()],
        bump
    )]
    pub minted_tx: Account<'info, MintedTx>,

    // BTC Light Client accounts
    #[account(mut, seeds = [b"btc_light_client"], bump)]
    pub btc_light_client_state: Account<'info, BtcLightClientState>,
    pub btc_light_client_program: Program<'info, BtcLightClient>,

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

pub fn mint_token(
    ctx: Context<MintToken>,
    amount: u64,
    block_height: u64,
    tx_proof: BtcTxProof,
) -> Result<()> {
    let bridge_state = &ctx.accounts.bridge_state;
    let minted_tx = &mut ctx.accounts.minted_tx;

    // Verify amount limits
    require!(
        amount >= bridge_state.min_btc_per_mint && amount <= bridge_state.max_btc_per_mint,
        BitvmBridgeError::InvalidPeginAmount
    );

    // Verify transaction hasn't been minted before
    require!(
        minted_tx.tx_id != tx_proof.tx_id,
        BitvmBridgeError::TxAlreadyMinted
    );

    // Store the tx id in the minted_tx account
    minted_tx.tx_id = tx_proof.tx_id;

    // Verify amount matches proof
    require!(
        amount == tx_proof.expected_amount,
        BitvmBridgeError::MismatchBtcAmount
    );

    // Verify BTC transaction with light client
    verify_transaction(
        CpiContext::new(
            ctx.accounts.btc_light_client_program.to_account_info(),
            VerifyTransaction {
                state: ctx.accounts.btc_light_client_state.to_account_info(),
            },
        ),
        block_height,
        tx_proof,
    )?;

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

    emit!(MintEvent {
        to: ctx.accounts.recipient.key(),
        value: amount,
    });

    Ok(())
}
