// BTCZS Fee Structure Integration
// This module implements fee calculations and distribution for BTCZS operations

use serde::{Deserialize, Serialize};
use stacks_common::types::chainstate::StacksAddress;

use crate::burnchains::bitcoinz::burn::MIN_BITCOINZ_BURN_AMOUNT;
use crate::chainstate::burn::operations::bitcoinz_burn::BitcoinZBurnOperation;
use crate::chainstate::stacks::btczs_token::{BTCZSFees, MICRO_BTCZS_PER_BTCZS};
use crate::chainstate::stacks::StacksTransaction;
use crate::chainstate::stacks::Error as ChainstateError;

/// BTCZS fee configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BTCZSFeeConfig {
    /// Base fee rate in microBTCZS per byte
    pub base_fee_rate: u128,
    /// Minimum fee in microBTCZS
    pub min_fee: u128,
    /// Maximum fee in microBTCZS
    pub max_fee: u128,
    /// Fee multiplier for BitcoinZ operations
    pub bitcoinz_operation_multiplier: f64,
    /// Network congestion factor (0.0 to 1.0)
    pub congestion_factor: f64,
}

impl Default for BTCZSFeeConfig {
    fn default() -> Self {
        BTCZSFeeConfig {
            base_fee_rate: 100, // 100 microBTCZS per byte
            min_fee: 1000,      // 0.001 BTCZS minimum
            max_fee: 1000 * MICRO_BTCZS_PER_BTCZS, // 1000 BTCZS maximum
            bitcoinz_operation_multiplier: 1.5,
            congestion_factor: 0.0,
        }
    }
}

/// BTCZS fee calculation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BTCZSFeeCalculation {
    /// Base fee in microBTCZS
    pub base_fee: u128,
    /// Size-based fee in microBTCZS
    pub size_fee: u128,
    /// Operation-specific fee in microBTCZS
    pub operation_fee: u128,
    /// Congestion fee in microBTCZS
    pub congestion_fee: u128,
    /// Total fee in microBTCZS
    pub total_fee: u128,
    /// Fee breakdown description
    pub breakdown: String,
}

impl BTCZSFeeCalculation {
    /// Create a new fee calculation
    pub fn new(
        base_fee: u128,
        size_fee: u128,
        operation_fee: u128,
        congestion_fee: u128,
    ) -> Self {
        let total_fee = base_fee + size_fee + operation_fee + congestion_fee;
        let breakdown = format!(
            "Base: {} + Size: {} + Operation: {} + Congestion: {} = Total: {}",
            base_fee, size_fee, operation_fee, congestion_fee, total_fee
        );

        BTCZSFeeCalculation {
            base_fee,
            size_fee,
            operation_fee,
            congestion_fee,
            total_fee,
            breakdown,
        }
    }

    /// Get total fee in BTCZS (not microBTCZS)
    pub fn total_fee_btczs(&self) -> f64 {
        self.total_fee as f64 / MICRO_BTCZS_PER_BTCZS as f64
    }
}

/// BTCZS fee distribution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BTCZSFeeDistribution {
    /// Fees going to miners
    pub miner_fees: u128,
    /// Fees going to stackers
    pub stacker_fees: u128,
    /// Fees going to network fund
    pub network_fees: u128,
    /// Fees burned (removed from circulation)
    pub burned_fees: u128,
}

impl BTCZSFeeDistribution {
    /// Create fee distribution from total fees
    pub fn from_total_fees(total_fees: u128) -> Self {
        // Distribution percentages
        let miner_percentage = 60;    // 60% to miners
        let stacker_percentage = 25;  // 25% to stackers
        let network_percentage = 10;  // 10% to network fund
        let burn_percentage = 5;      // 5% burned

        BTCZSFeeDistribution {
            miner_fees: (total_fees * miner_percentage) / 100,
            stacker_fees: (total_fees * stacker_percentage) / 100,
            network_fees: (total_fees * network_percentage) / 100,
            burned_fees: (total_fees * burn_percentage) / 100,
        }
    }

