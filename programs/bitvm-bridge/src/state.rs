use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum LPStatus {
    Unregistered,
    Active,
    Suspended,
    Terminated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct LPRegister {
    pub lp_id: u64,
    pub bitcoin_addr: String,
    pub lp_addr: Pubkey,
    pub fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct LPInfo {
    pub id: u64,
    pub bitcoin_addr: String,
    pub lp_addr: Pubkey,
    pub fee: u64,
    pub status: LPStatus,
}

#[account]
pub struct BridgeState {
    pub owner: Pubkey,
    pub mint_account: Pubkey,
    pub max_btc_per_mint: u64,
    pub min_btc_per_mint: u64,
    pub max_btc_per_burn: u64,
    pub min_btc_per_burn: u64,
    pub skip_tx_verification: bool,
    pub burn_paused: bool,
    pub max_fee_rate: u64,
    pub lp_withdraw_timeout: u64,
}

#[account]
pub struct TxMintedState {
    pub is_minted: bool,
}

impl TxMintedState {
    pub const SPACE: usize = 8 + 1; // discriminator + is_minted
}

#[account]
pub struct LPState {
    pub id: u64,
    pub bitcoin_addr: String,
    pub lp_addr: Pubkey,
    pub fee: u64,
    pub status: LPStatus,
}

impl LPState {
    pub fn space(bitcoin_addr: &str) -> usize {
        8 + // discriminator
        8 + // id
        (4 + bitcoin_addr.len()) + // bitcoin_addr (actual length)
        32 + // lp_addr
        8 + // fee
        1 // status (enum)
    }
}

#[account]
pub struct LPWithdrawState {
    pub id: u64,
    pub withdraw_amount: u64,
    pub receiver_addr: String,
    pub receiver_script_hash: [u8; 32],
    pub receive_min_amount: u64,
    pub fee_rate: u64,
    pub timestamp: i64,
    pub lp_id: u64,
    pub from_address: Pubkey,
}

impl LPWithdrawState {
    pub fn space(receiver_addr: &str) -> usize {
        8 + // discriminator
        8 + // id
        8 + // withdraw_amount
        (4 + receiver_addr.len()) + // receiver_addr (actual length)
        32 + // receiver_script_hash
        8 + // receive_min_amount
        8 + // fee_rate
        8 + // timestamp
        8 + // lp_id
        32 // from_address
    }
}
