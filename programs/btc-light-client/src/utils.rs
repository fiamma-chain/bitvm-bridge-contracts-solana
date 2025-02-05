use crate::errors::BtcLightClientError;
use crate::state::BlockHashEntry;
use anchor_lang::prelude::*;
use bitcoin::hashes::Hash;

pub fn verify_merkle_proof(
    tx_hash: bitcoin::Txid,
    merkle_root: bitcoin::TxMerkleNode,
    tx_index: u32,
    proof: &[[u8; 32]],
) -> bool {
    let mut current = tx_hash.to_raw_hash();
    let mut index = tx_index;

    for sibling in proof {
        current = if index & 1 == 0 {
            bitcoin::hashes::sha256d::Hash::hash(&[&current[..], sibling].concat())
        } else {
            bitcoin::hashes::sha256d::Hash::hash(&[sibling, &current[..]].concat())
        };
        index >>= 1;
    }

    current == merkle_root.to_raw_hash()
}

pub fn verify_output_script(script: &bitcoin::Script, expected_hash: &[u8; 32]) -> bool {
    if script.is_p2wsh() {
        let script_hash = script.as_bytes();
        &script_hash[2..] == expected_hash
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
    Pubkey::find_program_address(&[b"block_hash", &height.to_le_bytes()], program_id)
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
