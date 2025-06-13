# BTCZS Token Economics Specification

## 🎯 Overview

BTCZS is the native token of the BitcoinZ Layer 2 solution, designed to create a sustainable economic model that bridges BitcoinZ burns with Layer 2 rewards. The token economics are carefully designed to reflect BitcoinZ's unique characteristics while providing incentives for network participation.

## 📊 Token Supply & Distribution

### Total Supply
- **BTCZS Total Supply**: 21 billion BTCZS
- **Rationale**: 1:1 parity with BitcoinZ's 21 billion total supply for user-friendly economics
- **Precision**: 1 BTCZS = 1,000,000 microBTCZS

### Genesis Distribution
- **Development Fund**: 10% (2.1 billion BTCZS)
- **Community Fund**: 20% (4.2 billion BTCZS)
- **Mining & Stacking Rewards**: 70% (14.7 billion BTCZS)

## ⛏️ Mining Rewards

### Block Rewards
- **Genesis Reward**: 12,500 BTCZS per block (1:1 with BitcoinZ)
- **Halving Interval**: Every 840,000 blocks (same as BitcoinZ)
- **Halving Schedule**:
  - Blocks 0-839,999: 12,500 BTCZS
  - Blocks 840,000-1,679,999: 6,250 BTCZS
  - Blocks 1,680,000-2,519,999: 3,125 BTCZS
  - And so on...

### Mining Bonuses
- **Base Reward**: Standard block reward based on height
- **Burn Bonus**: 10 microBTCZS per excess zatoshi burned above minimum
- **Total Mining Reward**: Base + Burn Bonus

## 🔥 BitcoinZ Burn Integration

### Conversion Rate
- **1 BTCZ burned = 1 BTCZS reward**
- **Ratio**: 1:1 (perfect parity)
- **Justification**: User-friendly economics with intuitive conversion

### Burn Operations
1. **Leader Block Commit**: Miners burn BTCZ to propose blocks
2. **Stacking Operations**: Users burn BTCZ to participate in stacking
3. **General Burns**: Any BTCZ burn contributes to reward pools

## 🏦 Stacking System

### Stacking Parameters
- **Minimum Stacking**: 1,000 BTCZS
- **Reward Cycle Length**: 2,100 blocks
- **Maximum Lock Period**: 12 cycles
- **Prepare Phase**: 100 blocks before each cycle

### Stacking Rewards
- **Source**: BitcoinZ burns during the cycle
- **Distribution**: Proportional to stacked amount
- **Duration Bonuses**:
  - 1-2 cycles: 1.0x multiplier
  - 3-6 cycles: 1.1x multiplier
  - 7-12 cycles: 1.25x multiplier
  - 12+ cycles: 1.5x multiplier

### Reward Calculation
```
Stacker Reward = (Total Cycle Burns × 100) × (Stacker Amount / Total Stacked) × Duration Multiplier
```

## 💰 Fee Structure

### Transaction Fees
- **Base Fee**: 1,000 microBTCZS minimum
- **Size Fee**: 100 microBTCZS per byte
- **Operation Fees**:
  - Token Transfer: 1,000 + amount-based fee
  - Contract Call: 2,000 microBTCZS
  - Contract Deploy: 5,000 microBTCZS
  - Coinbase: 0 microBTCZS

### BitcoinZ Operation Fees
- **Leader Block Commit**: 1,000 microBTCZS base
- **Stack STX**: 500 microBTCZS base
- **Burn**: 100 microBTCZS base
- **Large Burn Multiplier**: 2x for burns > 10x minimum

### Dynamic Fee Adjustment
- **Congestion Factor**: 0.0 to 2.0 multiplier
- **Factors**:
  - Block utilization > 80%
  - Mempool size > 1,000 transactions
  - Block time deviation from target

### Fee Distribution
- **Miners**: 60% of collected fees
- **Stackers**: 25% of collected fees
- **Network Fund**: 10% of collected fees
- **Burned**: 5% of collected fees (deflationary)

## 🔄 Economic Cycles

### Reward Cycles
1. **Cycle Start**: Stackers lock BTCZS for specified duration
2. **Burn Accumulation**: BitcoinZ burns throughout the cycle
3. **Reward Calculation**: At cycle end, calculate rewards
4. **Distribution**: Distribute BTCZS rewards to stackers
5. **Unlock**: Allow stackers to unlock after lock period

### Participation Incentives
- **High Participation Bonus**: 10% extra rewards when >10M STX stacked
- **Early Adopter Benefits**: Higher rewards in initial cycles
- **Network Security**: More stacking = more secure network

## 📈 Economic Model Benefits

### For BitcoinZ Holders
- **Utility**: BTCZ gains utility as burnchain for Layer 2
- **Rewards**: Earn BTCZS through stacking participation
- **Value Accrual**: Increased demand for BTCZ burns

### For BTCZS Holders
- **Stacking Rewards**: Earn additional BTCZS through stacking
- **Fee Discounts**: Reduced fees for network operations
- **Governance**: Future governance participation rights

### For Network Security
- **Burn Incentives**: Economic incentives for honest mining
- **Stacking Security**: Economic security through locked tokens
- **Fee Sustainability**: Self-sustaining fee model

## 🎯 Key Design Principles

1. **BitcoinZ Alignment**: Token economics reflect BitcoinZ characteristics
2. **Sustainable Rewards**: Long-term sustainable reward mechanisms
3. **Fair Distribution**: Equitable distribution across participants
4. **Network Security**: Economic incentives for network security
5. **Deflationary Pressure**: Fee burning creates deflationary pressure

## 📊 Economic Projections

### Year 1 Estimates
- **Total BTCZS Issued**: ~105 million (5% of supply)
- **Average Block Reward**: 5,000 BTCZS
- **Stacking Participation**: Target 30% of circulating supply
- **Fee Revenue**: Estimated 1-2% of total rewards

### Long-term Outlook
- **Supply Inflation**: Decreasing due to halving mechanism
- **Utility Growth**: Increasing utility drives demand
- **Network Effects**: Growing ecosystem increases value
- **Sustainability**: Self-sustaining economic model

## 🔧 Implementation Status

### ✅ Completed Features
- [x] BTCZS token balance management
- [x] Block reward calculation with halving
- [x] Stacking reward distribution
- [x] Fee calculation and distribution
- [x] BitcoinZ burn integration
- [x] Dynamic fee adjustment
- [x] Comprehensive test suite (13 tests)

### 🔄 Future Enhancements
- [ ] Governance token functionality
- [ ] Advanced stacking strategies
- [ ] Cross-chain bridge integration
- [ ] DeFi protocol integration

---

**Status**: ✅ **FULLY IMPLEMENTED AND TESTED**  
**Test Coverage**: 13/13 tests passing  
**Integration**: Complete BitcoinZ burn integration  
**Economics**: Sustainable and fair token model