    /// Get total distributed fees
    pub fn total(&self) -> u128 {
        self.miner_fees + self.stacker_fees + self.network_fees + self.burned_fees
    }
}

/// BTCZS fee calculator
pub struct BTCZSFeeCalculator {
    config: BTCZSFeeConfig,
}

impl BTCZSFeeCalculator {
    /// Create a new fee calculator
    pub fn new(config: BTCZSFeeConfig) -> Self {
        BTCZSFeeCalculator { config }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        BTCZSFeeCalculator::new(BTCZSFeeConfig::default())
    }

    /// Calculate fee for a Stacks transaction
    pub fn calculate_transaction_fee(
        &self,
        tx: &StacksTransaction,
    ) -> Result<BTCZSFeeCalculation, ChainstateError> {
        // Estimate transaction size (in practice, this would serialize the transaction)
        let tx_size = Self::estimate_transaction_size(tx);
        
        // Base fee
        let base_fee = self.config.min_fee;
        
        // Size-based fee
        let size_fee = tx_size * self.config.base_fee_rate;
        
        // Operation-specific fee (based on transaction type)
        let operation_fee = self.calculate_operation_fee(tx)?;
        
        // Congestion fee
        let congestion_fee = ((size_fee + operation_fee) as f64 * self.config.congestion_factor) as u128;
        
        let mut calculation = BTCZSFeeCalculation::new(base_fee, size_fee, operation_fee, congestion_fee);
        
        // Apply min/max limits
        if calculation.total_fee < self.config.min_fee {
            calculation.total_fee = self.config.min_fee;
        } else if calculation.total_fee > self.config.max_fee {
            calculation.total_fee = self.config.max_fee;
        }
        
        Ok(calculation)
    }

    /// Calculate fee for BitcoinZ operations
    pub fn calculate_bitcoinz_operation_fee(
        &self,
        operation: &BitcoinZBurnOperation,
    ) -> Result<BTCZSFeeCalculation, ChainstateError> {
        let operation_type = match operation {
            BitcoinZBurnOperation::LeaderBlockCommit(_) => "leader_block_commit",
            BitcoinZBurnOperation::StackStx(_) => "stack_stx",
            BitcoinZBurnOperation::Burn(_) => "burn",
        };

        let burn_amount = operation.burn_amount();
        
        // Base fee for BitcoinZ operations
        let base_fee = BTCZSFees::calculate_bitcoinz_operation_fee(operation_type, burn_amount);
        
        // Size fee (estimated transaction size)
        let estimated_size = 250; // Average BitcoinZ transaction size
        let size_fee = estimated_size * self.config.base_fee_rate;
        
        // Operation fee with BitcoinZ multiplier
        let operation_fee = (base_fee as f64 * self.config.bitcoinz_operation_multiplier) as u128;
        
        // Congestion fee
        let congestion_fee = ((size_fee + operation_fee) as f64 * self.config.congestion_factor) as u128;
        
        let calculation = BTCZSFeeCalculation::new(base_fee, size_fee, operation_fee, congestion_fee);
        
        Ok(calculation)
    }

    /// Calculate operation-specific fee for Stacks transactions
    fn calculate_operation_fee(&self, tx: &StacksTransaction) -> Result<u128, ChainstateError> {
        use crate::chainstate::stacks::TransactionPayload;
        
        let base_operation_fee = match &tx.payload {
            TransactionPayload::TokenTransfer(_, amount, _) => {
                // Fee scales with transfer amount
                let amount_fee = (*amount / 1_000_000) * 10; // 10 microBTCZS per STX
                1000 + amount_fee.min(10000) // Base + amount fee (capped)
            }
            TransactionPayload::ContractCall(_) => {
                2000 // Higher fee for contract calls
            }
            TransactionPayload::SmartContract(_, _) => {
                5000 // Highest fee for contract deployment
            }
            TransactionPayload::Coinbase(_, _, _) => {
                0 // No fee for coinbase
            }
            TransactionPayload::PoisonMicroblock(_, _) => {
                1000 // Standard fee for poison microblock
            }
            TransactionPayload::TenureChange(_) => {
                500 // Lower fee for tenure change
            }
        };

        Ok(base_operation_fee as u128)
    }

