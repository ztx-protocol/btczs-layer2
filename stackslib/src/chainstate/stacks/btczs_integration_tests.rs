// BTCZS Integration Tests
// This module contains comprehensive integration tests for the BTCZS network

use std::collections::HashMap;

use crate::burnchains::bitcoinz::address::{BitcoinZAddress, BitcoinZAddressType};
use crate::burnchains::bitcoinz::BitcoinZNetworkType;
use crate::burnchains::bitcoinz::burn::MIN_BITCOINZ_BURN_AMOUNT;
use crate::burnchains::{Txid};
use crate::chainstate::burn::operations::bitcoinz_burn::{BitcoinZLeaderBlockCommitOp, BitcoinZStackStxOp, BitcoinZBurnOperation};
use crate::chainstate::stacks::address::PoxAddress;
use crate::chainstate::stacks::btczs_network::{BTCZSNetworkConfig, BTCZSNetworkType};
use crate::chainstate::stacks::btczs_token::{BTCZSRewards, BTCZSAccount, BTCZS_MIN_STACKING_AMOUNT};
use crate::chainstate::stacks::btczs_stacking::{BTCZSStackingManager, BTCZSStackingState};
use crate::chainstate::stacks::btczs_fees::{BTCZSFeeCalculator, BTCZSFeeManager};
use crate::chainstate::stacks::Error as ChainstateError;
use stacks_common::types::chainstate::{StacksAddress, BurnchainHeaderHash};
use stacks_common::util::hash::Hash160;

/// Integration test suite for BTCZS network functionality
pub struct BTCZSIntegrationTestSuite {
    network_config: BTCZSNetworkConfig,
    test_addresses: TestAddresses,
    test_state: TestState,
}

/// Test addresses for integration testing
#[derive(Debug, Clone)]
pub struct TestAddresses {
    pub miner_stacks: StacksAddress,
    pub miner_bitcoinz: BitcoinZAddress,
    pub stacker_stacks: StacksAddress,
    pub stacker_bitcoinz: BitcoinZAddress,
    pub user_stacks: StacksAddress,
    pub user_bitcoinz: BitcoinZAddress,
}

/// Test state tracking
#[derive(Debug, Clone)]
pub struct TestState {
    pub current_block_height: u64,
    pub current_reward_cycle: u64,
    pub total_burns: u64,
    pub total_rewards_distributed: u128,
    pub active_stackers: HashMap<StacksAddress, BTCZSStackingState>,
}

impl BTCZSIntegrationTestSuite {
    /// Create a new integration test suite
    pub fn new(network_type: BTCZSNetworkType) -> Self {
        let network_config = match network_type {
            BTCZSNetworkType::Mainnet => BTCZSNetworkConfig::mainnet(),
            BTCZSNetworkType::Testnet => BTCZSNetworkConfig::testnet(),
            BTCZSNetworkType::Regtest => BTCZSNetworkConfig::regtest(),
            BTCZSNetworkType::Devnet => BTCZSNetworkConfig::devnet(None),
        };

        BTCZSIntegrationTestSuite {
            network_config,
            test_addresses: TestAddresses::new(network_type),
            test_state: TestState::new(),
        }
    }

    /// Run comprehensive integration tests
    pub fn run_full_test_suite(&mut self) -> Result<(), ChainstateError> {
        println!("🚀 Starting BTCZS Integration Test Suite for {}", self.network_config.network_type.name());

        // Test 1: Network Configuration
        self.test_network_configuration()?;
        println!("✅ Network configuration test passed");

        // Test 2: Token Operations
        self.test_token_operations()?;
        println!("✅ Token operations test passed");

        // Test 3: Mining and Rewards
        self.test_mining_and_rewards()?;
        println!("✅ Mining and rewards test passed");

        // Test 4: Stacking Operations
        self.test_stacking_operations()?;
        println!("✅ Stacking operations test passed");

        // Test 5: Fee Calculations
        self.test_fee_calculations()?;
        println!("✅ Fee calculations test passed");

        // Test 6: BitcoinZ Integration
        self.test_bitcoinz_integration()?;
        println!("✅ BitcoinZ integration test passed");

        // Test 7: Reward Cycles
        self.test_reward_cycles()?;
        println!("✅ Reward cycles test passed");

        // Test 8: Network Stress Test
        self.test_network_stress()?;
        println!("✅ Network stress test passed");

        println!("🎉 All integration tests passed successfully!");
        Ok(())
    }

