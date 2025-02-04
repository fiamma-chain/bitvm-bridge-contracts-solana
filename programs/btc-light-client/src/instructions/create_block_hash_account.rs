use crate::state::BlockHashEntry;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(height: u64)]
pub struct CreateBlockHashAccount<'info> {
    #[account(
        init_if_needed,
        seeds = [b"block_hash".as_ref(), height.to_le_bytes().as_ref()],
        bump,
        payer = payer,
        space = 8 + 8 + 32
    )]
    pub block_hash: Account<'info, BlockHashEntry>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateBlockHashAccount<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub fn create_block_hash_account(
    ctx: Context<CreateBlockHashAccount>,
    height: u64,
    block_hash: [u8; 32],
) -> Result<()> {
    ctx.accounts.block_hash.height = height;
    ctx.accounts.block_hash.hash = block_hash;
    Ok(())
}
