use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
use instructions::*;

declare_id!("Fdj7bMrz8u4ZLyHt3TAnbdqNxtNwQUtqEtgCM84SNWTG");

#[program]
pub mod bitvm_bridge {

    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        token_metadata: TokenMetadata,
        bridge_params: BridgeParams,
    ) -> Result<()> {
        initialize::initialize(ctx, token_metadata, bridge_params)
    }

    pub fn mint(ctx: Context<MintToken>, tx_id: [u8; 32], amount: u64) -> Result<()> {
        mint::mint_token(ctx, tx_id, amount)
    }

    pub fn burn(
        ctx: Context<BurnToken>,
        amount: u64,
        btc_addr: String,
        operator_id: u64,
    ) -> Result<()> {
        burn::burn_token(ctx, amount, btc_addr, operator_id)
    }

    pub fn transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        transfer::transfer_token(ctx, amount)
    }

    pub fn update_bridge_params(
        ctx: Context<UpdateBridgeParams>,
        max_btc_per_mint: u64,
        min_btc_per_mint: u64,
        max_btc_per_burn: u64,
        min_btc_per_burn: u64,
    ) -> Result<()> {
        admin::update_bridge_params(
            ctx,
            max_btc_per_mint,
            min_btc_per_mint,
            max_btc_per_burn,
            min_btc_per_burn,
        )
    }

    pub fn pause_burn(ctx: Context<ToggleBurnPause>) -> Result<()> {
        admin::pause_burn(ctx)
    }

    pub fn unpause_burn(ctx: Context<ToggleBurnPause>) -> Result<()> {
        admin::unpause_burn(ctx)
    }
}