    /// Test network configuration
    fn test_network_configuration(&self) -> Result<(), ChainstateError> {
        // Validate network configuration
        self.network_config.validate()?;

        // Test network parameters
        assert!(self.network_config.consensus_params.target_block_time > 0);
        assert!(self.network_config.consensus_params.reward_cycle_length > 0);
        assert!(self.network_config.consensus_params.max_block_size > 0);

        // Test fee configuration
        assert!(self.network_config.fee_config.base_fee_rate > 0);
        assert!(self.network_config.fee_config.min_fee > 0);
        assert!(self.network_config.fee_config.max_fee > self.network_config.fee_config.min_fee);

        // Test genesis configuration
        assert!(self.network_config.genesis_config.genesis_timestamp > 0);
        assert!(!self.network_config.genesis_config.initial_distribution.is_empty());
        assert!(!self.network_config.genesis_config.genesis_miners.is_empty());

        Ok(())
    }

    /// Test token operations
    fn test_token_operations(&mut self) -> Result<(), ChainstateError> {
        // Test token balance operations
        let initial_balance = 1000 * 1_000_000; // 1000 BTCZS
        
        // Simulate token transfer
        BTCZSAccount::transfer(
            &self.test_addresses.user_stacks,
            &self.test_addresses.stacker_stacks,
            initial_balance / 2,
            self.test_state.current_block_height,
        )?;

        // Test stacking lock
        BTCZSAccount::lock_for_stacking(
            &self.test_addresses.stacker_stacks,
            BTCZS_MIN_STACKING_AMOUNT,
            self.test_state.current_block_height,
        )?;

        Ok(())
    }

    /// Test mining and rewards
    fn test_mining_and_rewards(&mut self) -> Result<(), ChainstateError> {
        let burn_amount = MIN_BITCOINZ_BURN_AMOUNT * 10;
        
        // Calculate mining reward
        let mining_reward = BTCZSRewards::calculate_mining_reward(
            burn_amount,
            self.test_state.current_block_height,
        );
        
        assert!(mining_reward > 0);
        
        // Calculate block reward
        let block_reward = BTCZSRewards::calculate_block_reward(
            self.test_state.current_block_height,
        );
        
        assert!(block_reward > 0);
        
        // Update test state
        self.test_state.total_burns += burn_amount;
        self.test_state.total_rewards_distributed += mining_reward;
        
        Ok(())
    }

    /// Test stacking operations
    fn test_stacking_operations(&mut self) -> Result<(), ChainstateError> {
        // Create stacking operation
        let stacking_op = BitcoinZStackStxOp {
            sender: self.test_addresses.stacker_stacks.clone(),
            reward_addr: self.test_addresses.stacker_bitcoinz.clone(),
            stacked_ustx: BTCZS_MIN_STACKING_AMOUNT,
            num_cycles: 6,
            txid: Txid([0x01; 32]),
            vtxindex: 0,
            block_height: self.test_state.current_block_height,
            burn_header_hash: BurnchainHeaderHash([0x01; 32]),
        };

        // Process stacking operation
        let stacking_state = BTCZSStackingManager::process_stacking_operation(
            &stacking_op,
            self.test_state.current_block_height,
        )?;

        // Validate stacking state
        assert_eq!(stacking_state.stacker, self.test_addresses.stacker_stacks);
        assert_eq!(stacking_state.stacked_ustx, BTCZS_MIN_STACKING_AMOUNT);
        assert_eq!(stacking_state.lock_period, 6);
        assert!(stacking_state.is_active(self.test_state.current_block_height + 1000));

        // Add to active stackers
        self.test_state.active_stackers.insert(
            self.test_addresses.stacker_stacks.clone(),
            stacking_state,
        );

        Ok(())
    }

