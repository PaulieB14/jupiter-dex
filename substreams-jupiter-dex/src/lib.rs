use substreams::errors::Error;
use substreams::log;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

mod pb;

// Jupiter Program IDs
const JUPITER_AMMS: &[&str] = &[
    // Core programs
    "JUP4Fb2cqiRUcaTHdrPC8h2g7yFhLMZB19XWN5Q7McL",  // Core
    "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",  // v6
    "JUP2jxvXaqu7NQY1GmNF4m1vodw12LVXYxbFL2uJvfo",  // v2
    "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph",  // v3
    "JUP5cHjnnCx2DppVsufsHpwj7mZb8GKsTNh6T8mnzFf",  // v4
    
    // Additional programs
    "JUPNZLmBYhqApWWb9YrKvNsaGWxSqWpLh5FhqgfVKKfk",  // Versioned
    "JUP7th7h8x5RhKXEiWxzHpf8KoZGstuFKHGtszxYEyF",   // Limit Order
    "JUPGShgHoH5ZXMQy7mZ3FSRqbMrRJSZo3qUdxgA5LBJ",   // Stats
    "JUPcgE9dCJAExe3zQYzDmGHcg5JBnFLRWJcUqXn5B59",   // Price Feed
];

// Known Jupiter method IDs
const JUPITER_METHOD_IDS: &[(&str, &str)] = &[
    // Core methods
    ("d6eb2e64", "route"),           // route(RouteParams)
    ("d5eca8f9", "exactInput"),      // exactInput(ExactInputParams)
    ("f44947b3", "exactOutput"),     // exactOutput(ExactOutputParams)
    
    // Token ledger methods
    ("f7286c7c", "routeWithTokenLedger"), // routeWithTokenLedger(RouteWithTokenLedgerParams)
    ("e31ed638", "setTokenLedger"),  // setTokenLedger(TokenLedgerParams)
    
    // Versioned methods
    ("b5b3f362", "routeV2"),         // routeV2(RouteV2Params)
    ("a7e338e4", "exactInputV2"),    // exactInputV2(ExactInputV2Params)
    ("cb9124e4", "exactOutputV2"),   // exactOutputV2(ExactOutputV2Params)
    
    // Limit order methods
    ("c3b3f38a", "placeLimitOrder"), // placeLimitOrder(LimitOrderParams)
    ("d8a42e86", "cancelLimitOrder") // cancelLimitOrder(LimitOrderId)
];

// Helper function to get Jupiter method type
fn get_jupiter_method_type(method_id: &[u8]) -> &'static str {
    if method_id.len() < 4 {
        return "Unknown";
    }
    
    let method_hex = hex::encode(&method_id[..4]);
    for (id, name) in JUPITER_METHOD_IDS {
        if method_hex.starts_with(id) {
            return name;
        }
    }
    "Unknown"
}

// Helper function to get Jupiter program type
fn get_jupiter_program_type(program_id: &str) -> &'static str {
    if program_id.starts_with("JUP6") {
        "v6"
    } else if program_id.starts_with("JUP4") {
        "Core"
    } else if program_id.starts_with("JUP2") {
        "v2"
    } else if program_id.starts_with("JUP3") {
        "v3"
    } else if program_id.starts_with("JUP5") {
        "v4"
    } else if program_id.starts_with("JUPN") {
        "Versioned"
    } else if program_id.starts_with("JUP7") {
        "Limit Order"
    } else if program_id.starts_with("JUPG") {
        "Stats"
    } else if program_id.starts_with("JUPc") {
        "Price Feed"
    } else {
        "Unknown"
    }
}

// Target transaction details
const TARGET_TX_STR: &str = "2k9An8rvpYHcECgtNQUJJRkZwqXWHBZpbbMcYoFaS8vov4ZoUEFKRiQy73Bjx3Adc7WzXMaF163vWqKakV76hB4V";
const TARGET_BLOCK: u64 = 322167085;

fn is_target_transaction(raw_sig: &[u8], block_slot: u64) -> bool {
    // First check block number
    if block_slot != TARGET_BLOCK {
        return false;
    }

    // Then check signature if block matches
    if let Ok(target_bytes) = bs58::decode(TARGET_TX_STR).into_vec() {
        raw_sig == target_bytes.as_slice()
    } else {
        false
    }
}

