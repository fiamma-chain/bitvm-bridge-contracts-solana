# BitVM Bridge - Solana Smart Contract

A Solana smart contract for managing BTC cross-chain assets through BitVM protocol.

## Overview

The BitVM Bridge smart contract enables:
- Minting wrapped BTC tokens on Solana
- Burning tokens for BTC redemption
- Configurable parameters for pegin/pegout amounts
- Admin controls for emergency pause and parameter updates

## Prerequisites

- Rust 1.70.0+
- Solana CLI 1.17.0+
- Node.js 18.0.0+
- Anchor Framework 0.30.1+
- Yarn

## Quick Start

1. Install dependencies
```bash
yarn install
```

2. Build the program
```bash
anchor build
```

3. Run tests
```bash
anchor test
```

## Development Setup

1. Configure Solana CLI
```bash
solana config set --url localhost
```

2. Start local validator
   
```bash
solana-test-validator --clone metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s --clone PwDiXFxQsGra4sFFTT8r1QWRMd4vfumiWC1jfWNfdYT --url https://api.mainnet-beta.solana.com --reset
```

1. Deploy locally
```bash
anchor deploy
```

## Testing

The test suite includes:
- Token initialization
- Minting operations
- Burning operations
- Admin functions
- Parameter updates

Run tests with:
```bash
anchor test
```

## Deployment

### Local Testnet
```bash
anchor deploy
```

### Devnet
```bash
solana config set --url devnet
solana airdrop 2 # Get test SOL
anchor deploy
```

### Program Upgrade
```bash
anchor build
solana program write-buffer ./target/deploy/bitvm_bridge.so
solana program deploy --buffer <BUFFER_ADDRESS> --program-id <PROGRAM_ID>
```

### Close Deploy Buffer account
```bash
solana program close --buffers 
```

## Contract Parameters

The bridge contract includes configurable parameters:
- `max_btc_per_mint`: Maximum BTC amount per mint
- `min_btc_per_mint`: Minimum BTC amount per mint
- `max_btc_per_burn`: Maximum BTC amount per burn
- `min_btc_per_burn`: Minimum BTC amount per burn

## Security Features

- Owner-only admin functions
- Configurable burn pause mechanism
- Amount limits for minting and burning
- Proper decimal handling for BTC amounts

## Project Structure

```
├── programs/
│   └── bitvm-bridge/
│       ├── src/
│       │   ├── instructions/    # Contract instructions
│       │   ├── error.rs        # Error definitions
│       │   └── lib.rs          # Program entrypoint
├── tests/                      # Integration tests
└── Anchor.toml                 # Project configuration
```

## License

ISC

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request
