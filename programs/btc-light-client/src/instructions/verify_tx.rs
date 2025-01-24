use crate::{
    errors::BtcLightClientError,
    events::TransactionVerified,
    state::BtcLightClientState,
    utils::{verify_merkle_proof, verify_output_script},
};
use anchor_lang::prelude::*;
use bitcoin::{block::Header as BlockHeader, consensus::encode::deserialize, hashes::Hash};

pub fn verify_transaction(
    ctx: Context<VerifyTransaction>,
    block_height: u64,
    tx_proof: BtcTxProof,
) -> Result<()> {
    let state = &ctx.accounts.state;

    require!(
        state.latest_block_height >= block_height + state.min_confirmations,
        BtcLightClientError::InsufficientConfirmations
    );

    let block_hash = state.get_block_hash(block_height)?;

    let header: BlockHeader = deserialize(&tx_proof.block_header)
        .map_err(|_| BtcLightClientError::InvalidHeaderFormat)?;
    require!(
        header.block_hash().to_byte_array() == block_hash,
        BtcLightClientError::BlockHashMismatch
    );

    let tx_hash = bitcoin::Txid::from_byte_array(tx_proof.tx_id);
    require!(
        verify_merkle_proof(
            tx_hash,
            header.merkle_root,
            tx_proof.tx_index,
            &tx_proof.merkle_proof
        ),
        BtcLightClientError::InvalidMerkleProof
    );

    let tx: bitcoin::Transaction =
        deserialize(&tx_proof.raw_tx).map_err(|_| BtcLightClientError::InvalidTransactionFormat)?;

    require!(
        tx.txid().to_byte_array() == tx_proof.tx_id,
        BtcLightClientError::TransactionIdMismatch
    );

    let output = tx
        .output
        .get(tx_proof.output_index as usize)
        .ok_or(BtcLightClientError::InvalidOutputIndex)?;

    require!(
        output.value >= bitcoin::Amount::from_sat(tx_proof.expected_amount),
        BtcLightClientError::InsufficientAmount
    );

    require!(
        verify_output_script(&output.script_pubkey, &tx_proof.expected_script_hash),
        BtcLightClientError::InvalidOutputScript
    );

    emit!(TransactionVerified {
        block_height,
        tx_id: tx_proof.tx_id,
        amount: output.value.to_sat(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct VerifyTransaction<'info> {
    pub state: Account<'info, BtcLightClientState>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BtcTxProof {
    pub block_header: Vec<u8>,
    pub tx_id: [u8; 32],
    pub tx_index: u32,
    pub merkle_proof: Vec<[u8; 32]>,
    pub raw_tx: Vec<u8>,
    pub output_index: u32,
    pub expected_amount: u64,
    pub expected_script_hash: [u8; 32],
}
