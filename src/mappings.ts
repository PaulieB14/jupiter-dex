import { BigInt, Entity, store, TypedMap, JSONValue, Value, log } from "@graphprotocol/graph-ts";

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
    
    // Convert string array to Value array
    const inputTokenValues: Value[] = [];
    for (let i = 0; i < inputTokens.length; i++) {
      inputTokenValues.push(Value.fromString(inputTokens[i]));
    }
    pool.set("inputTokens", Value.fromArray(inputTokenValues));
    
    pool.setBigInt("token0Balance", BigInt.zero());
    pool.setBigInt("token1Balance", BigInt.zero());
    pool.setBigInt("outputTokenSupply", BigInt.zero());
    
    // Convert string array to Value array for volume
    const volumeValues: Value[] = [];
    volumeValues.push(Value.fromString("0"));
    volumeValues.push(Value.fromString("0"));
    pool.set("cumulativeVolumeByTokenAmount", Value.fromArray(volumeValues));
    
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

export function handleTriggers(data: TypedMap<string, JSONValue>): void {
  // Initialize protocols only once
  const protocols = [JUPITER_SWAP, JUPITER_LIMIT_ORDER, JUPITER_DCA];
  for (let i = 0; i < protocols.length; i++) {
    getOrCreateProtocol(protocols[i]);
  }

  // Safely get entityChanges
  const entityChanges = data.get("entityChanges");
  if (!entityChanges) {
    log.debug("No entityChanges found", []);
    return;
  }
  
  const entityChangesObj = entityChanges.toObject();
  if (!entityChangesObj) {
    log.debug("Invalid entityChanges format", []);
    return;
  }

  const entities = entityChangesObj.get("entities");
  if (!entities) {
    log.debug("No entities found", []);
    return;
  }

  const entitiesArray = entities.toArray();
  if (!entitiesArray) {
    log.debug("Invalid entities format", []);
    return;
  }

  // Process each entity
  for (let i = 0; i < entitiesArray.length; i++) {
    const entity = entitiesArray[i];
    if (!entity) continue;

    const entityObj = entity.toObject();
    if (!entityObj) continue;

    // Check if it's a Swap entity
    const type = entityObj.get("type");
    if (!type || type.toString() != "Swap") continue;

    // Get fields safely
    const fields = entityObj.get("fields");
    if (!fields) continue;

    const fieldsObj = fields.toObject();
    if (!fieldsObj) continue;

    // Extract all required fields
    const requiredFields = [
      "id", "blockHash", "protocol", "from", "to", "slot",
      "blockNumber", "timestamp", "tokenIn", "amountIn",
      "tokenOut", "amountOut"
    ];
    
    const values = new Map<string, JSONValue>();
    let hasAllFields = true;
    
    for (let j = 0; j < requiredFields.length; j++) {
      const field = requiredFields[j];
      const value = fieldsObj.get(field);
      if (!value) {
        hasAllFields = false;
        break;
      }
      values.set(field, value);
    }
    
    if (!hasAllFields) continue;

    // Create swap with safely extracted values
    createSwap(
      values.get("id")!.toString(),
      values.get("blockHash")!.toString(),
      values.get("protocol")!.toString(),
      values.get("from")!.toString(),
      values.get("to")!.toString(),
      BigInt.fromString(values.get("slot")!.toString()),
      BigInt.fromString(values.get("blockNumber")!.toString()),
      BigInt.fromString(values.get("timestamp")!.toString()),
      values.get("tokenIn")!.toString(),
      values.get("amountIn")!.toString(),
      values.get("tokenOut")!.toString(),
      values.get("amountOut")!.toString()
    );
  }
}
