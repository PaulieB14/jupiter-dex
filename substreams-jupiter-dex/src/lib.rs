use substreams::errors::Error;
use substreams::pb::substreams::store_delta::Operation;

mod pb;
use pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};
use pb::sf::substreams::v1::{EntityChanges, EntityChange, Field, Value};
use pb::sf::substreams::v1::value::TypedValue;

const JUPITER_SWAP: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
const JUPITER_LIMIT_ORDER: &str = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu";
const JUPITER_DCA: &str = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";

#[substreams::handlers::map]
pub fn map_jupiter_trades(block: Block) -> Result<EntityChanges, Error> {
    let mut changes = EntityChanges::default();

    for tx in block.transactions.iter() {
        if let Some(meta) = tx.meta {
            if meta.err.is_some() {
                continue; // Skip failed transactions
            }
        }

        if let Some(transaction) = &tx.transaction {
            if let Some(message) = &transaction.message {
                for instruction in message.instructions.iter() {
                    let program_id = &message.account_keys[instruction.program_id_index as usize];
                    let program_id_str = bs58::encode(program_id).into_string();
                    
                    if program_id_str == JUPITER_SWAP || program_id_str == JUPITER_LIMIT_ORDER || program_id_str == JUPITER_DCA {
                        let mut fields = Vec::new();
                        
                        // Basic transaction info
                        let tx_id = bs58::encode(&transaction.signatures[0]).into_string();
                        fields.push(Field {
                            name: "id".to_string(),
                            value: Some(Value {
                                typed_value: Some(TypedValue::StringValue(tx_id.clone()))
                            })
                        });

                        fields.push(Field {
                            name: "program_id".to_string(),
                            value: Some(Value {
                                typed_value: Some(TypedValue::StringValue(program_id_str.clone()))
                            })
                        });

                        // Block info
                        fields.push(Field {
                            name: "slot".to_string(),
                            value: Some(Value {
                                typed_value: Some(TypedValue::Int64Value(block.slot as i64))
                            })
                        });

                        if let Some(block_time) = block.block_time.as_ref() {
                            fields.push(Field {
                                name: "block_time".to_string(),
                                value: Some(Value {
                                    typed_value: Some(TypedValue::Int64Value(block_time.timestamp as i64))
                                })
                            });
                        }

                        fields.push(Field {
                            name: "block_hash".to_string(),
                            value: Some(Value {
                                typed_value: Some(TypedValue::StringValue(bs58::encode(&block.blockhash).into_string()))
                            })
                        });

                        // Signer info
                        let signer = bs58::encode(&message.account_keys[0]).into_string();
                        fields.push(Field {
                            name: "signer".to_string(),
                            value: Some(Value {
                                typed_value: Some(TypedValue::StringValue(signer))
                            })
                        });

                        // Transaction fee
                        if let Some(meta) = &tx.meta {
                            fields.push(Field {
                                name: "fee".to_string(),
                                value: Some(Value {
                                    typed_value: Some(TypedValue::Int64Value(meta.fee as i64))
                                })
                            });

                            // Token balances
                            for balance in meta.post_token_balances.iter() {
                                let mint = bs58::encode(&balance.mint).into_string();
                                let amount = balance.ui_token_amount.as_ref().unwrap().ui_amount;
                                
                                fields.push(Field {
                                    name: format!("token_balance_{}", mint),
                                    value: Some(Value {
                                        typed_value: Some(TypedValue::StringValue(amount.to_string()))
                                    })
                                });
                            }
                        }

                        // Add the trade entity
                        changes.changes.push(EntityChange {
                            entity_type: "Trade".to_string(),
                            id: tx_id,
                            fields,
                            operation: Operation::Create as i32,
                        });
                    }
                }
            }
        }
    }

    Ok(changes)
}
