use anchor_lang::prelude::*;

#[account]
pub struct BtcLightClientState {
    pub latest_block_height: u64,
    pub latest_block_time: u32,
    pub is_testnet: bool,
    pub min_confirmations: u64,
}

#[account]
pub struct BlockHashEntry {
    pub height: u64,
    pub hash: [u8; 32],
}

#[account]
pub struct PeriodTargetEntry {
    pub period: u64,
    pub target: [u8; 32],
}

impl BtcLightClientState {
    pub const SPACE: usize = 8 +  // discriminator
        8 +  // latest_block_height
        4 +  // latest_block_time
        1 +  // is_testnet
        8; // min_confirmations

    pub fn get_block_hash_pda(height: u64, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"block_hash", &height.to_le_bytes()], program_id)
    }

    pub fn get_period_target_pda(period: u64, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"period_target", &period.to_le_bytes()], program_id)
    }
}
