mod pb;
mod utils;

use num_bigint::BigUint;
use substreams::{log, proto};
use crate::pb::eth::Call;

/// Say hello to every first transaction in
/// of a transaction from a block
///
/// `block_ptr`: Pointer of where the block is the wasm heap memory
/// `block_len`: Length of the block in wasm heap memory
#[no_mangle]
pub extern "C" fn map_hello_world(block_ptr: *mut u8, block_len: usize) {
    // register panic hook to forward any panic to the executable
    substreams::register_panic_hook();

    // decode the block in the heap by using decode_ptr method from substreams lib
    let blk: pb::eth::Block = proto::decode_ptr(block_ptr, block_len).unwrap();

    // loop over every transaction
    for trx in blk.transaction_traces {

        // say hello
        log::println(format!("Hello, transaction sender: {}", utils::address_pretty(trx.from.as_slice())));
        log::println(format!("Hello, transaction receiver: {}", utils::address_pretty(trx.to.as_slice())));

        substreams::output(trx); // output the transaction
        break; // then get out
    }
}

#[no_mangle]
pub extern "C" fn store_hello_world(transaction_traces_ptr: *mut u8, transaction_traces_len: usize) {
    // register panic hook to forward any panic to the executable
    substreams::register_panic_hook();

    let transaction_traces: pb::eth::TransactionTrace = proto::decode_ptr(transaction_traces_ptr, transaction_traces_len).unwrap();

    log::println(format!("Hello Store, transaction sender: {}", utils::address_pretty(transaction_traces.from.as_slice())));
    log::println(format!("Hello Store, transaction receiver: {}", utils::address_pretty(transaction_traces.to.as_slice())));


}

//todo: use erc20-transfer and not
// have an example of the fetching the number of transfers by block
// maybe an example of the number (counter) of unique contracts in a block (bnb, weth, etc?)

/*
    1. get erc20 transfer
    2. store state ^^
    3. number of transfer per hour
*/

#[no_mangle]
pub extern "C" fn map_erc_20_transfer(block_ptr: *mut u8, block_len: usize) {
    substreams::register_panic_hook();

    let block: pb::eth::Block = proto::decode_ptr(block_ptr, block_len).unwrap();
    let mut transfers = pb::erc20::Transfers { transfers: vec![] };

    for trx in block.transaction_traces {
        for call in trx.calls {
            for log in call.clone().logs {
                if !utils::is_erc20transfer_event(&log) {
                    continue
                }

                let from_addr = &Vec::from(&log.topics[1][12..]);
                let to_addr = &Vec::from(&log.topics[2][12..]);
                let amount = &log.data[0..32];
                let log_ordinal = log.index as u64;

                let transfer_event = pb::erc20::Transfer {
                    from: utils::address_pretty(from_addr.as_slice()),
                    to: utils::address_pretty(to_addr.as_slice()),
                    amount: BigUint::from_bytes_le(amount).to_string(),
                    balance_change_from: utils::find_erc20_storage_changes(&call.clone(), from_addr),
                    balance_change_to: utils::find_erc20_storage_changes(&call.clone(), to_addr),
                    log_ordinal
                };

                transfers.transfers.push(transfer_event);
            }
        }
    }

    substreams::output(transfers);
}

