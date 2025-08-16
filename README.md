# ZetaChain Solana Universal NFT Program

A comprehensive Solana program enabling cross-chain NFT transfers and interactions between ZetaChain and Solana, built for seamless universal NFT experiences.

## 🌟 Overview

This project implements a universal NFT program that enables:
- Cross-chain NFT minting and transfers between Solana and ZetaChain
- Integration with ZetaChain's protocol-contracts-solana gateway
- Robust security measures including TSS and replay protection
- Optimized handling of Solana-specific challenges (compute budget, rent exemption, token accounts)
- Full compatibility with Metaplex Token Metadata standard

## 🏗️ Architecture

### Core Components
- **Universal NFT Program**: Main Solana program for cross-chain NFT operations
- **Gateway Integration**: Seamless interaction with ZetaChain's gateway contracts
- **Metadata Management**: Full Metaplex integration for NFT metadata
- **Security Layer**: TSS verification and replay protection
- **Cross-Chain Bridge**: Burn-and-mint mechanism for chain transfers

### Key Features
- 🔄 **Cross-Chain Transfers**: Move NFTs between Solana, ZetaChain, and other connected chains
- 🛡️ **Security First**: TSS signature verification and nonce-based replay protection
- ⚡ **Optimized Performance**: Compute budget optimization and efficient account management
- 🎨 **Rich Metadata**: Full support for NFT metadata, attributes, and collections
- 📊 **Origin Tracking**: PDA-based tracking of NFT origins across chains
- 🔧 **Developer Friendly**: Comprehensive documentation and examples

## 🚀 Quick Start

### Prerequisites
- Rust 1.79+
- Solana CLI
- Anchor Framework
- Node.js 18+

### Installation
```bash
git clone <repository-url>
cd zetachain-solana-nft
npm install
anchor build
```

### Testing
```bash
# Run unit tests
anchor test

# Run devnet integration tests
npm run test:devnet
```

## 📖 Documentation

- [Architecture Overview](./docs/ARCHITECTURE.md)
- [API Reference](./docs/API.md)
- [Security Model](./docs/SECURITY.md)
- [Cross-Chain Guide](./docs/CROSS_CHAIN.md)
- [Developer Tutorials](./docs/TUTORIALS.md)

## 🛠️ Technical Specifications

### Token ID Generation
- Format: `[mint_pubkey + block_number + timestamp]`
- Ensures unique IDs across all chains
- Maintains consistency during transfers

### Cross-Chain Flow
1. **Outbound Transfer**: Burn NFT on source chain
2. **Gateway Processing**: Message routed through ZetaChain
3. **Destination Mint**: Recreate NFT with same ID and metadata

### Security Features
- ECDSA secp256k1 signature verification
- Nonce-based replay attack prevention
- Program Derived Address (PDA) validation
- Gateway program authentication

## 🌐 Supported Chains

- ✅ Solana (Devnet/Mainnet)
- ✅ ZetaChain (Testnet/Mainnet)  
- ✅ Ethereum (via ZetaChain hub)
- ✅ BNB Chain (via ZetaChain hub)

## 📁 Project Structure

```
zetachain-solana-nft/
├── programs/
│   └── universal-nft/          # Main Solana program
├── tests/                      # Test suites
├── app/                        # Frontend demo
├── docs/                       # Documentation
├── scripts/                    # Deployment scripts
└── examples/                   # Usage examples
```

## 🏆 Bounty Requirements Addressed

- ✅ Robust cross-chain NFT transfers
- ✅ ZetaChain gateway integration
- ✅ Solana-specific optimizations
- ✅ Security best practices
- ✅ Comprehensive documentation
- ✅ Developer onboarding resources
- ✅ Working demos and examples

## 🤝 Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines.

## 📄 License

MIT License - see [LICENSE](./LICENSE) for details.

## 🎯 About This Submission

This project was developed for the ZetaChain Solana Universal NFT bounty, implementing all required features and bonus components for a comprehensive cross-chain NFT solution.