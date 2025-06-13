# BTCZS Technical Specifications

## Network Architecture

### Core Components
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

## Network Parameters

### BTCZS Network Configuration
```toml
[btczs_network]
chain_id = 0x80000001          # BTCZS mainnet identifier
network_id = 1                 # Network ID
burn_fee_cap = 5000           # Maximum BTCZ satoshis for burn
peer_port = 20444             # P2P network port
rpc_port = 20443              # RPC server port
api_port = 3999               # API server port
block_time_ms = 600000        # 10 minutes (following BTCZ)
microblock_time_ms = 30000    # 30 seconds for microblocks
```

### BitcoinZ Connection
```toml
[btcz_connection]
rpc_host = "127.0.0.1"
rpc_port = 1979               # BTCZ default RPC port
rpc_username = "btczrpc"
rpc_password = "secure_password"
network = "mainnet"           # mainnet, testnet, regtest
min_confirmations = 6         # Minimum BTCZ confirmations
```

## Token Economics

### BTCZS Token Specifications
- **Name**: BTCZS (BitcoinZ Stacks)
- **Symbol**: BTCZS
- **Total Supply**: 21,000,000 BTCZS
- **Decimals**: 8
- **Initial Block Reward**: 1000 BTCZS
- **Halving Schedule**: Every 210,000 blocks (~4 years)
- **Mining Algorithm**: Proof of Transfer (PoX)

### Reward Schedule
```
Block Range        | Reward per Block | Total Rewards
0 - 210,000       | 1000 BTCZS      | 210,000,000 BTCZS
210,001 - 420,000 | 500 BTCZS       | 105,000,000 BTCZS
420,001 - 630,000 | 250 BTCZS       | 52,500,000 BTCZS
630,001 - 840,000 | 125 BTCZS       | 26,250,000 BTCZS
...               | ...             | ...
```

### Gas Fee Structure
- **Base Fee**: 1000 microBTCZS (0.00001 BTCZS)
- **Contract Call**: 5000 microBTCZS
- **Contract Deploy**: 50000 microBTCZS
- **Token Transfer**: 1000 microBTCZS
- **Fee Payment**: BTCZS or sBTCZ (via atomic swap)

## sBTCZ Peg Mechanism

### sBTCZ Token Specifications
- **Name**: Stacked BitcoinZ
- **Symbol**: sBTCZ
- **Peg Ratio**: 1:1 with BTCZ
- **Minimum Deposit**: 0.001 BTCZ (100,000 satoshis)
- **Maximum Deposit**: 100 BTCZ per transaction
- **Withdrawal Time**: 6 BTCZ confirmations (~60 minutes)

### Peg Operations
```clarity
;; Deposit BTCZ to mint sBTCZ
(define-public (deposit-btcz 
  (amount uint)
  (btcz-tx-id (buff 32))
  (merkle-proof (list 14 (buff 32)))
  (block-header (buff 80)))
  ;; Verify BTCZ transaction and mint sBTCZ
)

;; Withdraw sBTCZ to get BTCZ
(define-public (withdraw-btcz 
  (amount uint)
  (btcz-address (buff 34)))
  ;; Burn sBTCZ and initiate BTCZ withdrawal
)
```

### Signer Network
- **Signer Count**: 15-30 signers
- **Threshold**: 70% consensus required
- **Rotation**: Every 2100 blocks (~2 weeks)
- **Staking Requirement**: 100,000 BTCZS minimum
- **Reward**: 0.1% of peg volume

## Consensus Mechanism

### Proof of Transfer (PoX) for BTCZ
```rust
pub struct BTCZProofOfTransfer {
    pub burn_amount: u64,           // BTCZ satoshis burned
    pub burn_block_height: u64,     // BTCZ block height
    pub burn_tx_id: [u8; 32],      // BTCZ transaction ID
    pub miner_address: StacksAddress, // BTCZS miner address
    pub reward_address: StacksAddress, // Reward recipient
}
```

