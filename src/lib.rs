mod pb;
mod utils;
use num_bigint::BigUint;
use substreams::{log, proto, state, Hex, errors::SubstreamError};

/// Say hello to every first transaction in of a transaction from a block
///
/// `blk`: Ethereum block
#[substreams::handler(type = "map")]
fn map_hello_world(blk: pb::eth::Block) -> Result<pb::eth::TransactionTrace, SubstreamError> {
    for trx in blk.transaction_traces {
        if trx.status != pb::eth::TransactionTraceStatus::Succeeded as i32 {
            // Only log successful transactions
            continue;
        }

        log::info!(
            "Hello, transaction from {} to {}",
            Hex(&trx.from),
            Hex(&trx.to)
        );

        return Ok(trx);
    }
    return Err(SubstreamError::new("block does not contain any transaction"))
}

/// Find and output all the ERC20 transfers
///
/// `blk`: Ethereum block
#[substreams::handler(type = "map")]
fn map_erc_20_transfer(blk: pb::eth::Block) -> Result<pb::erc20::Transfers, SubstreamError> {
    let mut transfers: Vec<pb::erc20::Transfer> = vec![];
    for trx in blk.transaction_traces {
        for call in trx.calls {
            if call.state_reverted {
                // If the call has been reverted, we should not map anything here
                continue;
            }

            transfers.extend(
                call.logs
                    .iter()
                    .filter(|log| utils::is_erc20transfer_event(log))
                    .map(|log| {
                        let from_addr = &Vec::from(&log.topics[1][12..]);
                        let to_addr = &Vec::from(&log.topics[2][12..]);
                        let amount = &log.data[0..32];
                        let log_ordinal = log.index as u64;

                        pb::erc20::Transfer {
                            from: Hex::encode(from_addr),
                            to: Hex::encode(to_addr),
                            amount: BigUint::from_bytes_le(amount).to_string(),
                            balance_change_from: utils::find_erc20_storage_changes(
                                &call, from_addr,
                            ),
                            balance_change_to: utils::find_erc20_storage_changes(&call, to_addr),
                            log_ordinal,
                        }
                    }),
            );
        }
    }
    Ok(pb::erc20::Transfers { transfers })
}

/// Build the erc 20 transfer store
///
/// `transfers`: ERC20 transfers
#[substreams::handler(type = "store")]
fn build_erc_20_transfer_state(transfers: pb::erc20::Transfers) {
    for transfer in transfers.transfers {
        state::set(
            1,
            format!("transfer:{}:{}", transfer.from, transfer.to),
            &proto::encode(&transfer).unwrap(),
        )
    }
}

/// Gets a counter of the number of transfers in a given transfers object (which is set by block)
///
/// `transfers`: ERC20 transfers
#[substreams::handler(type = "map")]
fn map_number_of_transfers_erc_20_transfer(transfers: pb::erc20::Transfers) -> Result<pb::counter::Counter, SubstreamError>{
    let counter: pb::counter::Counter = pb::counter::Counter {
        transfer_count: transfers.transfers.len() as u64,
    };
    log::debug!("Transfer count {}", counter.transfer_count);
    Ok(counter)
}

/// Find and output all the contract created
///
/// `blk`: Ethereum block
#[substreams::handler(type = "map")]
fn map_contract_creation(blk: pb::eth::Block) -> Result<pb::contract::Contracts,SubstreamError>{
    let mut contracts: Vec<pb::contract::Contract> = vec![];
    for trx in blk.transaction_traces {
        contracts.extend(
            trx.calls
                .iter()
                .filter(|call| {
                    call.call_type == pb::eth::CallType::Create as i32 && !call.state_reverted
                })
                .map(|call| pb::contract::Contract {
                    address: call.address.clone(),
                }),
        );
    }

    Ok(pb::contract::Contracts { contracts: vec![] })
}
