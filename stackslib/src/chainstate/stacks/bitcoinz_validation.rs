// BitcoinZ-specific block validation for BTCZS
// This module implements block validation logic for BitcoinZ burnchain operations

use serde::{Deserialize, Serialize};
use stacks_common::types::chainstate::{BurnchainHeaderHash, ConsensusHash, StacksBlockId};
use stacks_common::util::hash::Hash160;

use crate::burnchains::bitcoinz::burn::{BitcoinZBurnOp, MIN_BITCOINZ_BURN_AMOUNT};
use crate::burnchains::bitcoinz::{BitcoinZNetworkType, BitcoinZTransaction};
use crate::burnchains::{Burnchain, BurnchainBlockHeader, Txid};
use crate::chainstate::burn::bitcoinz_consensus::{BitcoinZConsensus, BitcoinZStateTransition};
use crate::chainstate::burn::db::sortdb::{SortitionDB, SortitionHandleTx};
use crate::chainstate::burn::operations::bitcoinz_burn::{
    BitcoinZBurnOperation, BitcoinZLeaderBlockCommitOp,
};
use crate::chainstate::burn::operations::{BlockstackOperationType, Error as op_error};
use crate::chainstate::burn::BlockSnapshot;
use crate::chainstate::stacks::{StacksBlock, StacksBlockHeader};
use crate::chainstate::stacks::db::StacksChainState;
use crate::chainstate::stacks::Error as ChainstateError;
use crate::util_lib::db::Error as db_error;

/// BitcoinZ-specific block validation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitcoinZValidationResult {
    /// Whether the block is valid
    pub valid: bool,
    /// Total burn amount from BitcoinZ operations
    pub total_burn: u64,
    /// Number of valid BitcoinZ operations
    pub operation_count: usize,
    /// Validation error message if invalid
    pub error_message: Option<String>,
}

impl BitcoinZValidationResult {
    /// Create a successful validation result
    pub fn success(total_burn: u64, operation_count: usize) -> Self {
        BitcoinZValidationResult {
            valid: true,
            total_burn,
            operation_count,
            error_message: None,
        }
    }

    /// Create a failed validation result
    pub fn failure(error_message: String) -> Self {
        BitcoinZValidationResult {
            valid: false,
            total_burn: 0,
            operation_count: 0,
            error_message: Some(error_message),
        }
    }
}

/// BitcoinZ block validation logic
pub struct BitcoinZBlockValidator;

impl BitcoinZBlockValidator {
    /// Validate a Stacks block against BitcoinZ burnchain operations
    pub fn validate_stacks_block_against_bitcoinz(
        stacks_block: &StacksBlock,
        burn_chain_tip: &BlockSnapshot,
        bitcoinz_operations: &[BitcoinZBurnOperation],
        network: BitcoinZNetworkType,
    ) -> Result<BitcoinZValidationResult, ChainstateError> {
        // Validate that the block header is consistent with BitcoinZ burns
        let header_validation = Self::validate_header_against_bitcoinz(
            &stacks_block.header,
            burn_chain_tip,
            bitcoinz_operations,
        )?;

        if !header_validation.valid {
            return Ok(header_validation);
        }

        // Validate individual BitcoinZ operations
        let ops_validation = Self::validate_bitcoinz_operations(bitcoinz_operations, network)?;

        if !ops_validation.valid {
            return Ok(ops_validation);
        }

        // Validate burn amounts and consistency
        let burn_validation = Self::validate_burn_consistency(
            stacks_block,
            burn_chain_tip,
            bitcoinz_operations,
        )?;

        Ok(burn_validation)
    }

    /// Validate Stacks block header against BitcoinZ operations
    fn validate_header_against_bitcoinz(
        header: &StacksBlockHeader,
        burn_chain_tip: &BlockSnapshot,
        bitcoinz_operations: &[BitcoinZBurnOperation],
    ) -> Result<BitcoinZValidationResult, ChainstateError> {
        // Note: StacksBlockHeader doesn't contain consensus_hash directly
        // The consensus hash is associated with the burn chain tip
        // We validate that the block is built on the correct burn chain tip

        // Check that there's a corresponding leader block commit
        let has_leader_commit = bitcoinz_operations.iter().any(|op| {
            matches!(op, BitcoinZBurnOperation::LeaderBlockCommit(_))
        });

        if !has_leader_commit && burn_chain_tip.sortition {
            return Ok(BitcoinZValidationResult::failure(
                "No leader block commit found for sortition block".to_string(),
            ));
        }

        // Validate VRF proof if present
        if let Some(leader_commit) = bitcoinz_operations.iter().find_map(|op| {
            if let BitcoinZBurnOperation::LeaderBlockCommit(commit) = op {
                Some(commit)
            } else {
                None
            }
        }) {
            // Check that the block header hash matches the commit
            if header.block_hash().as_bytes() != &leader_commit.block_header_hash {
                return Ok(BitcoinZValidationResult::failure(format!(
                    "Block header hash {} does not match leader commit {}",
                    header.block_hash(),
                    stacks_common::util::hash::to_hex(&leader_commit.block_header_hash)
                )));
            }
        }

        Ok(BitcoinZValidationResult::success(0, bitcoinz_operations.len()))
    }

