// BTCZS Stacking Implementation
// This module implements STX stacking with BitcoinZ rewards for BTCZS

use serde::{Deserialize, Serialize};
use stacks_common::types::chainstate::{StacksAddress, ConsensusHash, BurnchainHeaderHash};
use stacks_common::util::hash::Hash160;

use crate::burnchains::bitcoinz::address::BitcoinZAddress;
use crate::burnchains::bitcoinz::burn::MIN_BITCOINZ_BURN_AMOUNT;
use crate::chainstate::burn::operations::bitcoinz_burn::BitcoinZStackStxOp;
use crate::chainstate::stacks::address::PoxAddress;
use crate::chainstate::stacks::btczs_token::{BTCZSRewards, BTCZSFees, BTCZSDistribution, BTCZS_MIN_STACKING_AMOUNT};
use crate::chainstate::stacks::Error as ChainstateError;

/// BTCZS stacking cycle configuration
pub const BTCZS_REWARD_CYCLE_LENGTH: u64 = 2100; // blocks per reward cycle
pub const BTCZS_PREPARE_CYCLE_LENGTH: u64 = 100; // blocks to prepare for next cycle
pub const BTCZS_MAX_STACKING_CYCLES: u8 = 12; // maximum stacking duration

/// BTCZS stacking state for a user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BTCZSStackingState {
    /// The Stacks address that is stacking
    pub stacker: StacksAddress,
    /// Amount of STX being stacked (in microSTX)
    pub stacked_ustx: u128,
    /// BitcoinZ address for receiving rewards
    pub bitcoinz_reward_address: BitcoinZAddress,
    /// First reward cycle when stacking begins
    pub first_reward_cycle: u64,
    /// Number of cycles to stack for
    pub lock_period: u8,
    /// Block height when stacking ends
    pub unlock_burn_height: u64,
    /// Total BTCZS rewards earned
    pub total_btczs_rewards: u128,
    /// Last reward cycle processed
    pub last_reward_cycle: u64,
}

impl BTCZSStackingState {
    /// Create a new BTCZS stacking state
    pub fn new(
        stacker: StacksAddress,
        stacked_ustx: u128,
        bitcoinz_reward_address: BitcoinZAddress,
        first_reward_cycle: u64,
        lock_period: u8,
    ) -> Self {
        let unlock_burn_height = (first_reward_cycle + lock_period as u64) * BTCZS_REWARD_CYCLE_LENGTH;
        
        BTCZSStackingState {
            stacker,
            stacked_ustx,
            bitcoinz_reward_address,
            first_reward_cycle,
            lock_period,
            unlock_burn_height,
            total_btczs_rewards: 0,
            last_reward_cycle: 0,
        }
    }

    /// Check if stacking is currently active
    pub fn is_active(&self, current_burn_height: u64) -> bool {
        current_burn_height < self.unlock_burn_height
    }

    /// Check if stacking can be unlocked
    pub fn can_unlock(&self, current_burn_height: u64) -> bool {
        current_burn_height >= self.unlock_burn_height
    }

    /// Get the current reward cycle
    pub fn current_reward_cycle(burn_height: u64) -> u64 {
        burn_height / BTCZS_REWARD_CYCLE_LENGTH
    }

    /// Check if we're in the prepare phase for next cycle
    pub fn is_prepare_phase(burn_height: u64) -> bool {
        let cycle_position = burn_height % BTCZS_REWARD_CYCLE_LENGTH;
        cycle_position >= (BTCZS_REWARD_CYCLE_LENGTH - BTCZS_PREPARE_CYCLE_LENGTH)
    }
}

/// BTCZS reward cycle information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BTCZSRewardCycle {
    /// Reward cycle number
    pub cycle_number: u64,
    /// Total STX stacked in this cycle
    pub total_stacked_ustx: u128,
    /// Total BitcoinZ burned in this cycle
    pub total_bitcoinz_burned: u64,
    /// Total BTCZS rewards distributed
    pub total_btczs_rewards: u128,
    /// List of stackers in this cycle
    pub stackers: Vec<BTCZSStackingState>,
    /// Reward distribution completed
    pub rewards_distributed: bool,
}