    /// Test fee calculations
    fn test_fee_calculations(&self) -> Result<(), ChainstateError> {
        // Convert network fee config to calculator fee config
        let fee_config = crate::chainstate::stacks::btczs_fees::BTCZSFeeConfig {
            base_fee_rate: self.network_config.fee_config.base_fee_rate,
            min_fee: self.network_config.fee_config.min_fee,
            max_fee: self.network_config.fee_config.max_fee,
            bitcoinz_operation_multiplier: self.network_config.fee_config.bitcoinz_operation_multiplier,
            congestion_factor: 0.0,
        };
        let fee_calculator = BTCZSFeeCalculator::new(fee_config);
        
        // Test BitcoinZ operation fees
        let leader_commit_op = BitcoinZLeaderBlockCommitOp {
            sender: self.test_addresses.miner_bitcoinz.clone(),
            burn_fee: MIN_BITCOINZ_BURN_AMOUNT * 5,
            commit_outs: vec![PoxAddress::Standard(
                self.test_addresses.miner_stacks.clone(),
                None,
            )],
            txid: Txid([0x01; 32]),
            vtxindex: 0,
            block_height: self.test_state.current_block_height,
            burn_header_hash: BurnchainHeaderHash([0x01; 32]),
            block_header_hash: [0x01; 32],
            vrf_seed: [0x02; 32],
            key_block_ptr: 0,
            key_vtxindex: 0,
            parent_block_ptr: 0,
            parent_vtxindex: 0,
        };

        let operation = BitcoinZBurnOperation::LeaderBlockCommit(leader_commit_op);
        let fee_calc = fee_calculator.calculate_bitcoinz_operation_fee(&operation)?;
        assert!(fee_calc.total_fee >= self.network_config.fee_config.min_fee);
        assert!(fee_calc.total_fee <= self.network_config.fee_config.max_fee);

        Ok(())
    }

    /// Test BitcoinZ integration
    fn test_bitcoinz_integration(&mut self) -> Result<(), ChainstateError> {
        let burn_amount = MIN_BITCOINZ_BURN_AMOUNT * 20;
        
        // Test stacking reward calculation
        let total_stacked = BTCZS_MIN_STACKING_AMOUNT * 10; // Simulate 10 stackers
        let stacker_amount = BTCZS_MIN_STACKING_AMOUNT;
        
        let stacking_reward = BTCZSRewards::calculate_stacking_reward(
            burn_amount,
            total_stacked,
            stacker_amount,
        );
        
        assert!(stacking_reward > 0);
        
        // Test reward distribution
        let distributions = BTCZSFeeManager::distribute_fees(
            stacking_reward,
            &self.test_addresses.miner_stacks,
            &[self.test_addresses.stacker_stacks.clone()],
            self.test_state.current_block_height,
        )?;
        
        assert!(distributions.total() == stacking_reward);
        
        Ok(())
    }

    /// Test reward cycles
    fn test_reward_cycles(&mut self) -> Result<(), ChainstateError> {
        let cycle_length = self.network_config.consensus_params.reward_cycle_length;
        
        // Simulate multiple reward cycles
        for cycle in 0..3 {
            let cycle_start = cycle * cycle_length;
            let cycle_end = cycle_start + cycle_length;
            
            // Simulate burns throughout the cycle
            let mut cycle_burns = 0u64;
            for block in cycle_start..cycle_end {
                if block % 10 == 0 { // Burn every 10 blocks
                    cycle_burns += MIN_BITCOINZ_BURN_AMOUNT;
                }
            }
            
            // Process reward cycle completion
            let stackers: Vec<BTCZSStackingState> = self.test_state.active_stackers
                .values()
                .cloned()
                .collect();
            
            let distributions = BTCZSStackingManager::process_reward_cycle_completion(
                cycle,
                cycle_burns,
                stackers,
            )?;
            
            assert!(!distributions.is_empty());
            
            // Update test state
            self.test_state.current_reward_cycle = cycle;
            self.test_state.total_burns += cycle_burns;
        }
        
        Ok(())
    }

