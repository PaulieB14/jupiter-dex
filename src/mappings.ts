import { BigInt, BigDecimal } from "@graphprotocol/graph-ts"
import { Protocol, Market, Token, Swap } from "../generated/schema"
import { Transactions } from "./pb/sf/substreams/solana/v1/Transactions"

// Jupiter contract addresses
const JUPITER_SWAP_ADDRESS = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
const JUPITER_LIMIT_ORDER_ADDRESS = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu"
const JUPITER_DCA_ADDRESS = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"

export function handleTriggers(data: Transactions): void {
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

  // Process transactions
  for (let i = 0; i < data.transactions.length; i++) {
    const tx = data.transactions[i]
    if (!tx.transaction || !tx.meta) continue

    const transaction = tx.transaction!
    const message = transaction.message
    if (!message) continue

    // Check if transaction involves Jupiter contracts
    const accountKeys = message.accountKeys
    for (let j = 0; j < accountKeys.length; j++) {
      const accountKey = accountKeys[j]
      if (!accountKey) continue

      const address = accountKey.toString()
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
