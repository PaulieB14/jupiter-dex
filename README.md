# Jupiter DEX Subgraph

This subgraph indexes Jupiter DEX data from Solana, tracking protocol metrics, markets, tokens, and swaps. It uses Substreams technology for efficient data processing and The Graph for indexing.

## Overview

The subgraph tracks the following Jupiter contracts:
- Jupiter Swap: `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4`
- Jupiter Limit Order: `jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu`
- Jupiter DCA: `DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M`

## Schema

### Protocol
Tracks global protocol metrics:
- Total volume in USD
- Total unique users
- Last update timestamp

### Market
Tracks individual trading pairs:
- Base and quote tokens
- TVL
- 24h volume
- Swap count

### Token
Stores token information:
- Name and symbol
- Decimals
- Total supply
- Associated markets (derived)

### Swap
Records individual swap transactions:
- Market reference
- Amounts (in/out)
- USD values
- Trader address
- Transaction hash

## Technical Stack

### Dependencies
- `@graphprotocol/graph-cli`: The Graph CLI for building and deploying subgraphs
- `@graphprotocol/graph-ts`: TypeScript types for subgraph development
- `as-proto`: AssemblyScript Protobuf implementation for Substreams

### Substreams Package
We utilize the [Solana DEX Trades Substreams package](https://substreams.dev/packages/tl-solana-dex-trades-1-0-13/v1.0.13) which provides:
- Block data processing
- Transaction filtering
- SPL token event handling

## Development

### Prerequisites
1. Node.js and npm
2. The Graph CLI: `npm install -g @graphprotocol/graph-cli`
3. A Graph Studio account and deployment key

### Setup
1. Install dependencies:
```bash
npm install
```

2. Generate AssemblyScript types:
```bash
npm run codegen
```

3. Build the subgraph:
```bash
npm run build
```

### Deployment
1. Authenticate with Graph Studio:
```bash
graph auth <your-deploy-key>
```

2. Deploy the subgraph:
```bash
graph deploy --node https://api.studio.thegraph.com/deploy/ --version-label v1.0.0 jupiter-dex
```

## Querying

The subgraph is deployed at: https://api.studio.thegraph.com/query/92142/jupiter-dex/v1.0.3

Example query:
```graphql
{
  protocol(id: "jupiter") {
    totalVolumeUSD
    totalUniqueUsers
  }
  markets(first: 5) {
    id
    tvl
    volume24h
    swapCount
  }
}
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
