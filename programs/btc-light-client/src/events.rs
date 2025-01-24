use anchor_lang::prelude::*;

#[event]
pub struct NewTip {
    pub block_height: u64,
    pub block_time: u32,
    pub block_hash: [u8; 32],
}

#[event]
pub struct TransactionVerified {
    pub block_height: u64,
    pub tx_id: [u8; 32],
    pub amount: u64,
}

#[event]
pub struct ChainReorg {
    pub reorg_count: u64,
    pub old_tip: [u8; 32],
    pub new_tip: [u8; 32],
}
