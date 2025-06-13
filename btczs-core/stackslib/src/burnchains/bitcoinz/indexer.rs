// Copyright (C) 2013-2020 Blockstack PBC, a public benefit corporation
// Copyright (C) 2020 Stacks Open Internet Foundation
// Copyright (C) 2025 BTCZS Project
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// BitcoinZ Indexer implementation
// Adapts the Bitcoin indexer to work with BitcoinZ blockchain

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;
use stacks_common::types::chainstate::BurnchainHeaderHash;
use stacks_common::util::log;

use super::rpc::{BitcoinZRpcClient, BitcoinZRpcConfig};
use super::{BitcoinZNetworkType, BitcoinZBlock, BitcoinZTransaction, Error};
use crate::burnchains::indexer::BurnchainIndexer;
use crate::burnchains::db::BurnchainBlockData;
use crate::burnchains::{Burnchain, BurnchainBlockHeader, MagicBytes, BLOCKSTACK_MAGIC_MAINNET, Txid};
use crate::core::{EpochList, STACKS_EPOCHS_MAINNET, STACKS_EPOCHS_REGTEST, STACKS_EPOCHS_TESTNET};
use crate::util_lib::db::Error as DBError;

pub const USER_AGENT: &str = "BTCZS/1.0";

// BitcoinZ network IDs (using BitcoinZ magic bytes)
pub const BITCOINZ_MAINNET: u32 = 0x24E92764;
pub const BITCOINZ_TESTNET: u32 = 0xFA1AF9BF;
pub const BITCOINZ_REGTEST: u32 = 0xAAB5BFFA;

pub const BITCOINZ_MAINNET_NAME: &str = "mainnet";
pub const BITCOINZ_TESTNET_NAME: &str = "testnet";
pub const BITCOINZ_REGTEST_NAME: &str = "regtest";

/// BitcoinZ Indexer Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct BitcoinZIndexerConfig {
    pub rpc_host: String,
    pub rpc_port: u16,
    pub rpc_username: Option<String>,
    pub rpc_password: Option<String>,
    pub timeout: u32,
    pub first_block: u64,
    pub magic_bytes: MagicBytes,
    pub epochs: Option<EpochList>,
    pub network: BitcoinZNetworkType,
}

impl BitcoinZIndexerConfig {
    pub fn default_mainnet(first_block: u64) -> BitcoinZIndexerConfig {
        BitcoinZIndexerConfig {
            rpc_host: "127.0.0.1".to_string(),
            rpc_port: 1979,
            rpc_username: Some("btczrpc".to_string()),
            rpc_password: Some("password".to_string()),
            timeout: 30,
            first_block,
            magic_bytes: BLOCKSTACK_MAGIC_MAINNET.clone(),
            epochs: None,
            network: BitcoinZNetworkType::Mainnet,
        }
    }

    pub fn default_testnet(first_block: u64) -> BitcoinZIndexerConfig {
        BitcoinZIndexerConfig {
            rpc_host: "127.0.0.1".to_string(),
            rpc_port: 11979,
            rpc_username: Some("btczrpc".to_string()),
            rpc_password: Some("password".to_string()),
            timeout: 30,
            first_block,
            magic_bytes: BLOCKSTACK_MAGIC_MAINNET.clone(),
            epochs: None,
            network: BitcoinZNetworkType::Testnet,
        }
    }

    pub fn default_regtest() -> BitcoinZIndexerConfig {
        BitcoinZIndexerConfig {
            rpc_host: "127.0.0.1".to_string(),
            rpc_port: 11979,
            rpc_username: Some("btczrpc".to_string()),
            rpc_password: Some("password".to_string()),
            timeout: 30,
            first_block: 0,
            magic_bytes: BLOCKSTACK_MAGIC_MAINNET.clone(),
            epochs: None,
            network: BitcoinZNetworkType::Regtest,
        }
    }
}

/// BitcoinZ Indexer Runtime
#[derive(Debug)]
pub struct BitcoinZIndexerRuntime {
    pub network: BitcoinZNetworkType,
    pub block_height: u64,
    pub timeout: u64,
}

impl BitcoinZIndexerRuntime {
    pub fn new(network: BitcoinZNetworkType) -> BitcoinZIndexerRuntime {
        BitcoinZIndexerRuntime {
            network,
            block_height: 0,
            timeout: 300,
        }
    }
}

/// BitcoinZ Indexer
pub struct BitcoinZIndexer {
    pub config: BitcoinZIndexerConfig,
    pub runtime: BitcoinZIndexerRuntime,
    pub rpc_client: BitcoinZRpcClient,
    pub should_keep_running: Option<Arc<AtomicBool>>,
}

impl BitcoinZIndexer {
    pub fn new(config: BitcoinZIndexerConfig) -> Result<BitcoinZIndexer, Error> {
        let runtime = BitcoinZIndexerRuntime::new(config.network);
        
        let rpc_config = BitcoinZRpcConfig::new(
            config.rpc_host.clone(),
            config.network,
            config.rpc_username.clone(),
            config.rpc_password.clone(),
        );
        
        let rpc_client = BitcoinZRpcClient::new(rpc_config);

        Ok(BitcoinZIndexer {
            config,
            runtime,
            rpc_client,
            should_keep_running: None,
        })
    }

    pub fn new_with_keep_running(
        config: BitcoinZIndexerConfig,
        should_keep_running: Arc<AtomicBool>,
    ) -> Result<BitcoinZIndexer, Error> {
        let mut indexer = Self::new(config)?;
        indexer.should_keep_running = Some(should_keep_running);
        Ok(indexer)
    }

    /// Test connection to BitcoinZ node
    pub fn test_connection(&mut self) -> Result<bool, Error> {
        self.rpc_client.test_connection()
    }

