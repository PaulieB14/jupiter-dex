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
  log.debug("Received data", []);

  // Initialize protocols
  getOrCreateProtocol(JUPITER_SWAP);
  getOrCreateProtocol(JUPITER_LIMIT_ORDER);
  getOrCreateProtocol(JUPITER_DCA);

  // Log that we're processing the data
  log.debug("Processing data", []);

  // Check if we have entityChanges
  const entityChanges = data.get("entityChanges");
  if (!entityChanges || !entityChanges.toObject()) {
    log.debug("No entityChanges found or invalid format", []);
    return;
  }
  log.debug("Found entityChanges", []);

  // Try to get entities
  const entityChangesObj = entityChanges.toObject();
  const entities = entityChangesObj.get("entities");
  if (!entities || !entities.toArray()) {
    log.debug("No entities found or invalid format", []);
    return;
  }
  log.debug("Found entities", []);

  const entitiesArray = entities.toArray();
  log.debug("Found {} entities", [entitiesArray.length.toString()]);

  for (let i = 0; i < entitiesArray.length; i++) {
    const entity = entitiesArray[i];
    if (!entity) {
      log.debug("Entity {} is null", [i.toString()]);
      continue;
    }

    if (!entity || !entity.toObject()) {
      log.debug("Entity {} is invalid", [i.toString()]);
      continue;
    }

    const entityObj = entity.toObject();
    const type = entityObj.get("type");
    if (!type) {
      log.debug("Type is null for entity {}", [i.toString()]);
      continue;
    }
    log.debug("Entity {} type: {}", [i.toString(), type.toString()]);

    if (type.toString() != "Swap") {
      log.debug("Skipping non-Swap entity {}", [i.toString()]);
      continue;
    }

    const fields = entityObj.get("fields");
    if (!fields) {
      log.debug("Fields is null for entity {}", [i.toString()]);
      continue;
    }

    if (!fields || !fields.toObject()) {
      log.debug("Fields is invalid for entity {}", [i.toString()]);
      continue;
    }

    const fieldsObj = fields.toObject();

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
        !timestamp || !tokenIn || !amountIn || !tokenOut || !amountOut) {
      log.debug("Missing required fields for entity {}", [i.toString()]);
      continue;
    }

    log.debug("Creating swap with id: {}", [id.toString()]);

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
