use anchor_lang::prelude::*;

#[error_code]
pub enum BitvmBridgeError {
    #[msg("Unauthorized minter")]
    UnauthorizedMinter,

    #[msg("Unauthorized owner")]
    UnauthorizedOwner,

    #[msg("Invalid pegin amount")]
    InvalidPeginAmount,

    #[msg("Invalid pegout amount")]
    InvalidPegoutAmount,

    #[msg("Tx verification required")]
    TxVerificationRequired,

    #[msg("Tx not verified")]
    TxNotVerified,

    #[msg("Tx already minted")]
    TxAlreadyMinted,

    #[msg("Mismatch btc amount")]
    MismatchBtcAmount,

    #[msg("Burn paused")]
    BurnPaused,

    #[msg("Burn already paused")]
    BurnAlreadyPaused,

    #[msg("Burn not paused")]
    BurnNotPaused,

    #[msg("Invalid LP ID")]
    InvalidLPID,

    #[msg("LP already registered")]
    LPAlreadyRegistered,

    #[msg("Invalid LP address")]
    InvalidLPAddress,

    #[msg("Invalid Bitcoin address")]
    InvalidBitcoinAddress,

    #[msg("Invalid LP withdraw ID")]
    InvalidLPWithdrawID,

    #[msg("Invalid LP withdraw timeout")]
    InvalidLPWithdrawTimeout,

    #[msg("Invalid LP withdraw amount")]
    InvalidLPWithdrawAmount,

    #[msg("Invalid fee rate")]
    InvalidFeeRate,

    #[msg("LP not active")]
    LPNotActive,

    #[msg("Insufficient allowance")]
    InsufficientAllowance,
}
