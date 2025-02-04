use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod btc_light_client {
    use super::*;

    // Initialize BTC Light Client
    pub fn initialize(
        ctx: Context<Initialize>,
        block_height: u64,
        period: u64,
        block_hash: [u8; 32],
        block_time: u32,
        expected_target: [u8; 32],
        is_testnet: bool,
    ) -> Result<()> {
        instructions::initialize::initialize(
            ctx,
            block_height,
            period,
            block_hash,
            block_time,
            expected_target,
            is_testnet,
        )
    }

    pub fn create_block_hash_account(
        ctx: Context<CreateBlockHashAccount>,
        height: u64,
        block_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_block_hash_account(ctx, height, block_hash)
    }

    // Submit new block headers
    pub fn submit_block_headers(
        ctx: Context<SubmitBlockHeaders>,
        block_height: u64,
        headers: Vec<u8>,
    ) -> Result<()> {
        instructions::submit_block_headers(ctx, block_height, headers)
    }

    // Verify Bitcoin transaction
    pub fn verify_transaction(
        ctx: Context<VerifyTransaction>,
        block_height: u64,
        tx_proof: BtcTxProof,
    ) -> Result<()> {
        instructions::verify_tx::verify_transaction(ctx, block_height, tx_proof)
    }
}
