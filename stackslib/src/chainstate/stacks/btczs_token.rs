// BTCZS Token Economics Implementation
// This module implements the native BTCZS token mechanics for the BitcoinZ Layer 2

use serde::{Deserialize, Serialize};
use stacks_common::types::chainstate::{StacksAddress, StacksBlockId};
use stacks_common::util::hash::Hash160;

use crate::burnchains::bitcoinz::address::BitcoinZAddress;
use crate::burnchains::bitcoinz::burn::MIN_BITCOINZ_BURN_AMOUNT;
use crate::chainstate::stacks::db::accounts::MinerReward;
use crate::chainstate::stacks::Error as ChainstateError;

/// BTCZS token constants - VERIFIED from BitcoinZ source code
/// BitcoinZ has 21B total supply, so BTCZS will have 21B (1:1 ratio for user-friendly economics)
/// BitcoinZ genesis reward was 12,500 BTCZ, so BTCZS genesis reward is 12,500 BTCZS (1:1 ratio)
/// BitcoinZ halving interval is 840,000 blocks, block time is 2.5 minutes
pub const BTCZS_TOTAL_SUPPLY: u128 = 21_000_000_000_000_000; // 21B BTCZS in microBTCZS (1:1 with BitcoinZ)
pub const MICRO_BTCZS_PER_BTCZS: u128 = 1_000_000; // 1 BTCZS = 1,000,000 microBTCZS
pub const BTCZS_GENESIS_REWARD: u128 = 12500 * MICRO_BTCZS_PER_BTCZS; // 12,500 BTCZS (1:1 with BitcoinZ's 12,500)
pub const BTCZS_HALVING_INTERVAL: u64 = 840_000; // 840,000 blocks (verified from BitcoinZ source)
pub const BTCZS_MIN_STACKING_AMOUNT: u128 = 1000 * MICRO_BTCZS_PER_BTCZS; // 1000 BTCZS minimum for stacking

/// BTCZS token balance structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BTCZSBalance {
    /// Available BTCZS balance in microBTCZS
    pub available: u128,
    /// Locked BTCZS balance (for stacking) in microBTCZS
    pub locked: u128,
    /// Total BTCZS balance in microBTCZS
    pub total: u128,
    /// Block height when balance was last updated
    pub last_updated: u64,
}

impl BTCZSBalance {
    /// Create a new BTCZS balance
    pub fn new(available: u128, locked: u128, last_updated: u64) -> Self {
        BTCZSBalance {
            available,
            locked,
            total: available + locked,
            last_updated,
        }
    }

    /// Create an empty BTCZS balance
    pub fn zero(block_height: u64) -> Self {
        BTCZSBalance::new(0, 0, block_height)
    }

    /// Check if the balance can transfer the specified amount
    pub fn can_transfer(&self, amount: u128) -> bool {
        self.available >= amount
    }

    /// Debit available balance
    pub fn debit(&mut self, amount: u128) -> Result<(), ChainstateError> {
        if !self.can_transfer(amount) {
            return Err(ChainstateError::InvalidStacksBlock("Insufficient balance".to_string()));
        }
        self.available -= amount;
        self.total = self.available + self.locked;
        Ok(())
    }

    /// Credit available balance
    pub fn credit(&mut self, amount: u128) {
        self.available += amount;
        self.total = self.available + self.locked;
    }

    /// Lock BTCZS for stacking
    pub fn lock_for_stacking(&mut self, amount: u128) -> Result<(), ChainstateError> {
        if !self.can_transfer(amount) {
            return Err(ChainstateError::InvalidStacksBlock("Insufficient balance".to_string()));
        }
        self.available -= amount;
        self.locked += amount;
        self.total = self.available + self.locked;
        Ok(())
    }

    /// Unlock BTCZS from stacking
    pub fn unlock_from_stacking(&mut self, amount: u128) -> Result<(), ChainstateError> {
        if self.locked < amount {
            return Err(ChainstateError::InvalidStacksBlock("Insufficient balance".to_string()));
        }
        self.locked -= amount;
        self.available += amount;
        self.total = self.available + self.locked;
        Ok(())
    }
}

/// BTCZS reward calculation
pub struct BTCZSRewards;

impl BTCZSRewards {
    /// Calculate the BTCZS block reward at a given height
    /// Genesis: 1,250 BTCZS, After 1st halving (840k blocks): 625 BTCZS, After 2nd halving (1.68M blocks): 312.5 BTCZS
    pub fn calculate_block_reward(block_height: u64) -> u128 {
        let halvings = block_height / BTCZS_HALVING_INTERVAL;

        // Start with genesis reward (1,250 BTCZS) and halve for each halving period
        let mut reward = BTCZS_GENESIS_REWARD;
        for _ in 0..halvings {
            reward /= 2;
            if reward == 0 {
                break;
            }
        }

        reward
    }

