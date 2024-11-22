use anchor_lang::prelude::*;


pub mod events;
pub mod instructions;

use instructions::*;

declare_id!("AK4wVnCvogwctZjY6PYNpStXKanoiJN4yeyVcRRRtxkg");

#[program]
pub mod bitvm_bridge {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>, token_name: String, token_symbol: String, token_uri: String) -> Result<()> {
        initialize::initialize(ctx, token_name, token_symbol, token_uri)
    }

    pub fn mint(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        mint::mint_token(ctx, amount)
    }

    pub fn burn(ctx: Context<BurnToken>, amount: u64, btc_addr: String, operator_id: u64) -> Result<()> {
        burn::burn_token(ctx, amount, btc_addr, operator_id)
    }
}