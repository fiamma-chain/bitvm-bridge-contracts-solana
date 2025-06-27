use anchor_lang::prelude::*;

#[event]
pub struct MintEvent {
    pub to: Pubkey,
    pub value: u64,
}

#[event]
pub struct BurnEvent {
    pub from: Pubkey,
    pub btc_addr: String,
    pub fee_rate: u32,
    pub value: u64,
    pub operator_id: u64,
}
