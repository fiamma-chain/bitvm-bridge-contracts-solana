use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::errors::BitvmBridgeError;
use crate::events::{
    ClaimLPWithdraw as ClaimLPWithdrawEvent, LPRegistered, LPStatusUpdated,
    RefundLPWithdraw as RefundLPWithdrawEvent, WithdrawByLP as WithdrawByLPEvent,
};
use crate::state::{BridgeState, LPRegister, LPState, LPStatus, LPWithdrawState};

// Register LP instruction
#[derive(Accounts)]
#[instruction(lp_register: LPRegister)]
pub struct RegisterLP<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(
        init,
        payer = owner,
        space = LPState::space(&lp_register.bitcoin_addr),
        seeds = [b"lp_state".as_ref(), lp_register.lp_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub lp_state: Account<'info, LPState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn register_lp(ctx: Context<RegisterLP>, lp_register: LPRegister) -> Result<()> {
    // Validate LP register data
    require!(
        lp_register.lp_addr != Pubkey::default(),
        BitvmBridgeError::InvalidLPAddress
    );
    require!(
        !lp_register.bitcoin_addr.is_empty(),
        BitvmBridgeError::InvalidBitcoinAddress
    );

    let lp_state = &mut ctx.accounts.lp_state;

    // Initialize LP state
    lp_state.id = lp_register.lp_id;
    lp_state.bitcoin_addr = lp_register.bitcoin_addr.clone();
    lp_state.lp_addr = lp_register.lp_addr;
    lp_state.fee = lp_register.fee;
    lp_state.status = LPStatus::Active;

    emit!(LPRegistered {
        id: lp_register.lp_id,
        lp_addr: lp_register.lp_addr,
        bitcoin_addr: lp_register.bitcoin_addr,
        fee: lp_register.fee,
    });

    Ok(())
}

// Update LP status instruction
#[derive(Accounts)]
#[instruction(lp_id: u64)]
pub struct UpdateLPStatus<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(
        mut,
        seeds = [b"lp_state".as_ref(), lp_id.to_le_bytes().as_ref()],
        bump,
        constraint = lp_state.status != LPStatus::Unregistered @ BitvmBridgeError::InvalidLPID
    )]
    pub lp_state: Account<'info, LPState>,

    pub owner: Signer<'info>,
}

pub fn update_lp_status(
    ctx: Context<UpdateLPStatus>,
    _lp_id: u64,
    new_status: LPStatus,
) -> Result<()> {
    let lp_state = &mut ctx.accounts.lp_state;
    lp_state.status = new_status.clone();

    emit!(LPStatusUpdated {
        id: lp_state.id,
        new_status,
    });

    Ok(())
}

// Withdraw by LP instruction
#[derive(Accounts)]
#[instruction(withdraw_id: u64, lp_id: u64, btc_addr: String)]
pub struct WithdrawByLP<'info> {
    #[account(
        seeds = [b"bridge_state"],
        bump,
        constraint = !bridge_state.burn_paused @ BitvmBridgeError::BurnPaused
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(
        seeds = [b"lp_state".as_ref(), lp_id.to_le_bytes().as_ref()],
        bump,
        constraint = lp_state.status == LPStatus::Active @ BitvmBridgeError::LPNotActive
    )]
    pub lp_state: Account<'info, LPState>,

    #[account(
        init,
        payer = user,
        space = LPWithdrawState::space(&btc_addr),
        seeds = [b"lp_withdraw".as_ref(), withdraw_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub lp_withdraw_state: Account<'info, LPWithdrawState>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == bridge_state.mint_account,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = contract_token_account.owner == bridge_state.key(),
        constraint = contract_token_account.mint == bridge_state.mint_account,
    )]
    pub contract_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_by_lp(
    ctx: Context<WithdrawByLP>,
    withdraw_id: u64,
    btc_addr: String,
    receiver_script_hash: [u8; 32],
    receive_min_amount: u64,
    lp_id: u64,
    amount: u64,
    fee_rate: u64,
) -> Result<()> {
    let bridge_state = &ctx.accounts.bridge_state;

    // Validate burn restrictions
    require!(
        amount >= bridge_state.min_btc_per_burn && amount <= bridge_state.max_btc_per_burn,
        BitvmBridgeError::InvalidPegoutAmount
    );
    require!(
        fee_rate > 0 && fee_rate <= bridge_state.max_fee_rate,
        BitvmBridgeError::InvalidFeeRate
    );

    // Initialize LP withdraw state
    let lp_withdraw_state = &mut ctx.accounts.lp_withdraw_state;
    lp_withdraw_state.id = withdraw_id;
    lp_withdraw_state.withdraw_amount = amount;
    lp_withdraw_state.receiver_addr = btc_addr.clone();
    lp_withdraw_state.receiver_script_hash = receiver_script_hash;
    lp_withdraw_state.receive_min_amount = receive_min_amount;
    lp_withdraw_state.fee_rate = fee_rate;
    lp_withdraw_state.timestamp = Clock::get()?.unix_timestamp;
    lp_withdraw_state.lp_id = lp_id;
    lp_withdraw_state.from_address = ctx.accounts.user.key();

    // Transfer tokens from user to contract
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.contract_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, amount)?;

    emit!(WithdrawByLPEvent {
        from_address: ctx.accounts.user.key(),
        withdraw_id,
        btc_addr,
        fee_rate,
        amount,
        lp_id,
        receive_min_amount,
    });

    Ok(())
}

