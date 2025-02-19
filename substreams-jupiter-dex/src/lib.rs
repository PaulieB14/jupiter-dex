use substreams::errors::Error;

mod pb;
use pb::sf::solana::accounts::v1::AccountChange;
use pb::sf::substreams::v1::{EntityChanges, EntityChange, Field, Value, Operation};
use pb::sf::substreams::v1::value::TypedValue;

#[substreams::handlers::map]
fn map_filtered_accounts(account_changes: AccountChange) -> Result<EntityChanges, Error> {
    let mut changes = Vec::new();

    let pubkey = bs58::encode(&account_changes.pubkey).into_string();
    let owner = bs58::encode(&account_changes.owner).into_string();
    
    let mut fields = Vec::new();
    
    fields.push(Field {
        name: "pubkey".to_string(),
        value: Some(Value {
            typed_value: Some(TypedValue::StringValue(pubkey.clone()))
        })
    });
    
    fields.push(Field {
        name: "owner".to_string(),
        value: Some(Value {
            typed_value: Some(TypedValue::StringValue(owner))
        })
    });
    
    fields.push(Field {
        name: "lamports".to_string(),
        value: Some(Value {
            typed_value: Some(TypedValue::Int64Value(account_changes.lamports as i64))
        })
    });
    
    fields.push(Field {
        name: "slot".to_string(),
        value: Some(Value {
            typed_value: Some(TypedValue::Int64Value(account_changes.slot as i64))
        })
    });
    
    fields.push(Field {
        name: "executable".to_string(),
        value: Some(Value {
            typed_value: Some(TypedValue::BoolValue(account_changes.executable))
        })
    });
    
    fields.push(Field {
        name: "rentEpoch".to_string(),
        value: Some(Value {
            typed_value: Some(TypedValue::Int64Value(account_changes.rent_epoch as i64))
        })
    });
    
    fields.push(Field {
        name: "data".to_string(),
        value: Some(Value {
            typed_value: Some(TypedValue::BytesValue(account_changes.data))
        })
    });

    changes.push(EntityChange {
        entity_type: "Account".to_string(),
        id: pubkey,
        fields,
        operation: if account_changes.deleted {
            Operation::Delete as i32
        } else {
            Operation::Create as i32
        },
    });

    Ok(EntityChanges {
        changes
    })
}
