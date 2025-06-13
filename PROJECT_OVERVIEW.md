# 🚀 **BTCZS LAYER 2 - BITCOINZ PROOF OF TRANSFER**

> **⚠️ REPOSITORY RESTRUCTURED**: This repository has been restructured for optimal development. The main BTCZS implementation is now at the root level for easier access and contribution.

> A native Layer 2 solution for BitcoinZ, enabling smart contracts and DeFi through Proof of Transfer (PoX)

## 🚀 Project Overview

BTCZS (BitcoinZ Stacks) is a layer 2 blockchain that brings smart contract functionality to BitcoinZ without modifying the base layer. Built by forking and adapting the Stacks blockchain architecture, BTCZS enables:

- **Smart Contracts** via Clarity language with BTCZ integration
- **DeFi Applications** with native BTCZ support
- **sBTCZ Peg** for seamless BTCZ ↔ sBTCZ conversion
- **Proof of Transfer** consensus secured by BitcoinZ

## 📊 Current Status

**Phase**: Foundation & Setup (Week 1 of 24)  
**Progress**: 5% Complete  
**Target Launch**: July 2025  

```
Phase 1: Foundation & Setup     [██░░░░░░░░] 20%
Phase 2: Core Protocol          [░░░░░░░░░░]  0%
Phase 3: Token Economics        [░░░░░░░░░░]  0%
Phase 4: Development Tools      [░░░░░░░░░░]  0%
Phase 5: Testing & Security     [░░░░░░░░░░]  0%
Phase 6: Launch Preparation     [░░░░░░░░░░]  0%
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    BTCZS Layer 2                            │
├─────────────────────────────────────────────────────────────┤
│  Smart Contracts (Clarity)  │  sBTCZ Peg  │  DeFi Apps     │
├─────────────────────────────────────────────────────────────┤
│  BTCZS Consensus (PoX)      │  Block Production & Validation │
├─────────────────────────────────────────────────────────────┤
│  BitcoinZ Integration Layer │  RPC Client │  Block Indexer  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    BitcoinZ Layer 1                         │
│  Burn Transactions  │  Block Headers  │  Security Model     │
└─────────────────────────────────────────────────────────────┘
```

## 🎯 Key Features

### 🔗 BitcoinZ Integration
- **Native BTCZ Support**: Direct integration with BitcoinZ blockchain
- **Burn-to-Mine**: Proof of Transfer consensus using BTCZ
- **Block Anchoring**: Every BTCZS block anchored to BTCZ
- **State Reading**: Smart contracts can read BTCZ state

### 💰 Token Economics
- **BTCZS Token**: 21M total supply, halving every 4 years
- **sBTCZ Peg**: 1:1 pegged BitcoinZ for DeFi use
- **Mining Rewards**: 1000 BTCZS initial block reward
- **Gas Fees**: Payable in BTCZS or sBTCZ

### 🔒 Security Model
- **Bitcoin-level Security**: Inherits BTCZ PoW security
- **Threshold Signatures**: Multi-sig peg protection
- **Formal Verification**: Clarity smart contracts
- **Audit Ready**: Security-first development

## 📁 Repository Structure

```
btczs-core/                 # Main blockchain implementation
├── src/
│   ├── burnchains/bitcoinz/ # BTCZ integration layer
│   ├── chainstate/stacks/   # BTCZS consensus
│   └── clarity/             # Smart contract engine
├── clarity/contracts/sbtcz/ # sBTCZ peg contracts
└── testnet/                 # Testing infrastructure

docs/                       # Project documentation
├── PROJECT_OVERVIEW.md     # High-level project info
├── PHASE_TRACKER.md        # Development progress
├── TECHNICAL_SPECIFICATIONS.md # Technical details
├── DEVELOPMENT_ROADMAP.md  # Timeline and milestones
└── STATUS_TRACKER.md       # Current status
```

## 🛠️ Development Setup

### Prerequisites
- Rust 1.70+
- Node.js 18+
- BitcoinZ Core node
- Docker (optional)

### Quick Start
```bash
# Clone the repository (when available)
git clone https://github.com/your-org/btczs-core
cd btczs-core

# Install dependencies
cargo build

# Run tests
cargo test

# Start development node
cargo run -- start --config testnet.toml
```

## 📋 Development Phases

### Phase 1: Foundation & Setup (Week 1-2) ✅ 20%
- [x] Project documentation
- [ ] Repository setup
- [ ] Development environment
- [ ] BTCZ integration research

### Phase 2: Core Protocol (Week 3-8) ⏳ 0%
- [ ] Burnchain abstraction layer
- [ ] RPC client adaptation
- [ ] Proof of Transfer modification
- [ ] Block production & validation

### Phase 3: Token Economics (Week 9-12) ⏸️ 0%
- [ ] BTCZS token implementation
- [ ] sBTCZ peg design
- [ ] Signer network
- [ ] Economic model validation

### Phase 4: Development Tools (Week 13-16) ⏸️ 0%
- [ ] CLI tools
- [ ] Blockchain API
- [ ] Block explorer
- [ ] Wallet SDK

### Phase 5: Testing & Security (Week 17-20) ⏸️ 0%
- [ ] Comprehensive testing
- [ ] Testnet deployment
- [ ] Security audit
- [ ] Performance optimization

### Phase 6: Launch Preparation (Week 21-24) ⏸️ 0%
- [ ] Documentation completion
- [ ] Community preparation
- [ ] Mainnet deployment
- [ ] Launch coordination

## 🎯 Milestones

- **Week 2**: Development environment ready
- **Week 8**: Core protocol complete
- **Week 12**: Token economics implemented
- **Week 16**: Development tools ready
- **Week 20**: Testing complete
- **Week 24**: Mainnet launch

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines:

1. **Fork** the repository
2. **Create** a feature branch
3. **Commit** your changes
4. **Push** to the branch
5. **Create** a Pull Request

### Development Guidelines
- Follow Rust best practices
- Write comprehensive tests
- Update documentation
- Follow security guidelines

## 📚 Documentation

- [Project Overview](PROJECT_OVERVIEW.md) - High-level project information
- [Technical Specifications](TECHNICAL_SPECIFICATIONS.md) - Detailed technical specs
- [Development Roadmap](DEVELOPMENT_ROADMAP.md) - Timeline and planning
- [Phase Tracker](PHASE_TRACKER.md) - Development progress
- [Status Tracker](STATUS_TRACKER.md) - Current status

## 🔗 Resources

### Related Projects
- [Stacks Core](https://github.com/stacks-network/stacks-core) - Original Stacks implementation
- [BitcoinZ](https://github.com/btcz/bitcoinz) - BitcoinZ blockchain
- [Clarity](https://clarity-lang.org/) - Smart contract language

### Documentation
- [Stacks Documentation](https://docs.stacks.co/)
- [BitcoinZ Documentation](https://getbtcz.com/)
- [Rust Documentation](https://doc.rust-lang.org/)

## 📞 Contact

- **Project Lead**: [Your Name]
- **Email**: [your-email@domain.com]
- **Discord**: [Discord Server]
- **Twitter**: [@btczs_official]

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This project is in active development. Use at your own risk. Not suitable for production use until mainnet launch.

---

**Built with ❤️ for the BitcoinZ community**