    /// Calculate BTCZS stacking rewards based on BitcoinZ burns
    pub fn calculate_stacking_reward(
        bitcoinz_burn_amount: u64,
        total_stacked_btczs: u128,
        stacker_amount: u128,
    ) -> u128 {
        if total_stacked_btczs == 0 || stacker_amount == 0 {
            return 0;
        }

        // Convert BitcoinZ burn to BTCZS reward
        // 1 BTCZ burned = 1000 microBTCZS reward (1 BTCZS per 1 BTCZ burned)
        // This reflects the 1:1 supply ratio (21B BTCZ : 21B BTCZS)
        let btczs_reward_pool = (bitcoinz_burn_amount as u128) * 1000;
        
        // Distribute proportionally to stacker's share
        (btczs_reward_pool * stacker_amount) / total_stacked_btczs
    }

    /// Calculate mining rewards in BTCZS for BitcoinZ burns
    pub fn calculate_mining_reward(
        bitcoinz_burn_amount: u64,
        block_height: u64,
    ) -> u128 {
        let base_reward = Self::calculate_block_reward(block_height);
        
        // Bonus reward based on BitcoinZ burn amount
        // Higher burns get proportionally higher rewards
        let burn_bonus = if bitcoinz_burn_amount > MIN_BITCOINZ_BURN_AMOUNT {
            let excess_burn = bitcoinz_burn_amount - MIN_BITCOINZ_BURN_AMOUNT;
            (excess_burn as u128) * 10 // 10 microBTCZS per excess zatoshi burned
        } else {
            0
        };

        base_reward + burn_bonus
    }
}

/// BTCZS token distribution logic
pub struct BTCZSDistribution;

impl BTCZSDistribution {
    /// Calculate initial token distribution
    pub fn calculate_genesis_distribution() -> Vec<(StacksAddress, u128)> {
        let mut distribution = Vec::new();
        
        // Reserve 10% for development fund
        let dev_fund = BTCZS_TOTAL_SUPPLY / 10;
        
        // Reserve 20% for community rewards
        let community_fund = BTCZS_TOTAL_SUPPLY / 5;
        
        // Reserve 70% for mining and stacking rewards
        let mining_fund = BTCZS_TOTAL_SUPPLY - dev_fund - community_fund;
        
        // For now, create placeholder addresses
        // In production, these would be proper governance addresses
        let dev_address = StacksAddress::new(0, Hash160([1u8; 20])).unwrap();
        let community_address = StacksAddress::new(0, Hash160([2u8; 20])).unwrap();
        let mining_address = StacksAddress::new(0, Hash160([3u8; 20])).unwrap();
        
        distribution.push((dev_address, dev_fund));
        distribution.push((community_address, community_fund));
        distribution.push((mining_address, mining_fund));
        
        distribution
    }

    /// Calculate fair launch distribution based on BitcoinZ holdings
    pub fn calculate_fair_launch_distribution(
        bitcoinz_holders: Vec<(BitcoinZAddress, u64)>, // (address, BTCZ balance in zatoshis)
        total_btczs_for_airdrop: u128,
    ) -> Vec<(BitcoinZAddress, u128)> {
        let total_bitcoinz: u64 = bitcoinz_holders.iter().map(|(_, balance)| *balance).sum();
        
        if total_bitcoinz == 0 {
            return Vec::new();
        }

        bitcoinz_holders
            .into_iter()
            .map(|(address, balance)| {
                let btczs_amount = (total_btczs_for_airdrop * balance as u128) / total_bitcoinz as u128;
                (address, btczs_amount)
            })
            .collect()
    }

    /// Calculate stacking participation rewards
    pub fn calculate_stacking_participation_bonus(
        stacking_duration_cycles: u8,
        base_reward: u128,
    ) -> u128 {
        // Bonus for longer stacking periods
        let duration_multiplier = match stacking_duration_cycles {
            1..=2 => 100, // 1.0x
            3..=6 => 110, // 1.1x
            7..=12 => 125, // 1.25x
            _ => 150, // 1.5x for very long stacking
        };

        (base_reward * duration_multiplier) / 100
    }
}

/// BTCZS fee structure
pub struct BTCZSFees;