    /// Get current block height from BitcoinZ node
    pub fn get_block_height(&mut self) -> Result<u64, Error> {
        self.rpc_client.get_block_count()
    }

    /// Get block by height
    pub fn get_block_by_height(&mut self, height: u64) -> Result<BitcoinZBlock, Error> {
        let block_data = self.rpc_client.get_block_by_height(height, 2)?;
        self.parse_bitcoinz_block(block_data, height)
    }

    /// Get block by hash
    pub fn get_block_by_hash(&mut self, hash: &str) -> Result<BitcoinZBlock, Error> {
        let block_data = self.rpc_client.get_block(hash, 2)?;
        // Extract height from block data
        let height = block_data.get("height")
            .and_then(|h| h.as_u64())
            .ok_or_else(|| Error::BitcoinZRpcError("Missing block height".to_string()))?;
        
        self.parse_bitcoinz_block(block_data, height)
    }

    /// Parse BitcoinZ block from RPC response
    fn parse_bitcoinz_block(&self, block_data: Value, height: u64) -> Result<BitcoinZBlock, Error> {
        let hash_str = block_data.get("hash")
            .and_then(|h| h.as_str())
            .ok_or_else(|| Error::BitcoinZRpcError("Missing block hash".to_string()))?;

        let parent_hash_str = block_data.get("previousblockhash")
            .and_then(|h| h.as_str())
            .unwrap_or("0000000000000000000000000000000000000000000000000000000000000000");

        let timestamp = block_data.get("time")
            .and_then(|t| t.as_u64())
            .ok_or_else(|| Error::BitcoinZRpcError("Missing block timestamp".to_string()))?;

        // Parse block hash
        let block_hash = BurnchainHeaderHash::from_hex(hash_str)
            .map_err(|_| Error::BitcoinZRpcError("Invalid block hash format".to_string()))?;

        let parent_block_hash = BurnchainHeaderHash::from_hex(parent_hash_str)
            .map_err(|_| Error::BitcoinZRpcError("Invalid parent block hash format".to_string()))?;

        // Parse transactions
        let mut transactions = Vec::new();
        if let Some(tx_array) = block_data.get("tx").and_then(|t| t.as_array()) {
            for (index, tx_data) in tx_array.iter().enumerate() {
                if let Ok(tx) = self.parse_bitcoinz_transaction(tx_data, index as u32) {
                    transactions.push(tx);
                }
            }
        }

        Ok(BitcoinZBlock::new(
            height,
            &block_hash,
            &parent_block_hash,
            transactions,
            timestamp,
        ))
    }

    /// Parse BitcoinZ transaction from RPC response
    fn parse_bitcoinz_transaction(&self, tx_data: &Value, vtxindex: u32) -> Result<BitcoinZTransaction, Error> {
        let txid_str = tx_data.get("txid")
            .and_then(|t| t.as_str())
            .ok_or_else(|| Error::BitcoinZRpcError("Missing transaction ID".to_string()))?;

        // For now, create a minimal transaction structure
        // TODO: Implement full transaction parsing including inputs/outputs
        let txid_bytes = if txid_str.len() >= 64 {
            // Parse hex string to bytes
            let mut bytes = [0u8; 32];
            for i in 0..32 {
                if let Ok(byte) = u8::from_str_radix(&txid_str[i*2..i*2+2], 16) {
                    bytes[i] = byte;
                }
            }
            bytes
        } else {
            [0u8; 32]
        };

        Ok(BitcoinZTransaction {
            txid: Txid(txid_bytes),
            vtxindex,
            opcode: 0, // TODO: Extract actual opcode from transaction
            data: Vec::new(), // TODO: Extract OP_RETURN data
            data_amt: 0, // TODO: Calculate amount sent to data output
            inputs: Vec::new(), // TODO: Parse transaction inputs
            outputs: Vec::new(), // TODO: Parse transaction outputs
        })
    }

    /// Sync headers from BitcoinZ blockchain
    pub fn sync_headers(&mut self, start_height: u64, end_height: Option<u64>) -> Result<u64, Error> {
        let current_height = self.get_block_height()?;
        let target_height = end_height.unwrap_or(current_height);

        debug!("Syncing BitcoinZ headers from {} to {}", start_height, target_height);

        for height in start_height..=target_height {
            if let Some(ref should_keep_running) = self.should_keep_running {
                if !should_keep_running.load(Ordering::SeqCst) {
                    return Err(Error::TimedOut);
                }
            }

            // Get block header for this height
            let _block = self.get_block_by_height(height)?;
            
            // TODO: Store block header in database
            debug!("Processed BitcoinZ block at height {}", height);
        }

        Ok(target_height)
    }
}

/// Get default epochs for BitcoinZ network
pub fn get_bitcoinz_stacks_epochs(network: BitcoinZNetworkType) -> EpochList {
    match network {
        BitcoinZNetworkType::Mainnet => (*STACKS_EPOCHS_MAINNET).clone(),
        BitcoinZNetworkType::Testnet => (*STACKS_EPOCHS_TESTNET).clone(),
        BitcoinZNetworkType::Regtest => (*STACKS_EPOCHS_REGTEST).clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcoinz_indexer_config() {
        let config = BitcoinZIndexerConfig::default_mainnet(100);
        assert_eq!(config.rpc_host, "127.0.0.1");
        assert_eq!(config.rpc_port, 1979);
        assert_eq!(config.first_block, 100);
        assert_eq!(config.network, BitcoinZNetworkType::Mainnet);
    }

    #[test]
    fn test_bitcoinz_indexer_creation() {
        let config = BitcoinZIndexerConfig::default_regtest();
        let indexer = BitcoinZIndexer::new(config);
        assert!(indexer.is_ok());
    }
}
