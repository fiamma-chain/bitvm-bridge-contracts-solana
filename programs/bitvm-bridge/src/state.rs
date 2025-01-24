use anchor_lang::prelude::*;

#[account]
pub struct BridgeState {
    pub owner: Pubkey,
    pub max_btc_per_mint: u64,
    pub min_btc_per_mint: u64,
    pub max_btc_per_burn: u64,
    pub min_btc_per_burn: u64,
    pub burn_paused: bool,
}
