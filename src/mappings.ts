import { BigInt, BigDecimal } from "@graphprotocol/graph-ts"
import { Protocol, Market, Token, Swap } from "../generated/schema"
import { Transactions } from "./pb/sf/substreams/solana/v1/Transactions"

// Jupiter contract addresses
const JUPITER_SWAP_ADDRESS = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
const JUPITER_LIMIT_ORDER_ADDRESS = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu"
const JUPITER_DCA_ADDRESS = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"

export function handleTriggers(data: Transactions): void {
  if (!data || !data.transactions) return;

  // Initialize Protocol if it doesn't exist
  let protocol = Protocol.load("jupiter")
  if (!protocol) {
    protocol = new Protocol("jupiter")
    protocol.name = "Jupiter"
    protocol.version = "v6"
    protocol.totalVolumeUSD = BigDecimal.fromString("0")
    protocol.totalUniqueUsers = BigInt.fromI32(0)
    protocol.lastUpdateTimestamp = BigInt.fromI32(0)
    protocol.save()
  }

  // Process transactions with improved safety and efficiency
  const transactions = data.transactions;
  if (!transactions || transactions.length === 0) return;

  for (let i = 0; i < transactions.length; i++) {
    const tx = transactions[i];
    if (!tx || !tx.meta) continue;

    const txData = tx.transaction;
    if (!txData || !txData.message) continue;

    const message = txData.message;
    if (!message) continue;

    // Safely access accountKeys after message validation
    const accountKeys = message.accountKeys;
    if (!accountKeys || accountKeys.length === 0) continue;

    // Check if transaction involves Jupiter contracts
    for (let j = 0; j < accountKeys.length; j++) {
      const account = accountKeys[j];
      if (!account) continue;

      // Get account address, using direct toString() since we already validated account
      const address = account.toString();
      if (address == "") continue;

      // Check if this is a Jupiter contract
      if (address == JUPITER_SWAP_ADDRESS || 
          address == JUPITER_LIMIT_ORDER_ADDRESS || 
          address == JUPITER_DCA_ADDRESS) {
        
        // Update protocol stats
        protocol.totalUniqueUsers = protocol.totalUniqueUsers.plus(BigInt.fromI32(1))
        protocol.lastUpdateTimestamp = BigInt.fromI32(i)
        protocol.save()

        // TODO: Process market, token, and swap data once we have access to the full transaction data
      }
    }
  }
}
