use std::str::FromStr;
use std::str;
use substreams::pb::substreams::{Clock};
use substreams::{log, store};
use crate::pb;
use num_bigint;

pub fn process(
    _block: &Clock,
    transfer_deltas: store::Deltas,
) ->  pb::graphdb::EntitiesChanges {
    log::info!("about to process transfer_deltas items: {}", transfer_deltas.len());
    let mut entities_changes: pb::graphdb::EntitiesChanges = pb::graphdb::EntitiesChanges {
        entity_changes: vec![],
    };
    for delta in transfer_deltas {
        let mut field = pb::graphdb::Field {
            name: "count".to_string(),
            value_type: pb::graphdb::field::Type::Bigint as i32,
            ..Default::default()
        };
        let new_value_str = str::from_utf8(&delta.new_value).unwrap();
        let old_value_str = str::from_utf8(&delta.old_value).unwrap();

        log::info!("values: {} -> {}", old_value_str, new_value_str);
        if new_value_str != "" {
            field.new_value =  num_bigint::BigInt::from_str(&new_value_str).unwrap().to_signed_bytes_le();
            field.new_value_null = false;
        }

        if old_value_str != "" {
            field.old_value =  num_bigint::BigInt::from_str(&old_value_str).unwrap().to_signed_bytes_le();
            field.old_value_null = false;
        }

        entities_changes.entity_changes.push(pb::graphdb::EntityChange {
            entity: "holder".to_string(),
            id: delta.key.into_bytes(),
            ordinal: delta.ordinal,
            operation: delta.operation,
            fields: vec![
                field
            ],
        });
    }

    return entities_changes;
}