impl BTCZSFees {
    /// Calculate transaction fee in BTCZS for BitcoinZ operations
    pub fn calculate_bitcoinz_operation_fee(
        operation_type: &str,
        bitcoinz_burn_amount: u64,
    ) -> u128 {
        let base_fee = match operation_type {
            "leader_block_commit" => 1000, // 0.001 BTCZS
            "stack_stx" => 500,            // 0.0005 BTCZS
            "burn" => 100,                 // 0.0001 BTCZS
            _ => 1000,                     // Default fee
        };

        // Scale fee based on burn amount
        let burn_scale = if bitcoinz_burn_amount > MIN_BITCOINZ_BURN_AMOUNT * 10 {
            2 // 2x fee for large burns
        } else {
            1
        };

        base_fee * burn_scale
    }

    /// Calculate network fee for BTCZS transactions
    pub fn calculate_network_fee(
        transaction_size: u64,
        network_congestion: f64, // 0.0 to 1.0
    ) -> u128 {
        let base_fee_per_byte = 10; // 10 microBTCZS per byte
        let size_fee = transaction_size as u128 * base_fee_per_byte;
        
        // Apply congestion multiplier
        let congestion_multiplier = 1.0 + network_congestion;
        (size_fee as f64 * congestion_multiplier) as u128
    }

    /// Calculate stacking fee (percentage of rewards)
    pub fn calculate_stacking_fee(reward_amount: u128) -> u128 {
        // 2% fee on stacking rewards
        reward_amount / 50
    }
}

/// BTCZS token account management
pub struct BTCZSAccount;

impl BTCZSAccount {
    /// Get BTCZS balance for an address
    pub fn get_balance(
        _address: &StacksAddress,
        _block_height: u64,
    ) -> Result<BTCZSBalance, ChainstateError> {
        // TODO: Implement database lookup
        // For now, return zero balance
        Ok(BTCZSBalance::zero(0))
    }

    /// Update BTCZS balance for an address
    pub fn update_balance(
        _address: &StacksAddress,
        _new_balance: BTCZSBalance,
    ) -> Result<(), ChainstateError> {
        // TODO: Implement database update
        Ok(())
    }

    /// Transfer BTCZS between addresses
    pub fn transfer(
        from: &StacksAddress,
        to: &StacksAddress,
        amount: u128,
        block_height: u64,
    ) -> Result<(), ChainstateError> {
        // Get sender balance
        let mut from_balance = Self::get_balance(from, block_height)?;
        
        // Check if transfer is possible
        if !from_balance.can_transfer(amount) {
            return Err(ChainstateError::InvalidStacksBlock("Insufficient balance".to_string()));
        }

        // Get receiver balance
        let mut to_balance = Self::get_balance(to, block_height)?;

        // Perform transfer
        from_balance.debit(amount)?;
        to_balance.credit(amount);

        // Update balances
        Self::update_balance(from, from_balance)?;
        Self::update_balance(to, to_balance)?;

        Ok(())
    }

    /// Lock BTCZS for stacking
    pub fn lock_for_stacking(
        address: &StacksAddress,
        amount: u128,
        block_height: u64,
    ) -> Result<(), ChainstateError> {
        let mut balance = Self::get_balance(address, block_height)?;
        balance.lock_for_stacking(amount)?;
        Self::update_balance(address, balance)
    }

    /// Unlock BTCZS from stacking
    pub fn unlock_from_stacking(
        address: &StacksAddress,
        amount: u128,
        block_height: u64,
    ) -> Result<(), ChainstateError> {
        let mut balance = Self::get_balance(address, block_height)?;
        balance.unlock_from_stacking(amount)?;
        Self::update_balance(address, balance)
    }

    /// Mint new BTCZS tokens (for bridge operations)
    pub fn mint_tokens(
        address: &StacksAddress,
        amount: u128,
        block_height: u64,
    ) -> Result<(), ChainstateError> {
        let mut balance = Self::get_balance(address, block_height)?;
        balance.credit(amount);
        Self::update_balance(address, balance)
    }