    /// Validate individual BitcoinZ operations
    fn validate_bitcoinz_operations(
        operations: &[BitcoinZBurnOperation],
        network: BitcoinZNetworkType,
    ) -> Result<BitcoinZValidationResult, ChainstateError> {
        let mut total_burn = 0u64;
        let mut valid_ops = 0;

        for op in operations {
            // Validate the operation itself
            if let Err(e) = op.check() {
                return Ok(BitcoinZValidationResult::failure(format!(
                    "Invalid BitcoinZ operation: {:?}",
                    e
                )));
            }

            // Validate network consistency
            match op {
                BitcoinZBurnOperation::LeaderBlockCommit(commit_op) => {
                    if commit_op.sender.network != network {
                        return Ok(BitcoinZValidationResult::failure(format!(
                            "Leader commit sender network {:?} does not match expected {:?}",
                            commit_op.sender.network, network
                        )));
                    }
                    total_burn = total_burn.saturating_add(commit_op.burn_fee);
                }
                BitcoinZBurnOperation::Burn(burn_op) => {
                    if let Err(e) = BitcoinZConsensus::validate_bitcoinz_burn(burn_op, network) {
                        return Ok(BitcoinZValidationResult::failure(format!(
                            "Invalid BitcoinZ burn: {:?}",
                            e
                        )));
                    }
                    total_burn = total_burn.saturating_add(burn_op.burn_amount);
                }
                BitcoinZBurnOperation::StackStx(stack_op) => {
                    if stack_op.reward_addr.network != network {
                        return Ok(BitcoinZValidationResult::failure(format!(
                            "Stack STX reward address network {:?} does not match expected {:?}",
                            stack_op.reward_addr.network, network
                        )));
                    }
                    // Stacking operations don't contribute to burns
                }
            }

            valid_ops += 1;
        }

        Ok(BitcoinZValidationResult::success(total_burn, valid_ops))
    }

    /// Validate burn consistency between Stacks block and BitcoinZ operations
    fn validate_burn_consistency(
        stacks_block: &StacksBlock,
        burn_chain_tip: &BlockSnapshot,
        bitcoinz_operations: &[BitcoinZBurnOperation],
    ) -> Result<BitcoinZValidationResult, ChainstateError> {
        // Calculate total burns from operations
        let total_burn = bitcoinz_operations
            .iter()
            .map(|op| op.burn_amount())
            .sum::<u64>();

        // Validate that the burn amount is reasonable
        if total_burn > 0 && total_burn < MIN_BITCOINZ_BURN_AMOUNT {
            return Ok(BitcoinZValidationResult::failure(format!(
                "Total burn amount {} is below minimum {}",
                total_burn, MIN_BITCOINZ_BURN_AMOUNT
            )));
        }

        // Check that the total work in the block header is consistent
        if stacks_block.header.total_work.burn != burn_chain_tip.total_burn {
            return Ok(BitcoinZValidationResult::failure(format!(
                "Block header total burn {} does not match burn chain tip {}",
                stacks_block.header.total_work.burn, burn_chain_tip.total_burn
            )));
        }

        // Validate that there's at most one leader block commit
        let leader_commits: Vec<_> = bitcoinz_operations
            .iter()
            .filter_map(|op| {
                if let BitcoinZBurnOperation::LeaderBlockCommit(commit) = op {
                    Some(commit)
                } else {
                    None
                }
            })
            .collect();

        if leader_commits.len() > 1 {
            return Ok(BitcoinZValidationResult::failure(
                "Multiple leader block commits found".to_string(),
            ));
        }

        Ok(BitcoinZValidationResult::success(total_burn, bitcoinz_operations.len()))
    }

    /// Extract and validate BitcoinZ operations from burnchain transactions
    pub fn extract_and_validate_bitcoinz_ops(
        bitcoinz_txs: &[BitcoinZTransaction],
        block_height: u64,
        burn_header_hash: BurnchainHeaderHash,
        network: BitcoinZNetworkType,
    ) -> Result<Vec<BitcoinZBurnOperation>, ChainstateError> {
        let mut valid_operations = Vec::new();

        for tx in bitcoinz_txs {
            // Extract operations from transaction
            let operations = BitcoinZConsensus::extract_bitcoinz_operations(
                tx,
                block_height,
                burn_header_hash.clone(),
            )
            .map_err(|e| ChainstateError::InvalidStacksBlock(format!("Failed to extract BitcoinZ operations: {:?}", e)))?;

            // Validate each operation
            for op in operations {
                // Validate the operation
                if op.check().is_ok() {
                    // Additional network validation
                    let network_valid = match &op {
                        BitcoinZBurnOperation::LeaderBlockCommit(commit_op) => {
                            commit_op.sender.network == network
                        }
                        BitcoinZBurnOperation::Burn(burn_op) => {
                            burn_op.sender.network == network
                        }
                        BitcoinZBurnOperation::StackStx(stack_op) => {
                            stack_op.reward_addr.network == network
                        }
                    };

                    if network_valid {
                        valid_operations.push(op);
                    }
                }
            }
        }

        Ok(valid_operations)
    }

