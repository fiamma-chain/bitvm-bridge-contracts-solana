use crate::state::LPStatus;
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

#[event]
pub struct LPRegistered {
    pub id: u64,
    pub lp_addr: Pubkey,
    pub bitcoin_addr: String,
    pub fee: u64,
}

#[event]
pub struct LPStatusUpdated {
    pub id: u64,
    pub new_status: LPStatus,
}

#[event]
pub struct WithdrawByLP {
    pub from_address: Pubkey,
    pub withdraw_id: u64,
    pub btc_addr: String,
    pub fee_rate: u64,
    pub amount: u64,
    pub lp_id: u64,
    pub receive_min_amount: u64,
}

#[event]
pub struct ClaimLPWithdraw {
    pub withdraw_id: u64,
    pub lp_id: u64,
    pub lp_addr: Pubkey,
    pub withdraw_amount: u64,
}

#[event]
pub struct RefundLPWithdraw {
    pub withdraw_id: u64,
    pub receiver: Pubkey,
    pub withdraw_amount: u64,
}

#[event]
pub struct LPWithdrawTimeoutUpdated {
    pub new_timeout: u64,
}

#[event]
pub struct OwnershipTransferred {
    pub previous_owner: Pubkey,
    pub new_owner: Pubkey,
}