impl BTCZSRewardCycle {
    /// Create a new reward cycle
    pub fn new(cycle_number: u64) -> Self {
        BTCZSRewardCycle {
            cycle_number,
            total_stacked_ustx: 0,
            total_bitcoinz_burned: 0,
            total_btczs_rewards: 0,
            stackers: Vec::new(),
            rewards_distributed: false,
        }
    }

    /// Add a stacker to this reward cycle
    pub fn add_stacker(&mut self, stacker: BTCZSStackingState) {
        self.total_stacked_ustx += stacker.stacked_ustx;
        self.stackers.push(stacker);
    }

    /// Add BitcoinZ burn to this cycle
    pub fn add_bitcoinz_burn(&mut self, burn_amount: u64) {
        self.total_bitcoinz_burned += burn_amount;
        
        // Calculate additional BTCZS rewards from this burn
        // Using the updated 1 BTCZ = 0.1 BTCZS conversion rate
        let additional_rewards = BTCZSRewards::calculate_stacking_reward(
            burn_amount,
            self.total_stacked_ustx,
            self.total_stacked_ustx, // Use total as base for pool calculation
        );
        self.total_btczs_rewards += additional_rewards;
    }

    /// Distribute rewards to stackers
    pub fn distribute_rewards(&mut self) -> Result<Vec<(BitcoinZAddress, u128)>, ChainstateError> {
        if self.rewards_distributed {
            return Err(ChainstateError::InvalidStacksBlock("Rewards already distributed".to_string()));
        }

        let mut distributions = Vec::new();

        for stacker in &mut self.stackers {
            if self.total_stacked_ustx > 0 {
                // Calculate stacker's share of rewards
                let stacker_reward = (self.total_btczs_rewards * stacker.stacked_ustx) / self.total_stacked_ustx;
                
                // Apply stacking duration bonus
                let bonus_reward = BTCZSDistribution::calculate_stacking_participation_bonus(
                    stacker.lock_period,
                    stacker_reward,
                );

                // Deduct stacking fee
                let fee = BTCZSFees::calculate_stacking_fee(bonus_reward);
                let final_reward = bonus_reward - fee;

                // Update stacker's total rewards
                stacker.total_btczs_rewards += final_reward;
                stacker.last_reward_cycle = self.cycle_number;

                distributions.push((stacker.bitcoinz_reward_address.clone(), final_reward));
            }
        }

        self.rewards_distributed = true;
        Ok(distributions)
    }
}

/// BTCZS stacking manager
pub struct BTCZSStackingManager;

