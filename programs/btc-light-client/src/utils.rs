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
