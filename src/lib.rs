mod abi;
mod pb;
mod graph;
use hex_literal::hex;
use pb::erc721;
use substreams::{log, store, Hex};
use substreams_ethereum::{pb::eth::v1 as eth, NULL_ADDRESS};

// Bored Ape Club Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("bc4ca0eda7647a8ab7c2061c2e118a18a936f13d");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<erc721::Transfers, substreams::errors::Error> {
    let mut transfers: Vec<erc721::Transfer> = vec![];
    for trx in blk.transaction_traces {
        transfers.extend(trx.receipt.unwrap().logs.iter().filter_map(|log| {
            if log.address != TRACKED_CONTRACT {
                return None;
            }

            log::info!("NFT Contract {} invoked", Hex(&TRACKED_CONTRACT));

            if !abi::erc721::events::Transfer::match_log(log) {
                return None;
            }

            let transfer = abi::erc721::events::Transfer::must_decode(log);

            Some(erc721::Transfer {
                trx_hash: trx.hash.clone(),
                from: transfer.from,
                to: transfer.to,
                token_id: transfer.token_id.low_u64(),
                ordinal: log.block_index as u64,
            })
        }));
    }

    Ok(erc721::Transfers { transfers })
}

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
#[substreams::handlers::store]
fn store_transfers(transfers: erc721::Transfers, s: store::StoreAddInt64) {
    log::info!("NFT state builder");
    for transfer in transfers.transfers {
        if transfer.from != NULL_ADDRESS {
            log::info!("Found a transfer out {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.from), -1);
        }

        if transfer.to != NULL_ADDRESS {
            log::info!("Found a transfer in {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.to), 1);
        }
    }
}


#[substreams::handlers::map]
pub fn graph_entities(
    block: substreams::pb::substreams::Clock,
    transfers_deltas: store::Deltas,
) -> Result<pb::graphdb::EntitiesChanges, substreams::errors::Error> {
    log::info!(
        "graph_entities: transfers deltas:{}",
        transfers_deltas.len(),
    );
    let changes = graph::process(&block,transfers_deltas);
    return Ok(changes);
}



fn generate_key(holder: &Vec<u8>) -> String {
    return format!("total:{}:{}", Hex(holder), Hex(TRACKED_CONTRACT));
}



