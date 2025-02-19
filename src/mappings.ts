import { BigInt, BigDecimal, TypedMap, Value, JSONValue, Bytes } from "@graphprotocol/graph-ts"
import { Protocol, Market, Token, Swap } from "../generated/schema"
import { Transactions } from "./pb/sf/substreams/solana/v1/Transactions"

// Jupiter contract addresses
const JUPITER_SWAP_ADDRESS = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
const JUPITER_LIMIT_ORDER_ADDRESS = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu"
const JUPITER_DCA_ADDRESS = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"

// Helper function to safely get string from potentially null bytes
function safeToString(value: Bytes | null): string {
  if (!value) return "";
  return value.toHexString();
}

// Helper function to check if address is a Jupiter contract
function isJupiterContract(address: string): boolean {
  return address == JUPITER_SWAP_ADDRESS || 
         address == JUPITER_LIMIT_ORDER_ADDRESS || 
         address == JUPITER_DCA_ADDRESS;
}

export function handleTriggers(data: Transactions): void {
  if (!data) return;

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

  // Safely handle transactions array
  const transactions = data.transactions;
  if (!transactions) return;

  for (let i = 0; i < transactions.length; i++) {
    const tx = transactions[i];
    if (!tx || !tx.meta) continue;

    const txData = tx.transaction;
    if (!txData || !txData.message) continue;

    const message = txData.message;
    if (!message) continue;

    // Handle account keys as nullable array
    const keys = message.accountKeys;
    if (!keys) continue;

    let foundJupiterContract = false;
    
    // Process each key, treating them as potentially null bytes
    for (let j = 0; j < keys.length; j++) {
      const key = keys[j] as Bytes | null;
      
      // Convert key to string safely using helper function
      const address = safeToString(key);
      if (address == "") continue;

      // Check if this is a Jupiter contract using helper function
      if (isJupiterContract(address)) {
        foundJupiterContract = true;
        break;
      }
    }

    // Only update protocol stats if we found a Jupiter contract
    if (foundJupiterContract) {
      const currentUsers = protocol.totalUniqueUsers;
      protocol.totalUniqueUsers = currentUsers.plus(BigInt.fromI32(1));
      protocol.lastUpdateTimestamp = BigInt.fromI32(i);
      protocol.save();
    }
  }
}
