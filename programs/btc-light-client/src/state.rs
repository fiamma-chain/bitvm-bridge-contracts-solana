use crate::errors::BtcLightClientError;
use anchor_lang::prelude::*;

#[account]
pub struct BtcLightClientState {
    pub latest_block_height: u64,
    pub latest_block_time: u32,
    pub block_hashes: Vec<(u64, [u8; 32])>,
    pub period_targets: Vec<(u64, [u8; 32])>,
    pub is_testnet: bool,
}

impl BtcLightClientState {
    pub const SPACE: usize = 8 + 8 + 8 + (8 + 32) * 100 + (8 + 32) * 10 + 1;

    pub fn get_block_hash(&self, height: u64) -> Result<[u8; 32]> {
        self.block_hashes
            .iter()
            .find(|(h, _)| *h == height)
            .map(|(_, hash)| *hash)
            .ok_or(BtcLightClientError::BlockNotFound.into())
    }

    pub fn get_period_target(&self, period: u64) -> Result<[u8; 32]> {
        self.period_targets
            .iter()
            .find(|(p, _)| *p == period)
            .map(|(_, target)| *target)
            .ok_or(BtcLightClientError::PeriodTargetNotFound.into())
    }
}
