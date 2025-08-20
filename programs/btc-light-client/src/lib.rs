#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("6ht3bQk95HutmusCwed2vh14YcVaJgbHmHdS5VkfbEUt");

#[program]
pub mod btc_light_client {
    use super::*;

    // Initialize BTC Light Client
    pub fn initialize(
        ctx: Context<Initialize>,
        block_height: u64,
        block_hash: [u8; 32],
        block_time: u32,
        expected_target: [u8; 32],
        is_testnet: bool,
        min_confirmations: u64,
    ) -> Result<()> {
        instructions::initialize::initialize(
            ctx,
            block_height,
            block_hash,
            block_time,
            expected_target,
            is_testnet,
            min_confirmations,
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

    pub fn update_min_confirmations(
        ctx: Context<UpdateMinConfirmations>,
        min_confirmations: u64,
    ) -> Result<()> {
        instructions::update_min_confirmations(ctx, min_confirmations)
    }
}
