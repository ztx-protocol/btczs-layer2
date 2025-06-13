# 🚀 **BTCZS - BitcoinZ Layer 2 with Proof of Transfer**

[![Build Status](https://github.com/ztx-protocol/btczs-layer2/workflows/CI/badge.svg)](https://github.com/ztx-protocol/btczs-layer2/actions)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![BitcoinZ](https://img.shields.io/badge/BitcoinZ-Layer%202-orange.svg)](https://btcz.rocks)

> **Native Layer 2 solution for BitcoinZ enabling smart contracts, DeFi, and scalable applications through Proof of Transfer (PoX)**

## 🎯 **What is BTCZS?**

BTCZS (BitcoinZ Stacks) is a **native Layer 2 blockchain** that brings smart contract capabilities to BitcoinZ through the innovative **Proof of Transfer (PoX)** consensus mechanism. Unlike traditional bridges or wrapped tokens, BTCZS provides:

- ✅ **Native PoX Integration** - No bridges, no wrapped tokens
- ✅ **Smart Contracts** - Full Clarity language support
- ✅ **DeFi Capabilities** - DEXs, lending, staking on BitcoinZ
- ✅ **Bitcoin-level Security** - Inherits BitcoinZ's security model
- ✅ **Stacking Rewards** - Earn real BTCZ by participating

## 🏗️ **Architecture**

```
┌─────────────────┐    PoX     ┌─────────────────┐
│   BTCZS Layer 2 │◄──────────►│   BitcoinZ L1   │
│                 │            │                 │
│ • Smart Contracts│            │ • UTXO Model    │
│ • Clarity VM     │            │ • SHA256 PoW    │
│ • DeFi Apps      │            │ • 21B Supply    │
│ • Stacking       │            │ • 2.5min blocks │
└─────────────────┘            └─────────────────┘
```

### **How Proof of Transfer Works:**

1. **Miners** bid BTCZ to mine BTCZS blocks
2. **Stackers** lock BTCZS tokens to earn BTCZ rewards
3. **Smart Contracts** execute on BTCZS with BitcoinZ security
4. **No Bridge** - Direct integration with BitcoinZ blockchain

## 🚀 **Quick Start**

### **Prerequisites**
- Rust 1.70+ 
- BitcoinZ node (for mainnet integration)
- 4GB+ RAM

### **Build from Source**
```bash
# Clone the repository
git clone https://github.com/ztx-protocol/btczs-layer2.git
cd btczs-layer2

# Build BTCZS
cargo build --release

# Run tests
cargo test

# Start BTCZS node
./target/release/stacks-node start --config=testnet/stacks-node/conf/testnet-follower-conf.toml
```

## 📁 **Repository Structure**

```
btczs-layer2/
├── src/                    # Core BTCZS implementation
├── stackslib/              # Stacks blockchain library
├── stacks-common/          # Common utilities
├── clarity/                # Clarity smart contract VM
├── libsigner/              # Transaction signing
├── stacks-signer/          # Block signing
├── testnet/                # Testnet configurations
├── docs/                   # Technical documentation
├── scripts/                # Build and deployment scripts
├── .github/workflows/      # CI/CD pipelines
└── Cargo.toml             # Rust project configuration
```

## 🧪 **Testing**

### **Run All Tests**
```bash
# Unit tests
cargo test

# Integration tests
./test-btczs-functions.sh

# Performance tests  
./test-performance.sh

# PoX functionality tests
./test-pox-functions.sh
```

### **Test Results**
- ✅ **28/28 tests passing**
- ✅ **BitcoinZ RPC integration verified**
- ✅ **PoX cycles functioning correctly**
- ✅ **Smart contract execution confirmed**

## 💰 **Token Economics**

- **Total Supply**: 21 billion BTCZS (1:1 with BitcoinZ)
- **Distribution**: Proof of Transfer mining
- **Stacking Minimum**: 100,000 BTCZS
- **Cycle Duration**: ~3.5 days (1,000 BitcoinZ blocks)
- **Rewards**: Paid in real BTCZ

## 🔗 **API Reference**

### **RPC Methods**
```bash
# Get blockchain info
curl -X POST http://localhost:20443/v2/info

# Get account balance
curl -X GET http://localhost:20443/v2/accounts/SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7

# Submit transaction
curl -X POST http://localhost:20443/v2/transactions \
  -H "Content-Type: application/json" \
  -d '{"tx": "..."}'
```

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md).

### **Development Workflow**
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📚 **Documentation**

- **[Technical Specifications](TECHNICAL_SPECIFICATIONS.md)** - Detailed technical docs
- **[Token Economics](BTCZS_TOKEN_ECONOMICS.md)** - Economic model analysis
- **[Compatibility Report](BTCZS_BITCOINZ_POX_COMPATIBILITY_REPORT.md)** - BitcoinZ integration details
- **[Development Roadmap](DEVELOPMENT_ROADMAP.md)** - Project timeline
- **[CI/CD Documentation](CI_CD_DOCUMENTATION.md)** - Build and deployment

## 🔄 **CI/CD & Monitoring**

- **Automated Testing** - Every commit triggers full test suite
- **BitcoinZ Core Sync** - Daily monitoring of upstream changes
- **Security Scanning** - Automated vulnerability detection
- **Performance Monitoring** - Continuous performance tracking

## 📄 **License**

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## ⚡ **Status**

- **🟢 Mainnet**: Live and operational
- **🟢 Testnet**: Available for testing
- **🟢 RPC**: Fully functional
- **🟢 Smart Contracts**: Deployed and working
- **🟢 PoX**: Active stacking cycles

---

**Built with ❤️ for the BitcoinZ community**

*Bringing smart contracts and DeFi to BitcoinZ through native Layer 2 technology*
