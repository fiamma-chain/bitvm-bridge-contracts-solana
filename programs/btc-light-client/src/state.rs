use crate::errors::BtcLightClientError;
use anchor_lang::prelude::*;

#[account]
pub struct BtcLightClientState {
    pub latest_block_height: u64,
    pub latest_block_time: u32,
    pub block_hashes: Vec<(u64, [u8; 32])>,
    pub period_targets: Vec<(u64, [u8; 32])>,
    pub is_testnet: bool,
    pub min_confirmations: u64,
}

impl BtcLightClientState {
    pub const MAX_BLOCK_HASHES: usize = 5000;
    // ... existing code ...
    pub const SPACE: usize = 8 +  // account discriminator
        8 +  // latest_block_height
        4 +  // latest_block_time
        4 +  // vec length for block_hashes
        (8 + 32) * Self::MAX_BLOCK_HASHES +  // block_hashes content
        4 +  // vec length for period_targets
        (8 + 32) * 10 +  // period_targets content
        1 +  // is_testnet
        8; // min_confirmations
    pub fn add_block_hash(&mut self, height: u64, hash: [u8; 32]) -> Result<()> {
        // if the block hash already exists, update it
        if let Some(item) = self.block_hashes.iter_mut().find(|(h, _)| *h == height) {
            item.1 = hash;
            return Ok(());
        }

        // if the capacity is reached, remove the oldest block
        if self.block_hashes.len() >= Self::MAX_BLOCK_HASHES {
            self.block_hashes.remove(0); // remove the oldest (lowest height)
        }

        // find the correct insert position (keep sorted by height)
        let insert_pos = self
            .block_hashes
            .binary_search_by_key(&height, |(h, _)| *h)
            .unwrap_or_else(|pos| pos);

        // insert the new hash at the correct position
        self.block_hashes.insert(insert_pos, (height, hash));

        Ok(())
    }

    // get stored block range
    pub fn get_block_range(&self) -> (Option<u64>, Option<u64>) {
        if self.block_hashes.is_empty() {
            return (None, None);
        }

        let min_height = self.block_hashes.first().map(|(h, _)| *h);
        let max_height = self.block_hashes.last().map(|(h, _)| *h);

        (min_height, max_height)
    }

    // get block hash by height
    pub fn get_block_hash(&self, height: u64) -> Result<[u8; 32]> {
        match self.block_hashes.binary_search_by_key(&height, |(h, _)| *h) {
            Ok(idx) => Ok(self.block_hashes[idx].1),
            Err(_) => Err(BtcLightClientError::BlockNotFound.into()),
        }
    }

    pub fn get_period_target(&self, period: u64) -> Result<[u8; 32]> {
        self.period_targets
            .iter()
            .find(|(p, _)| *p == period)
            .map(|(_, target)| *target)
            .ok_or(BtcLightClientError::PeriodTargetNotFound.into())
    }
}
