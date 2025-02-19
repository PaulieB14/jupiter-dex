import { BigInt, BigDecimal, Bytes } from "@graphprotocol/graph-ts"
import { Protocol } from "../generated/schema"
import { Transactions } from "./pb/sf/substreams/solana/v1/Transactions"

// Jupiter contract addresses
const JUPITER_SWAP_ADDRESS = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
const JUPITER_LIMIT_ORDER_ADDRESS = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu";
const JUPITER_DCA_ADDRESS = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";

// Helper function to check if address is a Jupiter contract
function isJupiterContract(address: string): boolean {
  return address == JUPITER_SWAP_ADDRESS || 
         address == JUPITER_LIMIT_ORDER_ADDRESS || 
         address == JUPITER_DCA_ADDRESS;
}

// Helper function to safely get or create protocol
function getOrCreateProtocol(): Protocol {
  let protocol = Protocol.load("jupiter");
  if (!protocol) {
    protocol = new Protocol("jupiter");
    protocol.name = "Jupiter";
    protocol.version = "v6";
    protocol.totalVolumeUSD = BigDecimal.fromString("0");
    protocol.totalUniqueUsers = BigInt.fromI32(0);
    protocol.lastUpdateTimestamp = BigInt.fromI32(0);
    protocol.save();
  }
  return protocol;
}

// Helper function to safely process account keys
function processAccountKeys(accountKeys: Array<Uint8Array>): boolean {
  for (let i = 0; i < accountKeys.length; i++) {
    const key = accountKeys[i];
    if (!key) continue;

    // Convert bytes to string safely
    const keyBytes = Bytes.fromUint8Array(key);
    if (!keyBytes) continue;

    const address = keyBytes.toBase58();
    if (!address || address == "") continue;

    if (isJupiterContract(address)) {
      return true;
    }
  }
  return false;
}

// Helper function to safely process a transaction
function processTransaction(tx: Transactions): boolean {
  if (!tx || !tx.transactions) return false;

  for (let i = 0; i < tx.transactions.length; i++) {
    const transaction = tx.transactions[i];
    if (!transaction || !transaction.meta) continue;

    const txData = transaction.transaction;
    if (!txData) continue;

    const message = txData.message;
    if (!message) continue;

    const accountKeys = message.accountKeys;
    if (!accountKeys) continue;

    if (processAccountKeys(accountKeys)) {
      return true;
    }
  }
  return false;
}

export function handleTriggers(data: Transactions): void {
  // Get or create protocol first
  const protocol = getOrCreateProtocol();

  // Process transaction safely
  if (processTransaction(data)) {
    // Update protocol stats only if we found a Jupiter transaction
    const currentUsers = protocol.totalUniqueUsers;
    protocol.totalUniqueUsers = currentUsers.plus(BigInt.fromI32(1));
    protocol.lastUpdateTimestamp = BigInt.fromI32(0); // Use 0 as timestamp for now
    protocol.save();
  }
}
