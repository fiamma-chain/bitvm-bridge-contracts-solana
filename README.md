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

## Development Setup

1. Install dependencies
```bash
yarn install
```

2. Create `.env` file from template
```bash
cp .env.example .env
```

Update the following values in `.env`:
```plaintext
SOLANA_PRIVATE_KEY=your_base58_private_key
RPC_URL=https://api.devnet.solana.com
```

3. Build the program
```bash
anchor build
```

## Testing

### Local Testing
1. Start local validator with required programs
```bash
solana-test-validator \
  --clone metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s \
  --clone PwDiXFxQsGra4sFFTT8r1QWRMd4vfumiWC1jfWNfdYT \
  --url https://api.mainnet-beta.solana.com \
  --reset
```

2. Run tests
```bash
anchor test
```

## Deployment

### Local Deployment
```bash
anchor deploy
```

### Devnet Deployment
1. Configure Solana CLI for devnet
```bash
solana config set --url devnet
```

2. Get devnet SOL
```bash
solana airdrop 2
```

3. Deploy
```bash
anchor deploy
```

### Initialize Contract
After deployment, initialize the contract:
```bash
yarn initialize
```

### Program Upgrade
```bash
# Build program
anchor build

# Create upgrade buffer
solana program write-buffer ./target/deploy/bitvm_bridge.so

# Deploy upgrade
solana program deploy --buffer <BUFFER_ADDRESS> --program-id <PROGRAM_ID>
```

### Cleanup
Close deploy buffer account:
```bash
solana program close --buffers
```

## Contract Parameters

The bridge contract includes configurable parameters:
- `max_btc_per_mint`: Maximum BTC amount per mint
- `min_btc_per_mint`: Minimum BTC amount per mint
- `max_btc_per_burn`: Maximum BTC amount per burn
- `min_btc_per_burn`: Minimum BTC amount per burn
- `skip_tx_verification`: Enable/disable transaction verification

## Project Structure
```
├── programs/
│   ├── bitvm-bridge/       # Bridge program
│   └── btc-light-client/   # BTC Light Client program
├── tests/                  # Integration tests
├── cli/                    # CLI tools
│   └── src/
│       ├── commands/       # CLI commands
│       └── utils.ts        # Utility functions
└── Anchor.toml            # Project configuration
```

## License

ISC

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request
