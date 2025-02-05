use crate::state::BlockHashEntry;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(height: u64)]
pub struct CreateBlockHashAccount<'info> {
    #[account(
        init_if_needed,
        seeds = [b"block_hash_entry".as_ref(), height.to_le_bytes().as_ref()],
        bump,
        payer = payer,
        space = 8 + 8 + 32
    )]
    pub block_hash_entry: Account<'info, BlockHashEntry>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_block_hash_account(
    ctx: Context<CreateBlockHashAccount>,
    height: u64,
    block_hash: [u8; 32],
) -> Result<()> {
    let block_hash_entry = &mut ctx.accounts.block_hash_entry;
    block_hash_entry.height = height;
    block_hash_entry.hash = block_hash;
    Ok(())
}