    /// Update congestion factor
    pub fn update_congestion_factor(&mut self, factor: f64) {
        self.config.congestion_factor = factor.max(0.0).min(2.0); // Cap between 0 and 2
    }

    /// Get current fee configuration
    pub fn get_config(&self) -> &BTCZSFeeConfig {
        &self.config
    }

    /// Update fee configuration
    pub fn update_config(&mut self, config: BTCZSFeeConfig) {
        self.config = config;
    }

    /// Estimate transaction size in bytes
    fn estimate_transaction_size(tx: &StacksTransaction) -> u128 {
        use crate::chainstate::stacks::TransactionPayload;

        // Base transaction overhead
        let mut size = 200u128; // Base size for headers, auth, etc.

        // Add size based on payload type
        size += match &tx.payload {
            TransactionPayload::TokenTransfer(_, _, _) => 100,
            TransactionPayload::ContractCall(call) => {
                200 + call.function_args.len() as u128 * 50 // Estimate based on args
            }
            TransactionPayload::SmartContract(contract, _) => {
                300 + contract.code_body.len() as u128 // Contract code size
            }
            TransactionPayload::Coinbase(_, _, _) => 150,
            TransactionPayload::PoisonMicroblock(_, _) => 250,
            TransactionPayload::TenureChange(_) => 180,
        };

        // Add size for post conditions
        size += tx.post_conditions.len() as u128 * 80;

        size
    }
}

/// BTCZS fee manager for handling fee collection and distribution
pub struct BTCZSFeeManager;

impl BTCZSFeeManager {
    /// Collect fees from a transaction
    pub fn collect_transaction_fee(
        payer: &StacksAddress,
        fee_calculation: &BTCZSFeeCalculation,
        block_height: u64,
    ) -> Result<(), ChainstateError> {
        // TODO: Implement fee collection from payer's BTCZS balance
        // This would integrate with BTCZSAccount::debit
        
        println!(
            "Collecting {} microBTCZS fee from {} at block {}",
            fee_calculation.total_fee, payer, block_height
        );
        
        Ok(())
    }

    /// Distribute collected fees
    pub fn distribute_fees(
        total_fees: u128,
        miner: &StacksAddress,
        stackers: &[StacksAddress],
        block_height: u64,
    ) -> Result<BTCZSFeeDistribution, ChainstateError> {
        let distribution = BTCZSFeeDistribution::from_total_fees(total_fees);
        
        // TODO: Implement actual fee distribution
        // This would integrate with BTCZSAccount::credit
        
        println!(
            "Distributing fees at block {}: Miner: {}, Stackers: {}, Network: {}, Burned: {}",
            block_height,
            distribution.miner_fees,
            distribution.stacker_fees,
            distribution.network_fees,
            distribution.burned_fees
        );
        
        Ok(distribution)
    }

