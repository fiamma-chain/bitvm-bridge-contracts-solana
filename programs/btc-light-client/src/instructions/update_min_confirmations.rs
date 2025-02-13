use crate::errors::BtcLightClientError;
use crate::events::MinConfirmationsUpdated;
use crate::state::BtcLightClientState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateMinConfirmations<'info> {
    /// The BTC Light Client state account
    #[account(
        mut,
        seeds = [b"btc_light_client"],
        bump,
        // ensure the caller is the owner
        constraint = state.owner == authority.key() @ BtcLightClientError::UnauthorizedSigner
    )]
    pub state: Account<'info, BtcLightClientState>,

    /// The authority that can update min confirmations (must be owner)
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn update_min_confirmations(
    ctx: Context<UpdateMinConfirmations>,
    min_confirmations: u64,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let old_value = state.min_confirmations;
    state.min_confirmations = min_confirmations;

    // emit event
    emit!(MinConfirmationsUpdated {
        old_value,
        new_value: min_confirmations,
        authority: ctx.accounts.authority.key(),
    });

    Ok(())
}
