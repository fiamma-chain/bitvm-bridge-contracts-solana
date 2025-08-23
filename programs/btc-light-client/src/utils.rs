use crate::errors::BtcLightClientError;
use crate::state::BlockHashEntry;
use anchor_lang::prelude::*;
use bitcoin::hashes::{sha256d, Hash};

pub fn verify_merkle_proof(
    tx_hash: bitcoin::Txid,
    merkle_root: bitcoin::TxMerkleNode,
    tx_index: u32,
    proof: &[[u8; 32]],
) -> bool {
    let mut current_hash = tx_hash.to_raw_hash();

    if proof.is_empty() && tx_index != 0 {
        return false;
    }

    for (i, next_hash) in proof.iter().enumerate() {
        let mut concat = vec![];
        // extracts the i-th bit of tx idx
        if ((tx_index >> i) & 1) == 1 {
            // If the bit is 1, the transaction is in the right subtree of the current hash
            // Append the next hash and then the current hash to the concatenated hash value
            concat.extend_from_slice(next_hash);
            concat.extend_from_slice(&current_hash[..]);
        } else {
            // If the bit is 0, the transaction is in the left subtree of the current hash
            // Append the current hash and then the next hash to the concatenated hash value
            concat.extend_from_slice(&current_hash[..]);
            concat.extend_from_slice(next_hash);
        }

        current_hash = sha256d::Hash::hash(&concat);
    }

    &current_hash == &merkle_root.to_raw_hash()
}

pub fn verify_output_script(script: &bitcoin::Script, expected_hash: &[u8; 32]) -> bool {
    if script.is_p2wsh() {
        // P2WSH: Extract 32-byte hash directly
        let script_bytes = script.as_bytes();
        &script_bytes[2..34] == expected_hash
    } else if script.is_p2sh() {
        // P2SH: Extract 20-byte hash and pad to 32 bytes (right-padded with zeros)
        let script_bytes = script.as_bytes();
        let mut padded_hash = [0u8; 32];
        padded_hash[..20].copy_from_slice(&script_bytes[2..22]);
        &padded_hash == expected_hash
    } else if script.is_p2tr() {
        // P2TR: Extract 32-byte taproot output directly
        let script_bytes = script.as_bytes();
        &script_bytes[2..34] == expected_hash
    } else {
        false
    }
}

pub fn mul_in_place(arr: &mut [u8; 32], multiplicator: u32) {
    let casted_mul: u64 = multiplicator as u64;
    let mut remainder: u64 = 0;

    for i in 0..32 {
        let pos = 31 - i;
        let val = ((arr[pos] as u64) * casted_mul) + remainder;
        let byte = val & 0xFF;
        remainder = val >> 8;
        arr[pos] = byte as u8;
    }
}

fn get_block_hash_pda(height: u64, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"block_hash_entry", &height.to_le_bytes()], program_id)
}

pub fn get_and_verify_block_hash_account<'info>(
    account_info: &AccountInfo<'info>,
    height: u64,
    program_id: &Pubkey,
) -> Result<BlockHashEntry> {
    let (pda, _) = get_block_hash_pda(height, program_id);

    require!(
        account_info.key() == pda,
        BtcLightClientError::InvalidPdaAccount
    );

    if account_info.data_is_empty() {
        return Err(error!(BtcLightClientError::EmptyPdaAccount));
    }

    BlockHashEntry::try_deserialize(&mut &account_info.data.borrow()[..])
        .map_err(|_| error!(BtcLightClientError::DeserializationError))
}