#[substreams::handlers::map]
pub fn map_jupiter_trades(block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    log::info!("Processing block:");
    log::info!("- Slot: {} (target: {}, match: {})", 
        block.slot, TARGET_BLOCK, block.slot == TARGET_BLOCK);
    
    // Use safer handling of blockhash
    log::info!("- Hash: {}", bs58::encode(&block.blockhash).into_string());
    log::info!("- Parent Hash: {}", bs58::encode(&block.previous_blockhash).into_string());
    log::info!("- Timestamp: {}", block.block_time.as_ref().map_or("Unknown".to_string(), |bt| bt.timestamp.to_string()));
    log::info!("- Transactions: {}", block.transactions.len());
    log::info!("- Failed Transactions: {}", block.transactions.iter()
        .filter(|tx| tx.meta.as_ref().and_then(|m| m.err.as_ref()).is_some())
        .count());

    log::info!("Creating protocol entities for Jupiter AMM programs:");
    for &program_id in JUPITER_AMMS {
        let program_type = get_jupiter_program_type(program_id);
        log::info!("Creating protocol entity:");
        log::info!("- ID: {}", program_id);
        log::info!("- Type: {}", program_type);
        log::info!("- Initial Stats:");
        log::info!("  * Unique Users: 0");
        log::info!("  * Pool Count: 0");
        
        let protocol = tables.create_row("Protocol", program_id);
        protocol.set("id", program_id);
        protocol.set("cumulativeUniqueUsers", 0i64);
        protocol.set("totalPoolCount", 0i64);
    }

    // Use Block::transactions() helper for safer iteration over successful transactions
    for (tx_idx, tx) in block.transactions.iter().enumerate() {
        // Skip failed transactions early
        if tx.meta.as_ref().and_then(|m| m.err.as_ref()).is_some() {
            log::info!("Skipping failed transaction {}", tx_idx);
            continue;
        }
        
        // Get transaction signature safely
        let raw_sig = tx.transaction.as_ref()
            .and_then(|t| t.signatures.first())
            .map(|sig| sig.as_slice())
            .unwrap_or(&[]);
            
        // Use safer handling
        let tx_signature = if !raw_sig.is_empty() {
            bs58::encode(raw_sig).into_string()
        } else {
            "unknown".to_string()
        };
        
        let is_target = is_target_transaction(raw_sig, block.slot);
        
        // Log signature details
        log::info!("Processing transaction {} with signature {} (target: {}, match: {})", 
            tx_idx, tx_signature, TARGET_TX_STR, is_target);
        
        if let Some(meta) = &tx.meta {
            // Log token balances if this is our target transaction
            if is_target_transaction(raw_sig, block.slot) {
                log::info!("Target transaction token balances:");
                for (idx, balance) in meta.pre_token_balances.iter().enumerate() {
                    if balance.mint.is_empty() {
                        continue; // Skip entries with empty mint addresses
                    }
                    
                    log::info!("Pre balance {}: Mint: {}, Owner: {}, Amount: {:?}", 
                        idx,
                        bs58::encode(&balance.mint).into_string(),
                        bs58::encode(&balance.owner).into_string(),
                        balance.ui_token_amount.as_ref().map(|a| a.ui_amount)
                    );
                }
                
                for (idx, balance) in meta.post_token_balances.iter().enumerate() {
                    if balance.mint.is_empty() {
                        continue; // Skip entries with empty mint addresses
                    }
                    
                    log::info!("Post balance {}: Mint: {}, Owner: {}, Amount: {:?}",
                        idx,
                        bs58::encode(&balance.mint).into_string(),
                        bs58::encode(&balance.owner).into_string(),
                        balance.ui_token_amount.as_ref().map(|a| a.ui_amount)
                    );
                }
            }

            if let Some(transaction) = &tx.transaction {
                if let Some(message) = &transaction.message {
                    // Process all instructions (main and inner) using walk_instructions
                    for instruction_view in tx.walk_instructions() {
                        let program_id_str = instruction_view.program_id().to_string();
                        
                        // Check if this is a Jupiter AMM program
                        let is_jupiter_amm = JUPITER_AMMS.contains(&program_id_str.as_str());
                        
                        // Also check if any account in the instruction references Jupiter AMMs
                        let has_jupiter_account = instruction_view.accounts()
                            .iter()
                            .any(|account| JUPITER_AMMS.contains(&account.to_string().as_str()));
                        
                        if is_jupiter_amm || has_jupiter_account {
                            log::info!("Found Jupiter program in instruction:");
                            log::info!("- Program ID: {}", program_id_str);
                            log::info!("- Transaction: {}", tx_signature);
                            
                            // Log instruction data if this is our target transaction
                            if is_target_transaction(raw_sig, block.slot) {
                                log::info!("Target tx instruction data: 0x{}", hex::encode(instruction_view.data()));
                                
                                // Check if this looks like a swap instruction
                                if instruction_view.data().len() >= 8 {
                                    let method_id = &instruction_view.data()[..8];
                                    let method_type = get_jupiter_method_type(method_id);
                                    log::info!("Found Jupiter instruction: {}", method_type);
                                }
                            }
                            
                            process_jupiter_instruction(tx, transaction, message, &program_id_str, &mut tables, &block)?;
                        }
                    }
                }
            }
        }
    }

    let changes = tables.to_entity_changes();
    log::info!("Generated {} entity changes", changes.entity_changes.len());
    
    // Group changes by entity type
    let mut protocols = 0;
    let mut pools = 0;
    let mut swaps = 0;
    
    for change in &changes.entity_changes {
        match change.entity.as_str() {
            "Protocol" => protocols += 1,
            "LiquidityPool" => pools += 1,
            "Swap" => swaps += 1,
            _ => {}
        }
    }
    
    log::info!("Entity change summary:");
    log::info!("- Protocols: {}", protocols);
    log::info!("- Pools: {}", pools);
    log::info!("- Swaps: {}", swaps);
    
    Ok(changes)
}

