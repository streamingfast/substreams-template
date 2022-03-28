mod pb;
mod utils;

use num_bigint::BigUint;
use substreams::{log, proto};

/// Say hello to every first transaction in of a transaction from a block
///
/// `block_ptr`: Pointer of where the block is located in the wasm heap memory
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

//todo: use erc20-transfer and not pancake stuff
// have an example of the fetching the number of transfers by block
// maybe an example of the number (counter) of unique contracts in a block (bnb, weth, etc?)

/*
    1. get erc20 transfer => Done
    2. store state ^^ => todo
    3. number of transfer per hour => todo
*/

/// Find and output all the ERC20 transfers
///
/// `block_ptr`: Pointer of where the block is located in the wasm heap memory
/// `block_len`: Length of the block in wasm heap memory
#[no_mangle]
pub extern "C" fn map_erc_20_transfer(block_ptr: *mut u8, block_len: usize) {
    // register panic hook to forward any panic to the executable
    substreams::register_panic_hook();

    log::println("SALUT".to_string());

    // decode the block in the heap by using decode_ptr method from substreams lib
    let block: pb::eth::Block = proto::decode_ptr(block_ptr, block_len).unwrap();

    // declare transfers which will contain all the erc20 transfers
    let mut transfers = pb::erc20::Transfers { transfers: vec![] };

    // loop over every transaction
    for trx in block.transaction_traces {
        // loop over every call for a given transaction
        for call in trx.calls {
            // loop over every log in a given call
            for log in call.clone().logs {

                // check if the log is an erc20 transfer event
                if !utils::is_erc20transfer_event(&log) {
                    continue
                }

                // get required values to create transfer event
                let from_addr = &Vec::from(&log.topics[1][12..]);
                let to_addr = &Vec::from(&log.topics[2][12..]);
                let amount = &log.data[0..32];
                let log_ordinal = log.index as u64;

                // create transfer event
                let transfer_event = pb::erc20::Transfer {
                    from: utils::address_pretty(from_addr.as_slice()),
                    to: utils::address_pretty(to_addr.as_slice()),
                    amount: BigUint::from_bytes_le(amount).to_string(),
                    balance_change_from: utils::find_erc20_storage_changes(&call.clone(), from_addr),
                    balance_change_to: utils::find_erc20_storage_changes(&call.clone(), to_addr),
                    log_ordinal
                };

                // add the event to the transfers in the transfers' list
                transfers.transfers.push(transfer_event);
            }
        }
    }

    // output transfers
    substreams::output(transfers);
}

