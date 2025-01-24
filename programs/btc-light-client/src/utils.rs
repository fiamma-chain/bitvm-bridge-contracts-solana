use anchor_lang::prelude::*;
use bitcoin::{hashes::Hash, Work};

use crate::errors::BtcLightClientError;

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

pub fn get_work_in_period(
    state: &crate::state::BtcLightClientState,
    period: u64,
    height: u64,
) -> Result<Work> {
    let target_bytes = state.get_period_target(period)?;
    let target = bitcoin::Target::from_be_bytes(target_bytes);
    let work_per_block = target.to_work();

    let num_blocks = height - (period * 2016) + 1;
    require!(
        num_blocks >= 1 && num_blocks <= 2016,
        BtcLightClientError::InvalidBlockCount
    );

    let mut total_work = work_per_block;
    for _ in 1..num_blocks {
        total_work = total_work + work_per_block;
    }
    Ok(total_work)
}
