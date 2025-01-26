use {
    crate::state::BridgeState,
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{
            create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
            CreateMetadataAccountsV3, Metadata,
        },
        token::{Mint, Token},
    },
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BridgeParams {
    pub max_btc_per_mint: u64,
    pub min_btc_per_mint: u64,
    pub max_btc_per_burn: u64,
    pub min_btc_per_burn: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        mint::decimals = 8,
        mint::authority = owner.key(),
        mint::freeze_authority = owner.key(),

    )]
    // token address
    pub mint_account: Account<'info, Mint>,

    #[account(
        init,
        payer = owner,
        seeds = [b"bridge_state"],
        bump,
        space = 8 + std::mem::size_of::<BridgeState>(),
    )]
    pub bridge_state: Account<'info, BridgeState>,
    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    token_metadata: TokenMetadata,
    bridge_params: BridgeParams,
    btc_light_client: Pubkey,
) -> Result<()> {
    msg!("Creating metadata account");

    // Cross Program Invocation (CPI)
    // Invoking the create_metadata_account_v3 instruction on the token metadata program
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.owner.to_account_info(),
                update_authority: ctx.accounts.owner.to_account_info(),
                payer: ctx.accounts.owner.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name: token_metadata.name,
            symbol: token_metadata.symbol,
            uri: token_metadata.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, // Is mutable
        true,  // Update authority is signer
        None,  // Collection details
    )?;

    ctx.accounts.bridge_state.owner = ctx.accounts.owner.key();
    ctx.accounts.bridge_state.btc_light_client = btc_light_client;
    ctx.accounts.bridge_state.mint_account = ctx.accounts.mint_account.key();
    ctx.accounts.bridge_state.max_btc_per_mint = bridge_params.max_btc_per_mint;
    ctx.accounts.bridge_state.min_btc_per_mint = bridge_params.min_btc_per_mint;
    ctx.accounts.bridge_state.max_btc_per_burn = bridge_params.max_btc_per_burn;
    ctx.accounts.bridge_state.min_btc_per_burn = bridge_params.min_btc_per_burn;

    msg!("Initialized successfully.");

    Ok(())
}
