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
  if (!entities) return;

  const entitiesArray = entities.toArray();
  for (let i = 0; i < entitiesArray.length; i++) {
    const entity = entitiesArray[i];
    if (!entity) continue;

    const entityObj = entity.toObject();
    if (!entityObj) continue;

    const type = entityObj.get("type");
    if (!type || type.toString() != "Swap") continue;

    const fields = entityObj.get("fields");
    if (!fields) continue;

    const fieldsObj = fields.toObject();
    if (!fieldsObj) continue;

    const id = fieldsObj.get("id");
    const blockHash = fieldsObj.get("blockHash");
    const protocol = fieldsObj.get("protocol");
    const from = fieldsObj.get("from");
    const to = fieldsObj.get("to");
    const slot = fieldsObj.get("slot");
    const blockNumber = fieldsObj.get("blockNumber");
    const timestamp = fieldsObj.get("timestamp");
    const tokenIn = fieldsObj.get("tokenIn");
    const amountIn = fieldsObj.get("amountIn");
    const tokenOut = fieldsObj.get("tokenOut");
    const amountOut = fieldsObj.get("amountOut");

    if (!id || !blockHash || !protocol || !from || !to || !slot || !blockNumber || 
        !timestamp || !tokenIn || !amountIn || !tokenOut || !amountOut) continue;

    createSwap(
      id.toString(),
      blockHash.toString(),
      protocol.toString(),
      from.toString(),
      to.toString(),
      BigInt.fromString(slot.toString()),
      BigInt.fromString(blockNumber.toString()),
      BigInt.fromString(timestamp.toString()),
      tokenIn.toString(),
      amountIn.toString(),
      tokenOut.toString(),
      amountOut.toString()
    );
  }
}
