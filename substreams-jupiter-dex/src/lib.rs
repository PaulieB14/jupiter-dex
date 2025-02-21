use substreams::errors::Error;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

mod pb;

const JUPITER_SWAP: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
const JUPITER_LIMIT_ORDER: &str = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu";
const JUPITER_DCA: &str = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";

#[substreams::handlers::map]
pub fn map_jupiter_trades(block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for tx in block.transactions.iter() {
        if let Some(meta) = &tx.meta {
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
                        let tx_id = bs58::encode(&transaction.signatures[0]).into_string();
                        
                        let trade = tables.create_row("Trade", &tx_id);
                        
                        // Basic transaction info
                        trade.set("id", tx_id);
                        trade.set("program_id", program_id_str);

                        // Block info
                        trade.set("slot", block.slot as i64);
                        if let Some(block_time) = block.block_time.as_ref() {
                            trade.set("block_time", block_time.timestamp as i64);
                        }
                        trade.set("block_hash", bs58::encode(&block.blockhash).into_string());

                        // Signer info
                        trade.set("signer", bs58::encode(&message.account_keys[0]).into_string());

                        // Transaction fee
                        if let Some(meta) = &tx.meta {
                            trade.set("fee", meta.fee as i64);

                            // Token balances
                            for balance in meta.post_token_balances.iter() {
                                let mint = bs58::encode(&balance.mint).into_string();
                                let amount = balance.ui_token_amount.as_ref().unwrap().ui_amount;
                                let field_name = format!("token_balance_{}", mint);
                                trade.set(&field_name, amount.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(tables.to_entity_changes())
}
