use anchor_lang::prelude::*;

#[account]
pub struct BridgeState {
    pub owner: Pubkey,
    pub btc_light_client: Pubkey,
    pub mint_account: Pubkey,
    pub max_btc_per_mint: u64,
    pub min_btc_per_mint: u64,
    pub max_btc_per_burn: u64,
    pub min_btc_per_burn: u64,
    pub burn_paused: bool,
}

#[account]
pub struct MintedTx {
    pub tx_id: [u8; 32],
}

impl MintedTx {
    pub const SPACE: usize = 8 + 32; // discriminator + tx_id

    pub fn get_pda_address(tx_id: &[u8; 32], program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"minted_tx".as_ref(), tx_id.as_ref()], program_id)
    }
}