    /// Calculate dynamic fee based on network conditions
    pub fn calculate_dynamic_fee_rate(
        recent_block_utilization: f64, // 0.0 to 1.0
        mempool_size: usize,
        target_block_time: u64, // seconds
        actual_block_time: u64,  // seconds
    ) -> f64 {
        let mut congestion_factor = 0.0;
        
        // Factor in block utilization
        if recent_block_utilization > 0.8 {
            congestion_factor += (recent_block_utilization - 0.8) * 2.0;
        }
        
        // Factor in mempool size
        if mempool_size > 1000 {
            congestion_factor += ((mempool_size - 1000) as f64 / 1000.0).min(1.0);
        }
        
        // Factor in block time deviation
        if actual_block_time > target_block_time {
            let time_factor = (actual_block_time as f64 / target_block_time as f64) - 1.0;
            congestion_factor += time_factor.min(1.0);
        }
        
        congestion_factor.min(2.0) // Cap at 2x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chainstate::stacks::{TransactionPayload, TokenTransferMemo};
    use stacks_common::types::chainstate::StacksAddress;
    use stacks_common::util::hash::Hash160;

    #[test]
    fn test_fee_calculation() {
        let calculator = BTCZSFeeCalculator::default();
        
        // Create a mock transaction
        let tx = create_mock_transfer_transaction(1000000); // 1 STX transfer
        
        let fee_calc = calculator.calculate_transaction_fee(&tx).unwrap();
        
        assert!(fee_calc.total_fee >= calculator.config.min_fee);
        assert!(fee_calc.total_fee <= calculator.config.max_fee);
        assert!(fee_calc.base_fee > 0);
        assert!(fee_calc.size_fee > 0);
    }

    #[test]
    fn test_fee_distribution() {
        let total_fees = 1000 * MICRO_BTCZS_PER_BTCZS; // 1000 BTCZS
        let distribution = BTCZSFeeDistribution::from_total_fees(total_fees);
        
        assert_eq!(distribution.miner_fees, total_fees * 60 / 100);
        assert_eq!(distribution.stacker_fees, total_fees * 25 / 100);
        assert_eq!(distribution.network_fees, total_fees * 10 / 100);
        assert_eq!(distribution.burned_fees, total_fees * 5 / 100);
        assert_eq!(distribution.total(), total_fees);
    }

    #[test]
    fn test_dynamic_fee_calculation() {
        // Low congestion
        let fee_rate = BTCZSFeeManager::calculate_dynamic_fee_rate(0.5, 500, 600, 600);
        assert_eq!(fee_rate, 0.0);
        
        // High block utilization
        let fee_rate = BTCZSFeeManager::calculate_dynamic_fee_rate(0.9, 500, 600, 600);
        assert!(fee_rate > 0.0);
        
        // Large mempool
        let fee_rate = BTCZSFeeManager::calculate_dynamic_fee_rate(0.5, 2000, 600, 600);
        assert!(fee_rate > 0.0);
        
        // Slow blocks
        let fee_rate = BTCZSFeeManager::calculate_dynamic_fee_rate(0.5, 500, 600, 900);
        assert!(fee_rate > 0.0);
    }

    #[test]
    fn test_congestion_factor_update() {
        let mut calculator = BTCZSFeeCalculator::default();
        
        calculator.update_congestion_factor(0.5);
        assert_eq!(calculator.config.congestion_factor, 0.5);
        
        // Test bounds
        calculator.update_congestion_factor(-1.0);
        assert_eq!(calculator.config.congestion_factor, 0.0);
        
        calculator.update_congestion_factor(5.0);
        assert_eq!(calculator.config.congestion_factor, 2.0);
    }

    // Helper function to create mock transaction
    fn create_mock_transfer_transaction(amount: u64) -> StacksTransaction {
        use crate::chainstate::stacks::*;
        
        let recipient = StacksAddress::new(0, Hash160([1u8; 20])).unwrap();
        let memo = TokenTransferMemo([0u8; 34]);
        
        StacksTransaction {
            version: TransactionVersion::Mainnet,
            chain_id: 0x80000000,
            auth: TransactionAuth::Standard(TransactionSpendingCondition::Singlesig(
                SinglesigSpendingCondition {
                    signer: Hash160([0u8; 20]),
                    hash_mode: SinglesigHashMode::P2PKH,
                    key_encoding: TransactionPublicKeyEncoding::Compressed,
                    nonce: 0,
                    tx_fee: 1000,
                    signature: MessageSignature([0u8; 65]),
                }
            )),
            anchor_mode: TransactionAnchorMode::OnChainOnly,
            post_condition_mode: TransactionPostConditionMode::Allow,
            post_conditions: vec![],
            payload: TransactionPayload::TokenTransfer(
                recipient.into(),
                amount,
                memo,
            ),
        }
    }
}
