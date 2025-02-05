use anchor_lang::prelude::*;

#[error_code]
pub enum BtcLightClientError {
    #[msg("Invalid block header")]
    InvalidHeader,
    #[msg("Invalid account number")]
    InvalidAccountNumber,
    #[msg("No headers provided")]
    NoHeaders,
    #[msg("Old difficulty period")]
    OldDifficultyPeriod,
    #[msg("Invalid proof of work")]
    InvalidProofOfWork,
    #[msg("Invalid previous block hash")]
    InvalidPrevHash,
    #[msg("Parent block not yet submitted")]
    ParentBlockNotYetSubmitted,
    #[msg("Invalid header format")]
    InvalidHeaderFormat,
    #[msg("Block hash mismatch")]
    BlockHashMismatch,
    #[msg("Invalid merkle proof")]
    InvalidMerkleProof,
    #[msg("Invalid transaction format")]
    InvalidTransactionFormat,
    #[msg("Transaction ID mismatch")]
    TransactionIdMismatch,
    #[msg("Invalid output index")]
    InvalidOutputIndex,
    #[msg("Insufficient amount")]
    InsufficientAmount,
    #[msg("Invalid output script")]
    InvalidOutputScript,
    #[msg("Insufficient confirmations")]
    InsufficientConfirmations,
    #[msg("Invalid difficulty adjustment")]
    InvalidDifficultyAdjustment,
    #[msg("Invalid PDA account")]
    InvalidPdaAccount,
    #[msg("Deserialization PDA Account error")]
    DeserializationError,
    #[msg("Empty PDA account")]
    EmptyPdaAccount,
}
