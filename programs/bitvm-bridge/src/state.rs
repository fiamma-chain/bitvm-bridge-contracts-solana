use anchor_lang::prelude::*;

#[account]
pub struct BridgeState {
    pub owner: Pubkey,
    pub mint_account: Pubkey,
    pub max_btc_per_mint: u64,
    pub min_btc_per_mint: u64,
    pub max_btc_per_burn: u64,
    pub min_btc_per_burn: u64,
    pub burn_paused: bool,
}

#[account]
pub struct MintedTx {
    pub tx_id: [u8; 32],
}

impl MintedTx {
    pub const SPACE: usize = 8 + 32; // discriminator + tx_id
}
