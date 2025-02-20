import { BigInt, BigDecimal, Entity, store, TypedMap, JSONValue, Value, log } from "@graphprotocol/graph-ts";

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

function getOrCreateLiquidityPool(id: string, protocolId: string, inputTokens: string[]): void {
  if (!store.get("LiquidityPool", id)) {
    const pool = new Entity();
    pool.setString("id", id);
    pool.setString("protocol", protocolId);
    pool.set("inputTokens", Value.fromStringArray(inputTokens));
    pool.setBigInt("token0Balance", BigInt.zero());
    pool.setBigInt("token1Balance", BigInt.zero());
    pool.setBigInt("outputTokenSupply", BigInt.zero());
    pool.set("cumulativeVolumeByTokenAmount", Value.fromStringArray([BigInt.zero().toString(), BigInt.zero().toString()]));
    pool.setBigInt("createdTimestamp", BigInt.fromI32(0)); // We'll update this with the actual block timestamp
    pool.setBigInt("createdBlockNumber", BigInt.fromI32(0));
    store.set("LiquidityPool", id, pool);
  }
}

function createSwap(
  id: string,
  protocolId: string,
  poolId: string,
  from: string,
  to: string,
  tokenIn: string,
  tokenOut: string,
  amountIn: BigInt,
  amountOut: BigInt,
  slot: BigInt,
  blockNumber: BigInt,
  timestamp: BigInt,
  blockHash: string
): void {
  const swap = new Entity();
  swap.setString("id", id);
  swap.setString("protocol", protocolId);
  swap.setString("pool", poolId);
  swap.setString("from", from);
  swap.setString("to", to);
  swap.setString("tokenIn", tokenIn);
  swap.setString("tokenOut", tokenOut);
  swap.setBigInt("amountIn", amountIn);
  swap.setBigInt("amountOut", amountOut);
  swap.setBigInt("slot", slot);
  swap.setBigInt("blockNumber", blockNumber);
  swap.setBigInt("timestamp", timestamp);
  swap.setString("blockHash", blockHash);
  
  // For now, set USD amounts to 0 since we need price data
  swap.setString("amountInUSD", BigDecimal.zero().toString());
  swap.setString("amountOutUSD", BigDecimal.zero().toString());
  
  store.set("Swap", id, swap);
}

export function handleTriggers(data: TypedMap<string, JSONValue>): void {
  const changes = data.get("changes");
  if (!changes) return;

  // Initialize protocols
  getOrCreateProtocol(JUPITER_SWAP);
  getOrCreateProtocol(JUPITER_LIMIT_ORDER);
  getOrCreateProtocol(JUPITER_DCA);

  const changesArray = changes.toArray();
  for (let i = 0; i < changesArray.length; i++) {
    const change = changesArray[i].toObject();
    const entityType = change.get("entity_type");
    if (!entityType || entityType.toString() !== "Trade") continue;

    const trade = change.get("data");
    if (!trade) continue;

    const tradeObj = trade.toObject();
    const programId = tradeObj.get("program_id");
    if (!programId) continue;

    const programIdStr = programId.toString();
    if (![JUPITER_SWAP, JUPITER_LIMIT_ORDER, JUPITER_DCA].includes(programIdStr)) continue;

    // Extract trade data
    const id = tradeObj.get("id");
    const from = tradeObj.get("from");
    const to = tradeObj.get("to");
    const tokenIn = tradeObj.get("token_in");
    const tokenOut = tradeObj.get("token_out");
    const amountIn = tradeObj.get("amount_in");
    const amountOut = tradeObj.get("amount_out");
    const slot = tradeObj.get("slot");
    const blockNumber = tradeObj.get("block_number");
    const timestamp = tradeObj.get("timestamp");
    const blockHash = tradeObj.get("block_hash");

    if (!id || !from || !to || !tokenIn || !tokenOut || !amountIn || !amountOut || 
        !slot || !blockNumber || !timestamp || !blockHash) continue;

    // Create or get pool
    const poolId = `${programIdStr}-${tokenIn.toString()}-${tokenOut.toString()}`;
    getOrCreateLiquidityPool(
      poolId,
      programIdStr,
      [tokenIn.toString(), tokenOut.toString()]
    );

    // Create swap
    createSwap(
      `swap-${id.toString()}`,
      programIdStr,
      poolId,
      from.toString(),
      to.toString(),
      tokenIn.toString(),
      tokenOut.toString(),
      amountIn.toBigInt(),
      amountOut.toBigInt(),
      slot.toBigInt(),
      blockNumber.toBigInt(),
      timestamp.toBigInt(),
      blockHash.toString()
    );
  }
}
