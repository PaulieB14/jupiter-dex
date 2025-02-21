import { BigInt, BigDecimal, Entity, store } from "@graphprotocol/graph-ts";

// Protocol addresses
const JUPITER_SWAP = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
const JUPITER_LIMIT_ORDER = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu";
const JUPITER_DCA = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";

function getOrCreateProtocol(id: string): void {
  if (!store.get("Protocol", id)) {
    const protocol = new Entity();
    protocol.setString("id", id);
    protocol.setBigInt("cumulativeUniqueUsers", BigInt.zero());
    protocol.setBigInt("totalPoolCount", BigInt.zero());
    store.set("Protocol", id, protocol);
  }
}

function createSwap(
  id: string,
  blockHash: string,
  protocol: string,
  from: string,
  to: string,
  slot: BigInt,
  blockNumber: BigInt,
  timestamp: BigInt,
  tokenIn: string,
  amountIn: string,
  tokenOut: string,
  amountOut: string
): void {
  const swap = new Entity();
  swap.setString("id", id);
  swap.setString("blockHash", blockHash);
  swap.setString("protocol", protocol);
  swap.setString("from", from);
  swap.setString("to", to);
  swap.setBigInt("slot", slot);
  swap.setBigInt("blockNumber", blockNumber);
  swap.setBigInt("timestamp", timestamp);
  swap.setString("tokenIn", tokenIn);
  swap.setBigInt("amountIn", BigInt.fromString(amountIn));
  swap.setString("tokenOut", tokenOut);
  swap.setBigInt("amountOut", BigInt.fromString(amountOut));
  store.set("Swap", id, swap);
}

export function handleTriggers(entityChanges: any): void {
  // Initialize protocols
  getOrCreateProtocol(JUPITER_SWAP);
  getOrCreateProtocol(JUPITER_LIMIT_ORDER);
  getOrCreateProtocol(JUPITER_DCA);

  // Handle swaps
  if (entityChanges.entities) {
    for (let i = 0; i < entityChanges.entities.length; i++) {
      const entity = entityChanges.entities[i];
      if (entity.type == "Swap") {
        const fields = entity.fields;
        createSwap(
          fields.id,
          fields.blockHash,
          fields.protocol,
          fields.from,
          fields.to,
          BigInt.fromString(fields.slot.toString()),
          BigInt.fromString(fields.blockNumber.toString()),
          BigInt.fromString(fields.timestamp.toString()),
          fields.tokenIn,
          fields.amountIn,
          fields.tokenOut,
          fields.amountOut
        );
      }
    }
  }
}