    /// Burn BTCZS tokens (for bridge operations)
    pub fn burn_tokens(
        address: &StacksAddress,
        amount: u128,
        block_height: u64,
    ) -> Result<(), ChainstateError> {
        let mut balance = Self::get_balance(address, block_height)?;
        balance.debit(amount)?;
        Self::update_balance(address, balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btczs_balance_operations() {
        let mut balance = BTCZSBalance::new(1000 * MICRO_BTCZS_PER_BTCZS, 0, 100);
        
        // Test transfer capability
        assert!(balance.can_transfer(500 * MICRO_BTCZS_PER_BTCZS));
        assert!(!balance.can_transfer(1500 * MICRO_BTCZS_PER_BTCZS));
        
        // Test debit
        assert!(balance.debit(200 * MICRO_BTCZS_PER_BTCZS).is_ok());
        assert_eq!(balance.available, 800 * MICRO_BTCZS_PER_BTCZS);
        
        // Test credit
        balance.credit(100 * MICRO_BTCZS_PER_BTCZS);
        assert_eq!(balance.available, 900 * MICRO_BTCZS_PER_BTCZS);
        
        // Test stacking lock
        assert!(balance.lock_for_stacking(300 * MICRO_BTCZS_PER_BTCZS).is_ok());
        assert_eq!(balance.available, 600 * MICRO_BTCZS_PER_BTCZS);
        assert_eq!(balance.locked, 300 * MICRO_BTCZS_PER_BTCZS);
        
        // Test stacking unlock
        assert!(balance.unlock_from_stacking(100 * MICRO_BTCZS_PER_BTCZS).is_ok());
        assert_eq!(balance.available, 700 * MICRO_BTCZS_PER_BTCZS);
        assert_eq!(balance.locked, 200 * MICRO_BTCZS_PER_BTCZS);
    }

    #[test]
    fn test_btczs_block_rewards() {
        // Test initial reward (12,500 BTCZS - 1:1 with BitcoinZ)
        assert_eq!(BTCZSRewards::calculate_block_reward(0), BTCZS_GENESIS_REWARD);
        assert_eq!(BTCZSRewards::calculate_block_reward(0), 12500 * MICRO_BTCZS_PER_BTCZS);

        // Test first halving at 840,000 blocks (6,250 BTCZS)
        assert_eq!(
            BTCZSRewards::calculate_block_reward(BTCZS_HALVING_INTERVAL),
            BTCZS_GENESIS_REWARD / 2
        );
        assert_eq!(
            BTCZSRewards::calculate_block_reward(840_000),
            6250 * MICRO_BTCZS_PER_BTCZS
        );

        // Test second halving at 1,680,000 blocks (3,125 BTCZS)
        assert_eq!(
            BTCZSRewards::calculate_block_reward(BTCZS_HALVING_INTERVAL * 2),
            BTCZS_GENESIS_REWARD / 4
        );
        assert_eq!(
            BTCZSRewards::calculate_block_reward(1_680_000),
            3125 * MICRO_BTCZS_PER_BTCZS // 3,125 BTCZS in microBTCZS
        );
    }

    #[test]
    fn test_stacking_rewards() {
        let burn_amount = MIN_BITCOINZ_BURN_AMOUNT * 10;
        let total_stacked = 1000 * MICRO_BTCZS_PER_BTCZS;
        let stacker_amount = 100 * MICRO_BTCZS_PER_BTCZS;
        
        let reward = BTCZSRewards::calculate_stacking_reward(
            burn_amount,
            total_stacked,
            stacker_amount,
        );
        
        // Should get 10% of the reward pool (100/1000)
        let expected_pool = (burn_amount as u128) * 1000; // 1:1 conversion ratio (1000 microBTCZS per zatoshi)
        let expected_reward = (expected_pool * stacker_amount) / total_stacked;
        assert_eq!(reward, expected_reward);
    }

    #[test]
    fn test_fee_calculations() {
        // Test operation fees
        let leader_fee = BTCZSFees::calculate_bitcoinz_operation_fee(
            "leader_block_commit",
            MIN_BITCOINZ_BURN_AMOUNT,
        );
        assert_eq!(leader_fee, 1000);
        
        // Test network fees
        let network_fee = BTCZSFees::calculate_network_fee(1000, 0.5);
        assert_eq!(network_fee, 15000); // 1000 * 10 * 1.5
        
        // Test stacking fees
        let stacking_fee = BTCZSFees::calculate_stacking_fee(1000 * MICRO_BTCZS_PER_BTCZS);
        assert_eq!(stacking_fee, 20 * MICRO_BTCZS_PER_BTCZS); // 2%
    }

    #[test]
    fn test_genesis_distribution() {
        let distribution = BTCZSDistribution::calculate_genesis_distribution();
        
        assert_eq!(distribution.len(), 3);
        
        let total_distributed: u128 = distribution.iter().map(|(_, amount)| *amount).sum();
        assert_eq!(total_distributed, BTCZS_TOTAL_SUPPLY);
        
        // Check percentages
        assert_eq!(distribution[0].1, BTCZS_TOTAL_SUPPLY / 10); // 10% dev
        assert_eq!(distribution[1].1, BTCZS_TOTAL_SUPPLY / 5);  // 20% community
        assert_eq!(distribution[2].1, BTCZS_TOTAL_SUPPLY * 7 / 10); // 70% mining
    }
}
