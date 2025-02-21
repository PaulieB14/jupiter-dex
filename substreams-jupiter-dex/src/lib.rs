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
    log::info!("- Hash: {}", bs58::encode(&block.blockhash).into_string());
    log::info!("- Parent Hash: {}", bs58::encode(&block.previous_blockhash).into_string());
    log::info!("- Timestamp: {}", block.block_time.as_ref().map_or("Unknown".to_string(), |bt| bt.timestamp.to_string()));
    log::info!("- Transactions: {}", block.transactions.len());
    log::info!("- Failed Transactions: {}", block.transactions.iter()
        .filter(|tx| tx.meta.as_ref().and_then(|m| m.err.as_ref()).is_some())
        .count());
    log::info!("- Total Instructions: {}", block.transactions.iter()
        .filter_map(|tx| tx.transaction.as_ref())
        .filter_map(|tx| tx.message.as_ref())
        .map(|msg| msg.instructions.len())
        .sum::<usize>());


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

    for (tx_idx, tx) in block.transactions.iter().enumerate() {
        // Get transaction signature for logging
        // Get raw signature bytes for detailed logging
        let raw_sig = tx.transaction.as_ref()
            .and_then(|t| t.signatures.first())
            .map(|sig| sig.as_slice())
            .unwrap_or(&[]);
            
        let tx_signature = bs58::encode(raw_sig).into_string();
        let is_target = is_target_transaction(raw_sig, block.slot);
        
        // Log signature details
        log::info!("Processing transaction {} with signature {} (target: {}, match: {})", 
            tx_idx, tx_signature, TARGET_TX_STR, is_target);
        
        // Log raw bytes comparison if this might be our target
        if is_target || tx_signature == TARGET_TX_STR {
            if let Ok(target_bytes) = bs58::decode(TARGET_TX_STR).into_vec() {
                log::info!("Raw signature comparison:");
                log::info!("Transaction: {:?}", raw_sig);
                log::info!("Target     : {:?}", target_bytes);
            }
        }
        
        if let Some(meta) = &tx.meta {
            // Log transaction metadata for debugging
            log::info!("Transaction {} metadata:", tx_idx);
            log::info!("- Signature: {}", tx_signature);
            log::info!("- Status: {}", if meta.err.is_some() { "Failed" } else { "Success" });
            if let Some(err) = &meta.err {
                log::info!("- Error: {:?}", err);
            }
            log::info!("- Fee: {} lamports", meta.fee);
            log::info!("- Token Balances:");
            log::info!("  * Pre: {} balances", meta.pre_token_balances.len());
            log::info!("  * Post: {} balances", meta.post_token_balances.len());
            log::info!("  * Changes: {} balances", meta.pre_token_balances.len().max(meta.post_token_balances.len()));
            log::info!("- Instructions:");
            log::info!("  * Main: {} instructions", tx.transaction.as_ref()
                .and_then(|t| t.message.as_ref())
                .map(|m| m.instructions.len())
                .unwrap_or(0));
            log::info!("  * Inner: {} groups with {} total instructions", 
                meta.inner_instructions.len(),
                meta.inner_instructions.iter()
                    .map(|group| group.instructions.len())
                    .sum::<usize>());
            
            if meta.err.is_some() {
                log::info!("Skipping failed transaction {}", tx_idx);
                continue;
            }

            // Log token balances if this is our target transaction
            if is_target_transaction(raw_sig, block.slot) {
                log::info!("Target transaction token balances:");
                for (idx, balance) in meta.pre_token_balances.iter().enumerate() {
                    log::info!("Pre balance {}: Mint: {}, Owner: {}, Amount: {:?}", 
                        idx,
                        bs58::encode(&balance.mint).into_string(),
                        bs58::encode(&balance.owner).into_string(),
                        balance.ui_token_amount.as_ref().map(|a| a.ui_amount)
                    );
                }
                for (idx, balance) in meta.post_token_balances.iter().enumerate() {
                    log::info!("Post balance {}: Mint: {}, Owner: {}, Amount: {:?}",
                        idx,
                        bs58::encode(&balance.mint).into_string(),
                        bs58::encode(&balance.owner).into_string(),
                        balance.ui_token_amount.as_ref().map(|a| a.ui_amount)
                    );
                }
            }

            // First check main instructions
            if let Some(transaction) = &tx.transaction {
                if let Some(message) = &transaction.message {
                    // Log all account keys if this is our target transaction
                    if is_target_transaction(raw_sig, block.slot) {
                        log::info!("Found target transaction! Logging all account keys:");
                        for (idx, key) in message.account_keys.iter().enumerate() {
                            log::info!("Account key {}: {}", idx, bs58::encode(key).into_string());
                        }
                    }

                    for (inst_idx, instruction) in message.instructions.iter().enumerate() {
                        if (instruction.program_id_index as usize) >= message.account_keys.len() {
                            log::info!("Invalid program_id_index {} for account_keys length {}", 
                                instruction.program_id_index, message.account_keys.len());
                            continue;
                        }

                            // Add bounds check
                            if (instruction.program_id_index as usize) >= message.account_keys.len() {
                                log::info!("Invalid program_id_index {} for account_keys length {}", 
                                    instruction.program_id_index, message.account_keys.len());
                                continue;
                            }
                            let program_id = &message.account_keys[instruction.program_id_index as usize];
                        let program_id_str = bs58::encode(program_id).into_string();
                        
                        // Log instruction details
                        log::info!(
                            "Main instruction {} - Program: {}, Data length: {}, Account keys: {:?}",
                            inst_idx,
                            program_id_str,
                            instruction.data.len(),
                            instruction.accounts.iter()
                                .filter(|&idx| (*idx as usize) < message.account_keys.len())
                                .map(|&idx| bs58::encode(&message.account_keys[idx as usize]).into_string())
                                .collect::<Vec<_>>()
                        );

                        // Log instruction data if this is our target transaction
                        if is_target_transaction(raw_sig, block.slot) {
                            // Log instruction data in hex for better analysis
                            log::info!(
                                "Target tx instruction {} data: 0x{}",
                                inst_idx,
                                hex::encode(&instruction.data)
                            );

                            // Check if this looks like a swap instruction
                            if instruction.data.len() >= 8 {
                                let method_id = &instruction.data[..8];
                                let method_id_hex = hex::encode(method_id);
                                log::info!(
                                    "Instruction method ID: 0x{} (data length: {})",
                                    method_id_hex,
                                    instruction.data.len()
                                );

                                let method_type = get_jupiter_method_type(method_id);
                                log::info!("Found Jupiter instruction: {}", method_type);
                            }
                        }

                        // Add debug logging for Jupiter program detection
                        if is_target_transaction(raw_sig, block.slot) {
                            let program_type = get_jupiter_program_type(&program_id_str);
                            
                            log::info!(
                                "Target tx instruction {} - Program: {} (Type: {})",
                                inst_idx,
                                program_id_str,
                                program_type
                            );
                        }

                        // Check if this is our target transaction
                        if is_target_transaction(raw_sig, block.slot) {
                            log::info!("Found our target transaction!");
                        }

                        // Check if this is a Jupiter AMM program
                        let is_jupiter_amm = JUPITER_AMMS.contains(&program_id_str.as_str());

                        // Also check if any account in the instruction references Jupiter AMMs
                        let has_jupiter_account = instruction.accounts.iter()
                            .filter(|&idx| (*idx as usize) < message.account_keys.len())
                            .any(|&idx| {
                                let account = bs58::encode(&message.account_keys[idx as usize]).into_string();
                                JUPITER_AMMS.contains(&account.as_str())
                            });

                        if is_jupiter_amm || has_jupiter_account {
                            log::info!("Found Jupiter program in main instruction:");
                            log::info!("- Program ID: {}", program_id_str);
                            log::info!("- Transaction: {}", tx_signature);
                            log::info!("- Detection: {} (AMM match: {})", 
                                if is_jupiter_amm { "Program ID" } else { "Account reference" },
                                JUPITER_AMMS.contains(&program_id_str.as_str()));
                            process_jupiter_instruction(tx, transaction, message, &program_id_str, &mut tables, &block)?;
                        }
                    }
                }
            }

            // Then check inner instructions
            for (group_idx, inner_inst_group) in meta.inner_instructions.iter().enumerate() {
                log::info!("Processing inner instruction group {} with {} instructions", 
                    group_idx, inner_inst_group.instructions.len());

                if let Some(transaction) = &tx.transaction {
                    if let Some(message) = &transaction.message {
                        for (inst_idx, inner_inst) in inner_inst_group.instructions.iter().enumerate() {
                            if (inner_inst.program_id_index as usize) >= message.account_keys.len() {
                                log::info!("Invalid program_id_index {} for account_keys length {}", 
                                    inner_inst.program_id_index, message.account_keys.len());
                                continue;
                            }

                            let program_id = &message.account_keys[inner_inst.program_id_index as usize];
                            let program_id_str = bs58::encode(program_id).into_string();
                            
                            // Log inner instruction details
                            log::info!(
                                "Inner instruction {}.{} - Program: {}, Data length: {}, Account keys: {:?}",
                                group_idx,
                                inst_idx,
                                program_id_str,
                                inner_inst.data.len(),
                                inner_inst.accounts.iter()
                                    .filter(|&idx| (*idx as usize) < message.account_keys.len())
                                    .map(|&idx| bs58::encode(&message.account_keys[idx as usize]).into_string())
                                    .collect::<Vec<_>>()
                            );

                            // Log inner instruction data if this is our target transaction
                            if is_target_transaction(raw_sig, block.slot) {
                                // Log inner instruction data in hex for better analysis
                                log::info!(
                                    "Target tx inner instruction {}.{} data: 0x{}",
                                    group_idx,
                                    inst_idx,
                                    hex::encode(&inner_inst.data)
                                );

                                // Check if this looks like a swap instruction
                                if inner_inst.data.len() >= 8 {
                                    let method_id = &inner_inst.data[..8];
                                    let method_id_hex = hex::encode(method_id);
                                    log::info!(
                                        "Inner instruction method ID: 0x{} (data length: {})",
                                        method_id_hex,
                                        inner_inst.data.len()
                                    );

                                    let method_type = get_jupiter_method_type(method_id);
                                    log::info!("Found Jupiter instruction: {}", method_type);
                                }
                            }

                            // Add debug logging for Jupiter program detection in inner instructions
                            if is_target_transaction(raw_sig, block.slot) {
                                let program_type = get_jupiter_program_type(&program_id_str);
                                
                                log::info!(
                                    "Target tx inner instruction {}.{} - Program: {} (Type: {})",
                                    group_idx,
                                    inst_idx,
                                    program_id_str,
                                    program_type
                                );
                            }
                
                            // Check if this is a Jupiter AMM program
                            let is_jupiter_amm = JUPITER_AMMS.contains(&program_id_str.as_str());

                            // Also check if any account in the instruction references Jupiter AMMs
                            let has_jupiter_account = inner_inst.accounts.iter()
                                .filter(|&idx| (*idx as usize) < message.account_keys.len())
                                .any(|&idx| {
                                    let account = bs58::encode(&message.account_keys[idx as usize]).into_string();
                                    JUPITER_AMMS.contains(&account.as_str())
                                });

                            if is_jupiter_amm || has_jupiter_account {
                                log::info!("Found Jupiter program in inner instruction:");
                                log::info!("- Program ID: {}", program_id_str);
                                log::info!("- Transaction: {}", tx_signature);
                                log::info!("- Group/Index: {}.{}", group_idx, inst_idx);
                                log::info!("- Detection: {} (AMM match: {})", 
                                    if is_jupiter_amm { "Program ID" } else { "Account reference" },
                                    JUPITER_AMMS.contains(&program_id_str.as_str()));
                                process_jupiter_instruction(tx, transaction, message, &program_id_str, &mut tables, &block)?;
                            }
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
        
        log::info!("Entity change:");
        log::info!("- Type: {}", change.entity);
        log::info!("- ID: {}", change.id);
        log::info!("- Operation: {}", if change.operation == 0 { "Create" } else { "Update" });
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
    let tx_id = bs58::encode(&transaction.signatures[0]).into_string();
    let swap_id = format!("swap-{}", tx_id);
    
    log::info!("Processing Jupiter transaction {} (checking if matches target: {})", swap_id, tx_id == TARGET_TX_STR);
    
    // Log which Jupiter program was found
    let program_type = get_jupiter_program_type(program_id_str);
    
    log::info!("Found Jupiter program: {} (Type: {})", program_id_str, program_type);
    
    // Create pool entity
    if let Some(meta) = &tx.meta {
        log::info!("Found {} post token balances for tx {}", meta.post_token_balances.len(), tx_id);
        
        // Log all token balances for debugging
        log::info!("All token balances for tx {}:", tx_id);
        log::info!("Pre token balances:");
        for (idx, balance) in meta.pre_token_balances.iter().enumerate() {
            log::info!("  {}: Mint: {}, Owner: {}, Amount: {:?}", 
                idx,
                bs58::encode(&balance.mint).into_string(),
                bs58::encode(&balance.owner).into_string(),
                balance.ui_token_amount.as_ref().map(|a| a.ui_amount)
            );
        }
        log::info!("Post token balances:");
        for (idx, balance) in meta.post_token_balances.iter().enumerate() {
            log::info!("  {}: Mint: {}, Owner: {}, Amount: {:?}",
                idx,
                bs58::encode(&balance.mint).into_string(),
                bs58::encode(&balance.owner).into_string(),
                balance.ui_token_amount.as_ref().map(|a| a.ui_amount)
            );
        }

        // Track all token balance changes
        let mut token_changes = Vec::new();
        
        // Track tokens that decreased (tokens spent)
        for pre_balance in meta.pre_token_balances.iter() {
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
            
            log::info!("Creating pool entity:");
            log::info!("- ID: {}", pool_id);
            log::info!("- Protocol: {}", program_id_str);
            log::info!("- Input Tokens: {:?}", vec![token_in.clone(), token_out.clone()]);
            log::info!("- Created At: Block {}, Timestamp {}", 
                block.slot,
                block.block_time.as_ref().map_or(0i64, |bt| bt.timestamp));

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

            log::info!("Creating swap entity:");
            log::info!("- ID: {}", swap_id);
            log::info!("- Block Hash: {}", bs58::encode(&block.blockhash).into_string());
            log::info!("- From: {}", if message.account_keys.len() > 0 {
                bs58::encode(&message.account_keys[0]).into_string()
            } else {
                "unknown".to_string()
            });
            log::info!("- Amount In: {}", (amount_in_pre - amount_in_post).abs());
            log::info!("- Amount Out: {}", (amount_out_post - amount_out_pre).abs());
            
            let swap = tables.create_row("Swap", &swap_id);
            swap.set("id", &swap_id);
            swap.set("blockHash", bs58::encode(&block.blockhash).into_string());
            swap.set("protocol", program_id_str);
            swap.set("pool", &pool_id);
            
            // Add bounds check for account keys
            let from_key = if message.account_keys.len() > 0 {
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