    /// Test network stress scenarios
    fn test_network_stress(&mut self) -> Result<(), ChainstateError> {
        // Test high congestion scenario
        let high_congestion = 0.9;
        let fee_rate = BTCZSFeeManager::calculate_dynamic_fee_rate(
            high_congestion,
            5000, // Large mempool
            self.network_config.consensus_params.target_block_time,
            self.network_config.consensus_params.target_block_time * 2, // Slow blocks
        );
        
        assert!(fee_rate > 0.0);
        
        // Test many simultaneous stackers
        let num_stackers = 100;
        for i in 0..num_stackers {
            let stacker_addr = StacksAddress::new(0, Hash160([i as u8; 20])).unwrap();
            let bitcoinz_addr = BitcoinZAddress::new(
                BitcoinZAddressType::PublicKeyHash,
                self.network_config.network_type.to_bitcoinz_network(),
                vec![i as u8; 20],
            );
            
            let stacking_op = BitcoinZStackStxOp {
                sender: stacker_addr.clone(),
                reward_addr: bitcoinz_addr,
                stacked_ustx: BTCZS_MIN_STACKING_AMOUNT,
                num_cycles: 3,
                txid: Txid([i as u8; 32]),
                vtxindex: i as u32,
                block_height: self.test_state.current_block_height,
                burn_header_hash: BurnchainHeaderHash([i as u8; 32]),
            };
            
            // Validate stacking operation
            BTCZSStackingManager::validate_stacking_operation(
                &stacker_addr,
                BTCZS_MIN_STACKING_AMOUNT,
                &stacking_op.reward_addr,
                3,
                self.test_state.current_block_height,
            )?;
        }
        
        Ok(())
    }

    /// Get test results summary
    pub fn get_test_summary(&self) -> TestSummary {
        TestSummary {
            network_type: self.network_config.network_type,
            total_burns: self.test_state.total_burns,
            total_rewards: self.test_state.total_rewards_distributed,
            active_stackers: self.test_state.active_stackers.len(),
            current_cycle: self.test_state.current_reward_cycle,
        }
    }
}

/// Test summary results
#[derive(Debug, Clone)]
pub struct TestSummary {
    pub network_type: BTCZSNetworkType,
    pub total_burns: u64,
    pub total_rewards: u128,
    pub active_stackers: usize,
    pub current_cycle: u64,
}

impl TestAddresses {
    fn new(network_type: BTCZSNetworkType) -> Self {
        let version = match network_type {
            BTCZSNetworkType::Mainnet => 0,
            BTCZSNetworkType::Testnet => 1,
            BTCZSNetworkType::Regtest => 2,
            BTCZSNetworkType::Devnet => 3,
        };

        let bitcoinz_network = network_type.to_bitcoinz_network();

        TestAddresses {
            miner_stacks: StacksAddress::new(version, Hash160([1u8; 20])).unwrap(),
            miner_bitcoinz: BitcoinZAddress::new(
                BitcoinZAddressType::PublicKeyHash,
                bitcoinz_network,
                vec![1u8; 20],
            ),
            stacker_stacks: StacksAddress::new(version, Hash160([2u8; 20])).unwrap(),
            stacker_bitcoinz: BitcoinZAddress::new(
                BitcoinZAddressType::PublicKeyHash,
                bitcoinz_network,
                vec![2u8; 20],
            ),
            user_stacks: StacksAddress::new(version, Hash160([3u8; 20])).unwrap(),
            user_bitcoinz: BitcoinZAddress::new(
                BitcoinZAddressType::PublicKeyHash,
                bitcoinz_network,
                vec![3u8; 20],
            ),
        }
    }
}

impl TestState {
    fn new() -> Self {
        TestState {
            current_block_height: 1000,
            current_reward_cycle: 0,
            total_burns: 0,
            total_rewards_distributed: 0,
            active_stackers: HashMap::new(),
        }
    }
}