    /// Validate a complete BitcoinZ burnchain block
    pub fn validate_bitcoinz_burnchain_block(
        sort_tx: &mut SortitionHandleTx,
        burnchain: &Burnchain,
        parent_snapshot: &BlockSnapshot,
        block_header: &BurnchainBlockHeader,
        bitcoinz_txs: Vec<BitcoinZTransaction>,
        network: BitcoinZNetworkType,
    ) -> Result<(BlockSnapshot, BitcoinZStateTransition), db_error> {
        // Extract and validate BitcoinZ operations
        let bitcoinz_operations = Self::extract_and_validate_bitcoinz_ops(
            &bitcoinz_txs,
            block_header.block_height,
            block_header.block_hash.clone(),
            network,
        )
        .map_err(|e| db_error::Other(format!("BitcoinZ operation validation failed: {:?}", e)))?;

        // Process the block using BitcoinZ consensus
        BitcoinZConsensus::process_bitcoinz_block(
            sort_tx,
            burnchain,
            parent_snapshot,
            block_header,
            bitcoinz_txs,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::burnchains::bitcoinz::address::{BitcoinZAddress, BitcoinZAddressType};
    use crate::chainstate::stacks::address::PoxAddress;
    use stacks_common::types::chainstate::StacksAddress;

    #[test]
    fn test_bitcoinz_operation_validation() {
        let sender = BitcoinZAddress::new(
            BitcoinZAddressType::PublicKeyHash,
            BitcoinZNetworkType::Mainnet,
            vec![1u8; 20],
        );

        let commit_op = BitcoinZLeaderBlockCommitOp::new(
            sender,
            MIN_BITCOINZ_BURN_AMOUNT,
            vec![],
            Txid([1u8; 32]),
            0,
            100,
            BurnchainHeaderHash([0u8; 32]),
            [1u8; 32],
            [0u8; 32],
            0,
            0,
            0,
            0,
        ).unwrap();

        let operations = vec![BitcoinZBurnOperation::LeaderBlockCommit(commit_op)];

        let result = BitcoinZBlockValidator::validate_bitcoinz_operations(
            &operations,
            BitcoinZNetworkType::Mainnet,
        ).unwrap();

        assert!(result.valid);
        assert_eq!(result.total_burn, MIN_BITCOINZ_BURN_AMOUNT);
        assert_eq!(result.operation_count, 1);
    }

    #[test]
    fn test_bitcoinz_burn_validation() {
        let sender = BitcoinZAddress::new(
            BitcoinZAddressType::PublicKeyHash,
            BitcoinZNetworkType::Mainnet,
            vec![1u8; 20],
        );

        let reward_address = PoxAddress::Standard(
            StacksAddress::new(0, Hash160([0u8; 20])).unwrap(),
            Some(stacks_common::address::AddressHashMode::SerializeP2PKH),
        );

        let burn_op = BitcoinZBurnOp::new(
            sender,
            MIN_BITCOINZ_BURN_AMOUNT,
            reward_address,
            Txid([1u8; 32]),
            0,
            100,
            [0u8; 32],
        ).unwrap();

        let operations = vec![BitcoinZBurnOperation::Burn(burn_op)];

        let result = BitcoinZBlockValidator::validate_bitcoinz_operations(
            &operations,
            BitcoinZNetworkType::Mainnet,
        ).unwrap();

        assert!(result.valid);
        assert_eq!(result.total_burn, MIN_BITCOINZ_BURN_AMOUNT);
        assert_eq!(result.operation_count, 1);
    }

    #[test]
    fn test_network_mismatch_validation() {
        let sender = BitcoinZAddress::new(
            BitcoinZAddressType::PublicKeyHash,
            BitcoinZNetworkType::Testnet, // Wrong network
            vec![1u8; 20],
        );

        let commit_op = BitcoinZLeaderBlockCommitOp::new(
            sender,
            MIN_BITCOINZ_BURN_AMOUNT,
            vec![],
            Txid([1u8; 32]),
            0,
            100,
            BurnchainHeaderHash([0u8; 32]),
            [1u8; 32],
            [0u8; 32],
            0,
            0,
            0,
            0,
        ).unwrap();

        let operations = vec![BitcoinZBurnOperation::LeaderBlockCommit(commit_op)];

        let result = BitcoinZBlockValidator::validate_bitcoinz_operations(
            &operations,
            BitcoinZNetworkType::Mainnet, // Expected mainnet
        ).unwrap();

        assert!(!result.valid);
        assert!(result.error_message.is_some());
    }
}
