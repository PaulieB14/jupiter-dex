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

  // Process transactions safely
  const txs = data.transactions;
  for (let i = 0; i < txs.length; i++) {
    const tx = txs[i];
    if (!tx) continue;

    const txData = tx.transaction;
    if (!txData) continue;

    const txMeta = tx.meta;
    if (!txMeta) continue;

    const msg = txData.message;
    if (!msg) continue;

    const keys = msg.accountKeys;
    if (!keys) continue;

    // Check if transaction involves Jupiter contracts
    for (let j = 0; j < keys.length; j++) {
      const key = keys[j];
      if (!key) continue;

      // Safely convert key to string
      const addr = key.toString();
      if (!addr || addr == "") continue;
      if (addr == JUPITER_SWAP_ADDRESS || 
          addr == JUPITER_LIMIT_ORDER_ADDRESS || 
          addr == JUPITER_DCA_ADDRESS) {
        
        // Update protocol stats
        protocol.totalUniqueUsers = protocol.totalUniqueUsers.plus(BigInt.fromI32(1))
        protocol.lastUpdateTimestamp = BigInt.fromI32(i)
        protocol.save()

        // TODO: Process market, token, and swap data once we have access to the full transaction data
      }
    }
  }
}
