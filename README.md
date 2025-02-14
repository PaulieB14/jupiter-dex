# Jupiter DEX Subgraph

This subgraph indexes Jupiter DEX activity on Solana using Substreams technology.

## Architecture

### Substreams Modules

The subgraph consumes data from two main Substreams modules:

1. `map_block_before_lookup` module:
   - Input: `sf.solana.type.v1.Block`
   - Output: `sf.solana.dex.trades.v1.Output`
   - Purpose: Initial block processing before address lookups
   - Hash: `ec16ccfb31e15db4f5ada823c9d3dfb54f1c001f`

2. `map_block` module:
   - Input: `sf.solana.type.v1.Block`
   - Output: `sf.solana.dex.trades.v1.Output`
   - Purpose: Main block processing with resolved addresses
   - Hash: `d5c2da3df75affd153d111cedacdaeb93d8f2735`

The modules process Solana blocks to extract:
- Jupiter DEX trades/swaps
- Pool creations and updates
- Token transfers and price data

### Data Flow

1. Solana Block → Substreams Modules
   - Raw blocks are processed to identify Jupiter DEX transactions
   - Addresses are resolved and transaction data is extracted
   - Trade information is formatted into the `sf.solana.dex.trades.v1.Output` structure

2. Substreams → Subgraph Entities
   - The subgraph consumes the processed data via `map_filtered_transactions`
   - Data is transformed into the following entities:
     - Protocol: Overall Jupiter DEX statistics
     - Market: Trading pair information and metrics
     - Token: Individual token data and volumes
     - Swap: Individual trade/swap events

## Schema

### Protocol
Tracks overall protocol metrics:
- Total volume
- Unique users
- Version information

### Market
Represents trading pairs:
- Token pair information
- Volume statistics
- Swap counts

### Token
Individual token data:
- Symbol and name
- Decimals
- Trading volume

### Swap
Individual trade events:
- Input/output amounts
- USD values
- User information
- Timestamps

## Example Queries

```graphql
{
  # Get protocol stats
  protocol(id: "jupiter") {
    totalVolumeUSD
    totalUniqueUsers
  }
  
  # Get top markets
  markets(
    first: 5
    orderBy: volumeUSD
    orderDirection: desc
  ) {
    name
    volumeUSD
    swapCount
  }
}
```

## Development

The subgraph uses the Solana DEX Trades Substreams package:
```yaml
source:
  package:
    moduleName: map_block
    file: https://substreams.dev/packages/tl-solana-dex-trades-1-0-13/v1.0.13
```

This package provides comprehensive DEX trade tracking on Solana, with two main modules:
- `map_block_before_lookup`: Initial processing of trades before address resolution
- `map_block`: Full trade processing with resolved addresses

We use the `map_block` module as it provides complete trade information with resolved addresses.

To rebuild the subgraph:
```bash
graph codegen
graph build
```

To deploy:
```bash
graph deploy --studio jupiter-dex
```