// Claim LP withdraw instruction
#[derive(Accounts)]
#[instruction(withdraw_id: u64, btc_tx_id: [u8; 32])]
pub struct ClaimLPWithdraw<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(
        seeds = [b"lp_state", lp_withdraw_state.lp_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub lp_state: Account<'info, LPState>,

    #[account(
        mut,
        seeds = [b"lp_withdraw".as_ref(), withdraw_id.to_le_bytes().as_ref()],
        bump,
        close = owner
    )]
    pub lp_withdraw_state: Account<'info, LPWithdrawState>,

    #[account(
        mut,
        constraint = contract_token_account.owner == bridge_state.key(),
        constraint = contract_token_account.mint == bridge_state.mint_account,
    )]
    pub contract_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = lp_token_account.owner == lp_state.lp_addr,
        constraint = lp_token_account.mint == bridge_state.mint_account,
    )]
    pub lp_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,

    // Bitcoin transaction verification accounts
    #[account(
        mut,
        seeds = [b"tx_verified_state", btc_tx_id.as_ref()],
        seeds::program = btc_light_client::ID,
        bump,
    )]
    pub tx_verified_state: Option<Account<'info, btc_light_client::state::TxVerifiedState>>,
}

pub fn claim_lp_withdraw(
    ctx: Context<ClaimLPWithdraw>,
    _withdraw_id: u64,
    _btc_tx_id: [u8; 32],
    amount_sats: u64,
) -> Result<()> {
    let bridge_state = &ctx.accounts.bridge_state;
    let lp_withdraw_state = &ctx.accounts.lp_withdraw_state;
    let lp_state = &ctx.accounts.lp_state;
    let tx_verified_state = &ctx.accounts.tx_verified_state;

    // Validate claim info
    require!(
        amount_sats > lp_withdraw_state.receive_min_amount,
        BitvmBridgeError::InvalidLPWithdrawAmount
    );

    require!(
        amount_sats <= lp_withdraw_state.withdraw_amount,
        BitvmBridgeError::InvalidLPWithdrawAmount
    );

    // Verify Bitcoin transaction (similar to mint function)
    require!(
        bridge_state.skip_tx_verification
            || (tx_verified_state.is_some() && tx_verified_state.as_ref().unwrap().is_verified),
        BitvmBridgeError::TxNotVerified
    );

    // Transfer tokens from contract to LP
    let cpi_accounts = Transfer {
        from: ctx.accounts.contract_token_account.to_account_info(),
        to: ctx.accounts.lp_token_account.to_account_info(),
        authority: ctx.accounts.bridge_state.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, lp_withdraw_state.withdraw_amount)?;

    emit!(ClaimLPWithdrawEvent {
        withdraw_id: lp_withdraw_state.id,
        lp_id: lp_state.id,
        lp_addr: lp_state.lp_addr,
        withdraw_amount: lp_withdraw_state.withdraw_amount,
    });

    Ok(())
}

// Refund LP withdraw instruction
#[derive(Accounts)]
#[instruction(withdraw_id: u64)]
pub struct RefundLPWithdraw<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump,
        constraint = bridge_state.owner == owner.key() @ BitvmBridgeError::UnauthorizedOwner
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(
        mut,
        seeds = [b"lp_withdraw".as_ref(), withdraw_id.to_le_bytes().as_ref()],
        bump,
        close = owner
    )]
    pub lp_withdraw_state: Account<'info, LPWithdrawState>,

    #[account(
        mut,
        constraint = contract_token_account.owner == bridge_state.key(),
        constraint = contract_token_account.mint == bridge_state.mint_account,
    )]
    pub contract_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = receiver_token_account.owner == receiver.key(),
        constraint = receiver_token_account.mint == bridge_state.mint_account,
    )]
    pub receiver_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is the receiver of the refund, validated by token account ownership
    pub receiver: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn refund_lp_withdraw(ctx: Context<RefundLPWithdraw>, _withdraw_id: u64) -> Result<()> {
    let lp_withdraw_state = &ctx.accounts.lp_withdraw_state;
    let bridge_state = &ctx.accounts.bridge_state;

    // Check timeout
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        lp_withdraw_state.timestamp + bridge_state.lp_withdraw_timeout as i64 <= current_time,
        BitvmBridgeError::InvalidLPWithdrawTimeout
    );

    // Transfer tokens from contract back to receiver
    let cpi_accounts = Transfer {
        from: ctx.accounts.contract_token_account.to_account_info(),
        to: ctx.accounts.receiver_token_account.to_account_info(),
        authority: ctx.accounts.bridge_state.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, lp_withdraw_state.withdraw_amount)?;

    emit!(RefundLPWithdrawEvent {
        withdraw_id: lp_withdraw_state.id,
        receiver: ctx.accounts.receiver.key(),
        withdraw_amount: lp_withdraw_state.withdraw_amount,
    });

    Ok(())
}
