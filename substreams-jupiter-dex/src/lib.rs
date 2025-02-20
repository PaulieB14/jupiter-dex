use substreams::errors::Error;
use substreams_solana::pb::sol::v1::Block;

mod pb;
use pb::sf::substreams::v1::{EntityChanges, EntityChange, Field, Value};
use substreams::pb::substreams::store_delta::Operation;
use pb::sf::substreams::v1::value::TypedValue;

const JUPITER_SWAP: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
const JUPITER_LIMIT_ORDER: &str = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu";
const JUPITER_DCA: &str = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";

impl Default for EntityChanges {
    fn default() -> Self {
        EntityChanges { changes: Vec::new() }
    }
}

impl prost::Message for EntityChanges {
    fn encoded_len(&self) -> usize {
        self.changes.iter().map(|change| change.encoded_len()).sum()
    }

    fn merge_field<B: prost::bytes::Buf>(&mut self, _: u32, data: ::prost::encoding::WireType, buf: &mut B) -> Result<(), prost::EncodeError> {
        match data {
            prost::encoding::WireType::LengthDelimited => {
                let mut change = EntityChange::default();
                change.merge_field(data, buf)?;
                self.changes.push(change);
            }
            _ => return Err(prost::EncodeError::WireType(data)),
        }
        Ok(())
    }

    fn encode(&self, buf: &mut Vec<u8>) -> Result<(), prost::EncodeError> {
        for change in &self.changes {
            change.encode(buf)?;
        }
        Ok(())
    }
}

impl prost::Message for EntityChange {
    fn encoded_len(&self) -> usize {
        let mut len = 0;
        len += prost::encoding::encoded_len_varint(1, self.entity_type.as_str().len() as u64);
        len += self.entity_type.as_bytes().len();
        len += prost::encoding::encoded_len_varint(2, self.id.as_str().len() as u64);
        len += self.id.as_bytes().len();
        len += prost::encoding::encoded_len_varint(3, self.operation as u64);
        for field in &self.fields {
            len += field.encoded_len();
        }
        len
    }

    fn merge_field<B: prost::bytes::Buf>(&mut self, tag: u32, data: ::prost::encoding::WireType, buf: &mut B) -> Result<(), prost::EncodeError> {
        match tag {
            1 => {
                let len = prost::encoding::decode_varint(buf)?;
                let mut bytes = vec![0; len as usize];
                buf.read_exact(&mut bytes)?;
                self.entity_type = String::from_utf8(bytes).map_err(|e| prost::EncodeError::Message(e.to_string()))?;
            }
            2 => {
                let len = prost::encoding::decode_varint(buf)?;
                let mut bytes = vec![0; len as usize];
                buf.read_exact(&mut bytes)?;
                self.id = String::from_utf8(bytes).map_err(|e| prost::EncodeError::Message(e.to_string()))?;
            }
            3 => {
                self.operation = prost::encoding::decode_varint(buf)? as i32;
            }
            4 => {
                let mut field = Field::default();
                field.merge_field(data, buf)?;
                self.fields.push(field);
            }
            _ => return Err(prost::EncodeError::WireType(data)),
        }
        Ok(())
    }

    fn encode(&self, buf: &mut Vec<u8>) -> Result<(), prost::EncodeError> {
        prost::encoding::encode_varint(1, self.entity_type.as_str().len() as u64, buf);
        buf.extend(self.entity_type.as_bytes());
        prost::encoding::encode_varint(2, self.id.as_str().len() as u64, buf);
        buf.extend(self.id.as_bytes());
        prost::encoding::encode_varint(3, self.operation as u64, buf);
        for field in &self.fields {
            field.encode(buf)?;
        }
        Ok(())
    }
}

#[substreams::handlers::map]
pub fn map_jupiter_trades(block: Block) -> Result<EntityChanges, Error> {
    let mut changes = EntityChanges::default();

    for tx in block.transactions.iter() {
        if tx.meta.as_ref().unwrap().err.is_some() {
            continue; // Skip failed transactions
        }

        for instruction in tx.transaction.as_ref().unwrap().walk_instructions() {
            if let Some(program_id) = instruction.program_id() {
                let program_id_str = bs58::encode(program_id).into_string();
                if program_id_str == JUPITER_SWAP || program_id_str == JUPITER_LIMIT_ORDER || program_id_str == JUPITER_DCA {
                    let mut fields = Vec::new();
                    
                    // Basic transaction info
                    let tx_id = bs58::encode(&tx.transaction.as_ref().unwrap().signatures[0]).into_string();
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

                    // Instruction info
                    fields.push(Field {
                        name: "instruction_data".to_string(),
                        value: Some(Value {
                            typed_value: Some(TypedValue::BytesValue(instruction.data.clone()))
                        })
                    });

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

    Ok(changes)
}