### Mining Process
1. **Burn BTCZ**: Miner sends BTCZ to burn address
2. **Commit Block**: Miner commits to mine next BTCZS block
3. **Block Production**: Winning miner produces BTCZS block
4. **Reward Distribution**: BTCZS rewards distributed to miner and stackers

### Stacking Mechanism
- **Minimum Stack**: 1000 BTCZS
- **Lock Period**: 2100 blocks (~2 weeks)
- **Reward**: Proportional share of BTCZ burns
- **Delegation**: Supported for smaller holders

## Smart Contract Platform

### Clarity Language Extensions
```clarity
;; BTCZ-specific functions
(define-read-only (get-btcz-block-info? (height uint))
  ;; Returns BTCZ block information
)

(define-read-only (verify-btcz-merkle-proof 
  (tx-bytes (buff 1024))
  (merkle-proof (list 14 (buff 32)))
  (block-header (buff 80)))
  ;; Verifies BTCZ transaction inclusion
)

(define-read-only (get-btcz-tx-sender (tx-bytes (buff 1024)))
  ;; Extracts sender from BTCZ transaction
)
```

### Contract Deployment
- **Max Contract Size**: 1MB
- **Deployment Cost**: 50,000 microBTCZS
- **Execution Limit**: 5,000,000 runtime cost units
- **Storage Limit**: 100MB per contract

## Network Security

### Security Model
- **Base Security**: BitcoinZ PoW consensus
- **Finality**: BTCZ block confirmations
- **Reorg Protection**: 6 BTCZ confirmations
- **Peg Security**: Multi-signature threshold

### Attack Vectors and Mitigations
1. **51% Attack**: Requires attacking BTCZ network
2. **Peg Attack**: Threshold signature protection
3. **Smart Contract Bugs**: Formal verification tools
4. **Network Partition**: Automatic recovery mechanisms

## Performance Specifications

### Throughput
- **Base Layer**: ~10 TPS (10-minute blocks)
- **With Microblocks**: ~1000 TPS (30-second microblocks)
- **Smart Contract Calls**: ~500 TPS
- **Token Transfers**: ~2000 TPS

### Storage Requirements
- **Full Node**: ~100GB initial, ~50GB/year growth
- **Archive Node**: ~500GB initial, ~200GB/year growth
- **Light Client**: ~1GB initial, ~100MB/year growth

### Network Requirements
- **Bandwidth**: 10 Mbps minimum, 100 Mbps recommended
- **Latency**: <500ms to BTCZ nodes
- **Uptime**: >99% for mining nodes

## API Specifications

### RPC Endpoints
```
# Node Information
GET /v2/info
GET /v2/pox

# Blocks and Transactions
GET /extended/v1/block/{hash}
GET /extended/v1/tx/{tx_id}

# BTCZ Integration
GET /v2/btcz/block/{height}
GET /v2/btcz/tx/{tx_id}/proof

# sBTCZ Operations
POST /v2/sbtcz/deposit
POST /v2/sbtcz/withdraw
GET /v2/sbtcz/status/{operation_id}
```

### WebSocket Events
```javascript
// Block events
ws.on('block', (block) => {
  // New BTCZS block
});

// Transaction events
ws.on('transaction', (tx) => {
  // New transaction
});

// sBTCZ events
ws.on('sbtcz_deposit', (deposit) => {
  // sBTCZ deposit confirmed
});
```

## Development Environment

### Prerequisites
- **Rust**: 1.70+
- **Node.js**: 18+
- **BitcoinZ Core**: Latest version
- **Docker**: For containerized deployment

### Build Configuration
```toml
[dependencies]
stacks-common = "0.1"
clarity = "2.0"
bitcoin = "0.30"
serde = "1.0"
tokio = "1.0"

[features]
default = ["btcz-integration"]
btcz-integration = []
testnet = []
```

**Last Updated**: January 13, 2025
