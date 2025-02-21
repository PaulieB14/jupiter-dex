import { BigInt, Entity, store, TypedMap, JSONValue, Value } from "@graphprotocol/graph-ts";

// Protocol addresses
const JUPITER_SWAP = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
const JUPITER_LIMIT_ORDER = "jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu";
const JUPITER_DCA = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";

function getOrCreateProtocol(id: string): string {
  if (!store.get("Protocol", id)) {
    const protocol = new Entity();
    protocol.setString("id", id);
    protocol.setBigInt("cumulativeUniqueUsers", BigInt.zero());
    protocol.setBigInt("totalPoolCount", BigInt.zero());
    store.set("Protocol", id, protocol);
  }
  return id;
}

function getOrCreateLiquidityPool(id: string, protocolId: string, inputTokens: string[]): string {
  if (!store.get("LiquidityPool", id)) {
    const pool = new Entity();
    pool.setString("id", id);
    pool.setString("protocol", protocolId);
    pool.set("inputTokens", Value.fromStringArray(inputTokens));
    pool.setBigInt("token0Balance", BigInt.zero());
    pool.setBigInt("token1Balance", BigInt.zero());
    pool.setBigInt("outputTokenSupply", BigInt.zero());
    pool.set("cumulativeVolumeByTokenAmount", Value.fromBigIntArray([BigInt.zero(), BigInt.zero()]));
    pool.setBigInt("createdTimestamp", BigInt.zero());
    pool.setBigInt("createdBlockNumber", BigInt.zero());
    store.set("LiquidityPool", id, pool);
  }
  return id;
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
  // Create or get protocol
  const protocolId = getOrCreateProtocol(protocol);

  // Create pool ID from protocol and token pair
  const poolId = protocol + "-" + tokenIn + "-" + tokenOut;
  const poolEntityId = getOrCreateLiquidityPool(poolId, protocolId, [tokenIn, tokenOut]);

  const swap = new Entity();
  swap.setString("id", id);
  swap.setString("blockHash", blockHash);
  swap.setString("protocol", protocolId);
  swap.setString("pool", poolEntityId);
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
