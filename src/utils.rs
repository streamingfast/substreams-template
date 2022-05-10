use crate::pb;

use hex;
use num_bigint::BigUint;
use std::str;

/// keccak value for Transfer(address,address,uint256)
const TRANSFER_TOPIC: &str = "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

/// Encodes an array of bytes in hex
///
/// `input`: Bytes
///
/// returns `String`: hex encoded input
pub fn address_pretty(input: &[u8]) -> String {
    format!("0x{}", hex::encode(input))
}

/// Checks if the given log is an erc20 transfer event by check the keccak value of the log topic
///
/// `log`: A log from a call
///
/// returns `bool`: true if erc20 transfer event, else false
pub fn is_erc20transfer_event(log: &pb::eth::Log) -> bool {
    if log.topics.len() != 3 || log.data.len() != 32 {
        return false;
    }
    return hex::encode(&log.topics[0]) == TRANSFER_TOPIC;
}

/// Find the erc20 storage changes (new and old valance of given holder)
///
/// `call`: Call containing storage changes
/// `holder`: Address of the holder (sender or the receiver)
///
/// returns `Vec<BalanceChange`: List of balance changes for a given holder
pub fn find_erc20_storage_changes(
    call: &pb::eth::Call,
    holder: &Vec<u8>,
) -> Vec<pb::erc20::BalanceChange> {
    let mut out: Vec<pb::erc20::BalanceChange> = Vec::new();
    let keys = erc20storage_keys_from_address(call, holder);

    for key in keys {
        let byte_key = hex::decode(key).unwrap();

        for change in &call.storage_changes {
            if change.key.eq(&byte_key) {
                let new_balance = BigUint::from_bytes_le(change.new_value.as_slice()).to_string();
                let old_balance = BigUint::from_bytes_le(change.old_value.as_slice()).to_string();

                let erc20_balance_change = &pb::erc20::BalanceChange {
                    old_balance,
                    new_balance,
                };

                out.push(erc20_balance_change.clone())
            }
        }
    }

    return out;
}

/// Get the storage keys from an address given a call
///
/// `call`: Call containing storage keys
/// `addr`: Address of the call
///
/// returns `Vec<String>`: List of hashes of keccak preimages
fn erc20storage_keys_from_address(call: &pb::eth::Call, addr: &Vec<u8>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let addr_as_hex = hex::encode(addr);
    for (hash, pre_image) in &call.keccak_preimages {
        if pre_image.chars().count() != 128 {
            continue;
        }

        // we're sure it's a top=level variable or something near that
        if &pre_image[64..126] != "00000000000000000000000000000000000000000000000000000000000000" {
            // Second part of the keccak should be a top-level
            continue;
        }

        if &pre_image[24..64] == addr_as_hex {
            out.push(String::from(hash));
        }
    }
    return out;
}