fn process_jupiter_instruction(
    tx: &substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction,
    transaction: &substreams_solana::pb::sf::solana::r#type::v1::Transaction,
    message: &substreams_solana::pb::sf::solana::r#type::v1::Message,
    program_id_str: &str,
    tables: &mut Tables,
    block: &Block,
) -> Result<(), Error> {
    // Use safer transaction ID handling
    let tx_id = if !transaction.signatures.is_empty() {
        bs58::encode(&transaction.signatures[0]).into_string()
    } else {
        "unknown".to_string()
    };
    
    let swap_id = format!("swap-{}", tx_id);
    
    log::info!("Processing Jupiter transaction {} (checking if matches target: {})", swap_id, tx_id == TARGET_TX_STR);
    
    // Log which Jupiter program was found
    let program_type = get_jupiter_program_type(program_id_str);
    
    log::info!("Found Jupiter program: {} (Type: {})", program_id_str, program_type);
    
    // Create pool entity
    if let Some(meta) = &tx.meta {
        log::info!("Found {} post token balances for tx {}", meta.post_token_balances.len(), tx_id);
        
        // Track all token balance changes
        let mut token_changes = Vec::new();
        
        // Track tokens that decreased (tokens spent)
        for pre_balance in meta.pre_token_balances.iter() {
            // Skip entries with empty mint addresses
            if pre_balance.mint.is_empty() {
                continue;
            }
            
            let mint = bs58::encode(&pre_balance.mint).into_string();
            
            let pre_amount = pre_balance.ui_token_amount.as_ref()
                .map(|a| a.ui_amount)
                .unwrap_or(0.0);
                
            let post_amount = meta.post_token_balances.iter()
                .find(|b| b.mint == pre_balance.mint && b.owner == pre_balance.owner)
                .and_then(|b| b.ui_token_amount.as_ref())
                .map(|a| a.ui_amount)
                .unwrap_or(0.0);
                
            if pre_amount > post_amount {
                log::info!("Found spent token: {} ({} -> {})", mint, pre_amount, post_amount);
                token_changes.push((mint, pre_amount, post_amount, true));
            }
        }
        
        // Track tokens that increased (tokens received)
        for post_balance in meta.post_token_balances.iter() {
            // Skip entries with empty mint addresses
            if post_balance.mint.is_empty() {
                continue;
            }
            
            let mint = bs58::encode(&post_balance.mint).into_string();
            
            let post_amount = post_balance.ui_token_amount.as_ref()
                .map(|a| a.ui_amount)
                .unwrap_or(0.0);
                
            let pre_amount = meta.pre_token_balances.iter()
                .find(|b| b.mint == post_balance.mint && b.owner == post_balance.owner)
                .and_then(|b| b.ui_token_amount.as_ref())
                .map(|a| a.ui_amount)
                .unwrap_or(0.0);
                
            if post_amount > pre_amount {
                log::info!("Found received token: {} ({} -> {})", mint, pre_amount, post_amount);
                token_changes.push((mint, pre_amount, post_amount, false));
            }
        }

        // Find token in/out from changes
        if let (Some(spent), Some(received)) = (
            token_changes.iter().find(|(_, _, _, is_spent)| *is_spent),
            token_changes.iter().find(|(_, _, _, is_spent)| !*is_spent)
        ) {
            let (token_in, amount_in_pre, amount_in_post, _) = spent;
            let (token_out, amount_out_pre, amount_out_post, _) = received;
            let pool_id = format!("{}-{}-{}", program_id_str, token_in, token_out);
            
            log::info!("Creating pool entity {} for tokens {} and {} (tx: {})", 
                pool_id, token_in, token_out, tx_id);

            log::info!("Swap details:");
            log::info!("- Token In : {} ({} -> {}, change: {})", 
                token_in, amount_in_pre, amount_in_post, (amount_in_pre - amount_in_post).abs());
            log::info!("- Token Out: {} ({} -> {}, change: {})", 
                token_out, amount_out_pre, amount_out_post, (amount_out_post - amount_out_pre).abs());
            
            // Create pool entity
            let pool = tables.create_row("LiquidityPool", &pool_id);
            pool.set("id", &pool_id);
            pool.set("protocol", program_id_str);
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
            swap.set("protocol", program_id_str);
            swap.set("pool", &pool_id);
            
            // Add bounds check for account keys and use safer handling
            let from_key = if !message.account_keys.is_empty() {
                bs58::encode(&message.account_keys[0]).into_string()
            } else {
                "unknown".to_string()
            };
            
            swap.set("from", from_key.clone());
            swap.set("to", from_key);
            swap.set("slot", block.slot as i64);
            swap.set("blockNumber", block.slot as i64);
            swap.set("timestamp", block.block_time.as_ref().map_or(0i64, |bt| bt.timestamp));
            swap.set("tokenIn", token_in);
            swap.set("tokenOut", token_out);
            
            // Set amounts based on the changes
            swap.set("amountIn", (amount_in_pre - amount_in_post).abs().to_string());
            swap.set("amountOut", (amount_out_post - amount_out_pre).abs().to_string());
        }
    }

    Ok(())
}
