use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized minter")]
    UnauthorizedMinter,

    #[msg("Unauthorized owner")]
    UnauthorizedOwner,

    #[msg("Invalid pegin amount")]
    InvalidPeginAmount,

    #[msg("Invalid pegout amount")]
    InvalidPegoutAmount,

    #[msg("Burn is paused")]
    BurnPaused,
}
