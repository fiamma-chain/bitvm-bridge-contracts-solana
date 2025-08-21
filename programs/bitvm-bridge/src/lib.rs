#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
use instructions::*;
use state::{LPClaimInfo, LPRegister, LPStatus};

declare_id!("APq3X5pBj5txLJmzmxL5yrDJXEbikgDMCVcQPoYtZCs");

#[program]
pub mod bitvm_bridge {

    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        token_metadata: TokenMetadata,
        bridge_params: BridgeParams,
    ) -> Result<()> {
        initialize::initialize(ctx, token_metadata, bridge_params)
    }

    pub fn mint(ctx: Context<MintToken>, tx_id: [u8; 32], amount: u64) -> Result<()> {
        mint::mint_token(ctx, tx_id, amount)
    }

    pub fn burn(
        ctx: Context<BurnToken>,
        amount: u64,
        btc_addr: String,
        fee_rate: u32,
        operator_id: u64,
    ) -> Result<()> {
        burn::burn_token(ctx, amount, btc_addr, fee_rate, operator_id)
    }

    pub fn transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        transfer::transfer_token(ctx, amount)
    }

    pub fn update_bridge_params(
        ctx: Context<UpdateBridgeParams>,
        max_btc_per_mint: u64,
        min_btc_per_mint: u64,
        max_btc_per_burn: u64,
        min_btc_per_burn: u64,
    ) -> Result<()> {
        admin::update_bridge_params(
            ctx,
            max_btc_per_mint,
            min_btc_per_mint,
            max_btc_per_burn,
            min_btc_per_burn,
        )
    }

    pub fn toggle_skip_tx_verification(ctx: Context<ToggleSkipTxVerification>) -> Result<()> {
        admin::toggle_skip_tx_verification(ctx)
    }

    pub fn pause_burn(ctx: Context<ToggleBurnPause>) -> Result<()> {
        admin::pause_burn(ctx)
    }

    pub fn unpause_burn(ctx: Context<ToggleBurnPause>) -> Result<()> {
        admin::unpause_burn(ctx)
    }

    pub fn set_max_fee_rate(ctx: Context<SetMaxFeeRate>, max_fee_rate: u64) -> Result<()> {
        admin::set_max_fee_rate(ctx, max_fee_rate)
    }

    pub fn set_lp_withdraw_timeout(ctx: Context<SetLPWithdrawTimeout>, timeout: u64) -> Result<()> {
        admin::set_lp_withdraw_timeout(ctx, timeout)
    }

    // LP Management Functions
    pub fn register_lp(ctx: Context<RegisterLP>, lp_register: LPRegister) -> Result<()> {
        lp::register_lp(ctx, lp_register)
    }

    pub fn update_lp_status(
        ctx: Context<UpdateLPStatus>,
        lp_id: u64,
        new_status: LPStatus,
    ) -> Result<()> {
        lp::update_lp_status(ctx, lp_id, new_status)
    }

    pub fn withdraw_by_lp(
        ctx: Context<WithdrawByLP>,
        withdraw_id: u64,
        btc_addr: String,
        receiver_script_hash: [u8; 32],
        receive_min_amount: u64,
        lp_id: u64,
        value: u64,
        fee_rate: u64,
    ) -> Result<()> {
        lp::withdraw_by_lp(
            ctx,
            withdraw_id,
            btc_addr,
            receiver_script_hash,
            receive_min_amount,
            lp_id,
            value,
            fee_rate,
        )
    }

    pub fn claim_lp_withdraw(
        ctx: Context<ClaimLPWithdraw>,
        withdraw_id: u64,
        lp_claim_info: LPClaimInfo,
    ) -> Result<()> {
        lp::claim_lp_withdraw(ctx, withdraw_id, lp_claim_info)
    }

    pub fn refund_lp_withdraw(ctx: Context<RefundLPWithdraw>, withdraw_id: u64) -> Result<()> {
        lp::refund_lp_withdraw(ctx, withdraw_id)
    }
}