impl BTCZSStackingManager {
    /// Validate a BTCZS stacking operation
    pub fn validate_stacking_operation(
        stacker: &StacksAddress,
        stacked_ustx: u128,
        bitcoinz_reward_address: &BitcoinZAddress,
        lock_period: u8,
        current_burn_height: u64,
    ) -> Result<(), ChainstateError> {
        // Check minimum stacking amount
        if stacked_ustx < BTCZS_MIN_STACKING_AMOUNT {
            return Err(ChainstateError::InvalidStacksBlock(format!(
                "Stacking amount {} below minimum {}",
                stacked_ustx, BTCZS_MIN_STACKING_AMOUNT
            )));
        }

        // Check lock period
        if lock_period == 0 || lock_period > BTCZS_MAX_STACKING_CYCLES {
            return Err(ChainstateError::InvalidStacksBlock(format!(
                "Invalid lock period: {}",
                lock_period
            )));
        }

        // Check that we're not in prepare phase
        if BTCZSStackingState::is_prepare_phase(current_burn_height) {
            return Err(ChainstateError::InvalidStacksBlock(
                "Cannot stack during prepare phase".to_string()
            ));
        }

        // Validate BitcoinZ address
        if bitcoinz_reward_address.bytes.len() != 20 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Invalid BitcoinZ reward address".to_string()
            ));
        }

        Ok(())
    }

    /// Process a BTCZS stacking operation
    pub fn process_stacking_operation(
        op: &BitcoinZStackStxOp,
        current_burn_height: u64,
    ) -> Result<BTCZSStackingState, ChainstateError> {
        // Validate the operation
        Self::validate_stacking_operation(
            &op.sender,
            op.stacked_ustx,
            &op.reward_addr,
            op.num_cycles,
            current_burn_height,
        )?;

        // Calculate first reward cycle
        let current_cycle = BTCZSStackingState::current_reward_cycle(current_burn_height);
        let first_reward_cycle = current_cycle + 1; // Start next cycle

        // Create stacking state
        let stacking_state = BTCZSStackingState::new(
            op.sender.clone(),
            op.stacked_ustx,
            op.reward_addr.clone(),
            first_reward_cycle,
            op.num_cycles,
        );

        Ok(stacking_state)
    }

    /// Calculate total stacking rewards for a cycle
    pub fn calculate_cycle_rewards(
        total_bitcoinz_burned: u64,
        total_stacked_ustx: u128,
    ) -> u128 {
        if total_stacked_ustx == 0 {
            return 0;
        }

        // Base reward pool from BitcoinZ burns
        let base_pool = BTCZSRewards::calculate_stacking_reward(
            total_bitcoinz_burned,
            total_stacked_ustx,
            total_stacked_ustx,
        );

        // Add bonus for high participation
        let participation_bonus = if total_stacked_ustx > 10_000_000 * 1_000_000 { // > 10M STX
            base_pool / 10 // 10% bonus
        } else {
            0
        };

        base_pool + participation_bonus
    }

    /// Get stacking information for an address
    pub fn get_stacking_info(
        _stacker: &StacksAddress,
        _current_burn_height: u64,
    ) -> Result<Option<BTCZSStackingState>, ChainstateError> {
        // TODO: Implement database lookup
        Ok(None)
    }

    /// Update stacking state
    pub fn update_stacking_state(
        _stacker: &StacksAddress,
        _state: BTCZSStackingState,
    ) -> Result<(), ChainstateError> {
        // TODO: Implement database update
        Ok(())
    }

    /// Process reward cycle completion
    pub fn process_reward_cycle_completion(
        cycle_number: u64,
        total_bitcoinz_burned: u64,
        stackers: Vec<BTCZSStackingState>,
    ) -> Result<Vec<(BitcoinZAddress, u128)>, ChainstateError> {
        let mut cycle = BTCZSRewardCycle::new(cycle_number);
        
        // Add all stackers to the cycle
        for stacker in stackers {
            cycle.add_stacker(stacker);
        }

        // Add total burns for the cycle
        cycle.add_bitcoinz_burn(total_bitcoinz_burned);

        // Distribute rewards
        cycle.distribute_rewards()
    }

    /// Check if stacking can be unlocked
    pub fn can_unlock_stacking(
        stacker: &StacksAddress,
        current_burn_height: u64,
    ) -> Result<bool, ChainstateError> {
        if let Some(stacking_state) = Self::get_stacking_info(stacker, current_burn_height)? {
            Ok(stacking_state.can_unlock(current_burn_height))
        } else {
            Ok(false)
        }
    }

    /// Unlock stacking for an address
    pub fn unlock_stacking(
        stacker: &StacksAddress,
        current_burn_height: u64,
    ) -> Result<u128, ChainstateError> {
        if let Some(mut stacking_state) = Self::get_stacking_info(stacker, current_burn_height)? {
            if !stacking_state.can_unlock(current_burn_height) {
                return Err(ChainstateError::InvalidStacksBlock(
                    "Stacking period not yet complete".to_string()
                ));
            }

            let unlocked_amount = stacking_state.stacked_ustx;
            
            // Remove stacking state (mark as unlocked)
            // TODO: Implement proper state management
            
            Ok(unlocked_amount)
        } else {
            Err(ChainstateError::InvalidStacksBlock(
                "No active stacking found".to_string()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::burnchains::bitcoinz::address::BitcoinZAddressType;
    use crate::burnchains::bitcoinz::BitcoinZNetworkType;

    #[test]
    fn test_btczs_stacking_state() {
        let stacker = StacksAddress::new(0, Hash160([1u8; 20])).unwrap();
        let reward_addr = BitcoinZAddress::new(
            BitcoinZAddressType::PublicKeyHash,
            BitcoinZNetworkType::Mainnet,
            vec![2u8; 20],
        );

        let stacking_state = BTCZSStackingState::new(
            stacker,
            1000 * 1_000_000, // 1000 STX
            reward_addr,
            10, // cycle 10
            6,  // 6 cycles
        );

        assert_eq!(stacking_state.first_reward_cycle, 10);
        assert_eq!(stacking_state.lock_period, 6);
        assert_eq!(stacking_state.unlock_burn_height, 16 * BTCZS_REWARD_CYCLE_LENGTH);

        // Test activity checks
        assert!(stacking_state.is_active(15 * BTCZS_REWARD_CYCLE_LENGTH));
        assert!(!stacking_state.is_active(17 * BTCZS_REWARD_CYCLE_LENGTH));
        assert!(stacking_state.can_unlock(16 * BTCZS_REWARD_CYCLE_LENGTH));
    }

    #[test]
    fn test_reward_cycle() {
        let mut cycle = BTCZSRewardCycle::new(5);
        
        let stacker1 = BTCZSStackingState::new(
            StacksAddress::new(0, Hash160([1u8; 20])).unwrap(),
            1000 * 1_000_000, // 1000 STX
            BitcoinZAddress::new(
                BitcoinZAddressType::PublicKeyHash,
                BitcoinZNetworkType::Mainnet,
                vec![1u8; 20],
            ),
            5,
            6,
        );

        let stacker2 = BTCZSStackingState::new(
            StacksAddress::new(0, Hash160([2u8; 20])).unwrap(),
            500 * 1_000_000, // 500 STX
            BitcoinZAddress::new(
                BitcoinZAddressType::PublicKeyHash,
                BitcoinZNetworkType::Mainnet,
                vec![2u8; 20],
            ),
            5,
            6,
        );

        cycle.add_stacker(stacker1);
        cycle.add_stacker(stacker2);
        cycle.add_bitcoinz_burn(MIN_BITCOINZ_BURN_AMOUNT * 100);

        assert_eq!(cycle.total_stacked_ustx, 1500 * 1_000_000);
        assert_eq!(cycle.total_bitcoinz_burned, MIN_BITCOINZ_BURN_AMOUNT * 100);
        assert!(cycle.total_btczs_rewards > 0);

        // Test reward distribution
        let distributions = cycle.distribute_rewards().unwrap();
        assert_eq!(distributions.len(), 2);
        assert!(cycle.rewards_distributed);

        // Should not be able to distribute again
        assert!(cycle.distribute_rewards().is_err());
    }

    #[test]
    fn test_stacking_validation() {
        let stacker = StacksAddress::new(0, Hash160([1u8; 20])).unwrap();
        let reward_addr = BitcoinZAddress::new(
            BitcoinZAddressType::PublicKeyHash,
            BitcoinZNetworkType::Mainnet,
            vec![1u8; 20],
        );

        // Valid stacking
        assert!(BTCZSStackingManager::validate_stacking_operation(
            &stacker,
            BTCZS_MIN_STACKING_AMOUNT,
            &reward_addr,
            6,
            1000,
        ).is_ok());

        // Invalid amount (too low)
        assert!(BTCZSStackingManager::validate_stacking_operation(
            &stacker,
            BTCZS_MIN_STACKING_AMOUNT - 1,
            &reward_addr,
            6,
            1000,
        ).is_err());

        // Invalid lock period (too long)
        assert!(BTCZSStackingManager::validate_stacking_operation(
            &stacker,
            BTCZS_MIN_STACKING_AMOUNT,
            &reward_addr,
            BTCZS_MAX_STACKING_CYCLES + 1,
            1000,
        ).is_err());

        // Invalid lock period (zero)
        assert!(BTCZSStackingManager::validate_stacking_operation(
            &stacker,
            BTCZS_MIN_STACKING_AMOUNT,
            &reward_addr,
            0,
            1000,
        ).is_err());
    }

    #[test]
    fn test_reward_cycle_calculations() {
        assert_eq!(BTCZSStackingState::current_reward_cycle(0), 0);
        assert_eq!(BTCZSStackingState::current_reward_cycle(BTCZS_REWARD_CYCLE_LENGTH), 1);
        assert_eq!(BTCZSStackingState::current_reward_cycle(BTCZS_REWARD_CYCLE_LENGTH * 5 + 100), 5);

        // Test prepare phase
        assert!(!BTCZSStackingState::is_prepare_phase(100));
        assert!(BTCZSStackingState::is_prepare_phase(BTCZS_REWARD_CYCLE_LENGTH - 50));
    }
}
