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
                        let swap_id = format!("swap-{}", tx_id);
                        
                        let swap = tables.create_row("Swap", &swap_id);
                        
                        // Basic transaction info
                        swap.set("id", swap_id);
                        swap.set("blockHash", bs58::encode(&block.blockhash).into_string());
                        swap.set("protocol", "jupiter-dex");
                        swap.set("to", bs58::encode(&message.account_keys[0]).into_string());
                        swap.set("from", bs58::encode(&message.account_keys[0]).into_string());
                        swap.set("slot", block.slot as i64);
                        swap.set("blockNumber", block.slot as i64);

                        if let Some(block_time) = block.block_time.as_ref() {
                            swap.set("timestamp", block_time.timestamp as i64);
                        }

                        // Token info
                        if let Some(meta) = &tx.meta {
                            for balance in meta.post_token_balances.iter() {
                                let mint = bs58::encode(&balance.mint).into_string();
                                let amount = balance.ui_token_amount.as_ref().unwrap().ui_amount;
                                
                                // For now, just set the first token as tokenIn and second as tokenOut
                                swap.set("tokenIn", mint.clone());
                                swap.set("amountIn", amount.to_string());
                                swap.set("tokenOut", mint);
                                swap.set("amountOut", amount.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(tables.to_entity_changes())
}
