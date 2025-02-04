use anchor_lang::prelude::*;

#[account]
pub struct BtcLightClientState {
    pub latest_block_height: u64,
    pub latest_block_hash: [u8; 32],
    pub latest_block_time: u32,
    pub latest_peroid_target: [u8; 32],
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
        4 +  // latest_block_time
        1 +  // is_testnet
        8; // min_confirmations
}
