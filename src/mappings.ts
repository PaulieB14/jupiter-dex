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

  // Log the data structure for debugging
  log.debug("Processing data object", []);

  // Safely get entityChanges
  if (!data) {
    log.debug("Data is null or undefined", []);
    return;
  }

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

  log.debug("Found " + entitiesArray.length.toString() + " entities", []);

  // Process each entity
  for (let i = 0; i < entitiesArray.length; i++) {
    const entity = entitiesArray[i];
    if (!entity) {
      log.debug("Entity at index " + i.toString() + " is null", []);
      continue;
    }

    const entityObj = entity.toObject();
    if (!entityObj) {
      log.debug("EntityObj at index " + i.toString() + " is null", []);
      continue;
    }

    // Check if it's a Swap entity
    const type = entityObj.get("type");
    if (!type) {
      log.debug("Entity at index " + i.toString() + " has no type", []);
      continue;
    }
    
    const typeStr = type.toString();
    if (typeStr != "Swap") {
      log.debug("Entity at index " + i.toString() + " is not a Swap: " + typeStr, []);
      continue;
    }

    // Get fields safely
    const fields = entityObj.get("fields");
    if (!fields) {
      log.debug("No fields found for entity at index " + i.toString(), []);
      continue;
    }

    const fieldsObj = fields.toObject();
    if (!fieldsObj) {
      log.debug("Invalid fields format for entity at index " + i.toString(), []);
      continue;
    }

    // Extract all required fields
    const requiredFields = [
      "id", "blockHash", "protocol", "from", "to", "slot",
      "blockNumber", "timestamp", "tokenIn", "amountIn",
      "tokenOut", "amountOut"
    ];
    
    const values = new Map<string, JSONValue>();
    let hasAllFields = true;
    let missingField = "";
    
    for (let j = 0; j < requiredFields.length; j++) {
      const field = requiredFields[j];
      const value = fieldsObj.get(field);
      if (!value) {
        hasAllFields = false;
        missingField = field;
        log.debug("Missing required field: " + field + " for entity at index " + i.toString(), []);
        break;
      }
      values.set(field, value);
    }
    
    if (!hasAllFields) {
      log.debug("Skipping entity at index " + i.toString() + " due to missing field: " + missingField, []);
      continue;
    }

    // Get values with safe fallbacks
    const idValue = values.get("id");
    const blockHashValue = values.get("blockHash");
    const protocolValue = values.get("protocol");
    const fromValue = values.get("from");
    const toValue = values.get("to");
    const slotValue = values.get("slot");
    const blockNumberValue = values.get("blockNumber");
    const timestampValue = values.get("timestamp");
    const tokenInValue = values.get("tokenIn");
    const amountInValue = values.get("amountIn");
    const tokenOutValue = values.get("tokenOut");
    const amountOutValue = values.get("amountOut");
    
    // Safely convert values with fallbacks
    const id = idValue ? idValue.toString() : "";
    const blockHash = blockHashValue ? blockHashValue.toString() : "";
    const protocol = protocolValue ? protocolValue.toString() : "";
    const from = fromValue ? fromValue.toString() : "";
    const to = toValue ? toValue.toString() : "";
    
    // Safely convert numeric values
    let slot = BigInt.zero();
    if (slotValue) {
      const slotStr = slotValue.toString();
      if (slotStr && slotStr.length > 0) {
        slot = BigInt.fromString(slotStr);
      }
    }
    
    let blockNumber = BigInt.zero();
    if (blockNumberValue) {
      const blockNumberStr = blockNumberValue.toString();
      if (blockNumberStr && blockNumberStr.length > 0) {
        blockNumber = BigInt.fromString(blockNumberStr);
      }
    }
    
    let timestamp = BigInt.zero();
    if (timestampValue) {
      const timestampStr = timestampValue.toString();
      if (timestampStr && timestampStr.length > 0) {
        timestamp = BigInt.fromString(timestampStr);
      }
    }
    
    const tokenIn = tokenInValue ? tokenInValue.toString() : "";
    const amountIn = amountInValue ? amountInValue.toString() : "0";
    const tokenOut = tokenOutValue ? tokenOutValue.toString() : "";
    const amountOut = amountOutValue ? amountOutValue.toString() : "0";
    
    // Create swap with safely extracted values
    createSwap(
      id,
      blockHash,
      protocol,
      from,
      to,
      slot,
      blockNumber,
      timestamp,
      tokenIn,
      amountIn,
      tokenOut,
      amountOut
    );
    
    log.debug("Successfully processed entity at index " + i.toString(), []);
  }
}
