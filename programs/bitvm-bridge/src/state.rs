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
    pub skip_tx_verification: bool,
}

#[account]
pub struct TxMintedState {
    pub is_minted: bool,
}

impl TxMintedState {
    pub const SPACE: usize = 8 + 1; // discriminator + is_minted
}
