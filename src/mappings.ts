import { BigInt, BigDecimal, Bytes } from "@graphprotocol/graph-ts"
import { Protocol } from "../generated/schema"
import { Transactions } from "./pb/sf/substreams/solana/v1/Transactions"

// Jupiter contract addresses as base58 strings
const JUPITER_SWAP_ADDRESS = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
const JUPITER_LIMIT_ORDER_ADDRESS = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu";
const JUPITER_DCA_ADDRESS = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";

// Helper function to check if address is a Jupiter contract
function isJupiterContract(address: string): boolean {
  return address == JUPITER_SWAP_ADDRESS || 
         address == JUPITER_LIMIT_ORDER_ADDRESS || 
         address == JUPITER_DCA_ADDRESS;
}

// Helper function to safely convert bytes to base58 string
function bytesToBase58(bytes: Uint8Array): string {
  if (bytes.length == 0) return "";
  return Bytes.fromUint8Array(bytes).toBase58();
}

// Helper function to get or create protocol
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
  let i = 0;
  while (i < accountKeys.length) {
    const bytes = accountKeys[i];
    if (bytes && bytes.length > 0) {
      const address = bytesToBase58(bytes);
      if (isJupiterContract(address)) {
        return true;
      }
    }
    i++;
  }
  return false;
}

export function handleTriggers(data: Transactions): void {
  if (!data || !data.transactions) return;

  const protocol = getOrCreateProtocol();
  let hasUpdates = false;

  let i = 0;
  while (i < data.transactions.length) {
    const tx = data.transactions[i];
    if (!tx || !tx.meta) {
      i++;
      continue;
    }

    const txData = tx.transaction;
    if (!txData) {
      i++;
      continue;
    }

    const message = txData.message;
    if (!message) {
      i++;
      continue;
    }

    const accountKeys = message.accountKeys;
    if (!accountKeys) {
      i++;
      continue;
    }

    // Process account keys safely
    if (processAccountKeys(accountKeys)) {
      hasUpdates = true;
      break;
    }

    i++;
  }

  // Update protocol stats if we found a Jupiter contract
  if (hasUpdates) {
    const currentUsers = protocol.totalUniqueUsers;
    protocol.totalUniqueUsers = currentUsers.plus(BigInt.fromI32(1));
    protocol.lastUpdateTimestamp = BigInt.fromI32(i);
    protocol.save();
  }
}
