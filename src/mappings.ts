import { BigInt, Entity, store, TypedMap, JSONValue } from "@graphprotocol/graph-ts";

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

export function handleTriggers(entityChanges: TypedMap<string, JSONValue>): void {
  // Initialize protocols
  getOrCreateProtocol(JUPITER_SWAP);
  getOrCreateProtocol(JUPITER_LIMIT_ORDER);
  getOrCreateProtocol(JUPITER_DCA);

  // Handle swaps
  const entities = entityChanges.get("entities");
  if (entities) {
    const entitiesArray = entities.toArray();
    for (let i = 0; i < entitiesArray.length; i++) {
      const entity = entitiesArray[i].toObject();
      const type = entity.get("type");
      if (type && type.toString() == "Swap") {
        const fields = entity.get("fields");
        if (!fields) continue;
        
        const fieldsObj = fields.toObject();
        createSwap(
          fieldsObj.get("id")!.toString(),
          fieldsObj.get("blockHash")!.toString(),
          fieldsObj.get("protocol")!.toString(),
          fieldsObj.get("from")!.toString(),
          fieldsObj.get("to")!.toString(),
          BigInt.fromString(fieldsObj.get("slot")!.toString()),
          BigInt.fromString(fieldsObj.get("blockNumber")!.toString()),
          BigInt.fromString(fieldsObj.get("timestamp")!.toString()),
          fieldsObj.get("tokenIn")!.toString(),
          fieldsObj.get("amountIn")!.toString(),
          fieldsObj.get("tokenOut")!.toString(),
          fieldsObj.get("amountOut")!.toString()
        );
      }
    }
  }
}
