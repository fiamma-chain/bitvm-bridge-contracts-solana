use anchor_lang::prelude::*;

#[account]
pub struct BtcLightClientState {
    pub latest_block_height: u64,
    pub latest_block_hash: [u8; 32],
    pub latest_block_time: u32,
    // Target for the current period, in big endian format
    pub latest_period_target: [u8; 32],
    pub is_testnet: bool,
    pub min_confirmations: u64,
}

#[account]
pub struct BlockHashEntry {
    pub height: u64,
    pub hash: [u8; 32],
}

impl BtcLightClientState {
    pub const SPACE: usize = 8 +  // discriminator
        8 +  // latest_block_height
        32 +  // latest_block_hash
        4 +  // latest_block_time
        32 +  // latest_period_target
        1 +  // is_testnet
        8; // min_confirmations
}
