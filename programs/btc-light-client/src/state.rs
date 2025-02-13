use anchor_lang::prelude::*;

#[account]
pub struct BtcLightClientState {
    /// Latest verified block hash (stored in little-endian)
    pub latest_block_hash: [u8; 32],
    /// Height of the latest verified block
    pub latest_block_height: u64,
    /// Timestamp of the latest verified block
    pub latest_block_time: u32,
    /// Current difficulty target for the chain
    pub latest_period_target: [u8; 32],
    /// Whether this client is for testnet or mainnet
    pub is_testnet: bool,
    /// Required number of confirmations for tx verification
    pub min_confirmations: u64,
    /// Owner of the light client who can update settings
    pub owner: Pubkey,
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
        8 +  // min_confirmations
        32; // owner
}

#[account]
pub struct TxVerifiedState {
    pub is_verified: bool,
}

impl TxVerifiedState {
    pub const SPACE: usize = 8 + 1; // discriminator + is_verified
}
