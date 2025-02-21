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

    // Create protocol entities
    for protocol_id in [JUPITER_SWAP, JUPITER_LIMIT_ORDER, JUPITER_DCA] {
        let protocol = tables.create_row("Protocol", protocol_id);
        protocol.set("id", protocol_id);
        protocol.set("cumulativeUniqueUsers", 0i64);
        protocol.set("totalPoolCount", 0i64);
    }

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
                        
                        // Create pool entity
                        if let Some(meta) = &tx.meta {
                            if let Some(first_balance) = meta.post_token_balances.first() {
                                if let Some(second_balance) = meta.post_token_balances.get(1) {
                                    let token_in = bs58::encode(&first_balance.mint).into_string();
                                    let token_out = bs58::encode(&second_balance.mint).into_string();
                                    let pool_id = format!("{}-{}-{}", program_id_str, token_in, token_out);
                                    
                                    let pool = tables.create_row("LiquidityPool", &pool_id);
                                    pool.set("id", &pool_id);
                                    pool.set("protocol", &program_id_str);
                                    pool.set("inputTokens", vec![token_in.clone(), token_out.clone()]);
                                    pool.set("token0Balance", 0i64);
                                    pool.set("token1Balance", 0i64);
                                    pool.set("outputTokenSupply", 0i64);
                                    pool.set("cumulativeVolumeByTokenAmount", vec!["0".to_string(), "0".to_string()]);
                                    pool.set("createdTimestamp", block.block_time.as_ref().map_or(0i64, |bt| bt.timestamp));
                                    pool.set("createdBlockNumber", block.slot as i64);

                                    // Create swap entity
                                    let swap = tables.create_row("Swap", &swap_id);
                                    swap.set("id", &swap_id);
                                    swap.set("blockHash", bs58::encode(&block.blockhash).into_string());
                                    swap.set("protocol", &program_id_str);
                                    swap.set("pool", &pool_id);
                                    swap.set("from", bs58::encode(&message.account_keys[0]).into_string());
                                    swap.set("to", bs58::encode(&message.account_keys[0]).into_string());
                                    swap.set("slot", block.slot as i64);
                                    swap.set("blockNumber", block.slot as i64);
                                    swap.set("timestamp", block.block_time.as_ref().map_or(0i64, |bt| bt.timestamp));
                                    swap.set("tokenIn", &token_in);
                                    swap.set("tokenOut", &token_out);
                                    
                                    if let Some(amount_in) = first_balance.ui_token_amount.as_ref() {
                                        swap.set("amountIn", amount_in.ui_amount.to_string());
                                    }
                                    if let Some(amount_out) = second_balance.ui_token_amount.as_ref() {
                                        swap.set("amountOut", amount_out.ui_amount.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(tables.to_entity_changes())
}
