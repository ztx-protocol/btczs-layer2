// Copyright (C) 2013-2020 Blockstack PBC, a public benefit corporation
// Copyright (C) 2020 Stacks Open Internet Foundation
// Copyright (C) 2025 BTCZS Project
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// BitcoinZ RPC Client implementation
// Adapted from Bitcoin RPC client to work with BitcoinZ nodes

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use serde_json::{json, Value};
use stacks_common::types::chainstate::BurnchainHeaderHash;
use stacks_common::util::log;

use super::{BitcoinZNetworkType, Error, get_bitcoinz_rpc_port};
use crate::burnchains::Txid;

/// Simple base64 encoding for HTTP Basic Auth
fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let bytes = input.as_bytes();

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[((b >> 6) & 63) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(b & 63) as usize] as char } else { '=' });
    }

    result
}

/// BitcoinZ RPC Client configuration
#[derive(Debug, Clone)]
pub struct BitcoinZRpcConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub timeout: Duration,
    pub network: BitcoinZNetworkType,
}

impl BitcoinZRpcConfig {
    pub fn new(
        host: String,
        network: BitcoinZNetworkType,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        let port = get_bitcoinz_rpc_port(network);
        Self {
            host,
            port,
            username,
            password,
            timeout: Duration::from_secs(60),
            network,
        }
    }

    pub fn default_mainnet() -> Self {
        Self::new(
            "127.0.0.1".to_string(),
            BitcoinZNetworkType::Mainnet,
            Some("btczrpc".to_string()),
            Some("password".to_string()),
        )
    }

    pub fn default_testnet() -> Self {
        Self::new(
            "127.0.0.1".to_string(),
            BitcoinZNetworkType::Testnet,
            Some("btczrpc".to_string()),
            Some("password".to_string()),
        )
    }

    pub fn default_regtest() -> Self {
        Self::new(
            "127.0.0.1".to_string(),
            BitcoinZNetworkType::Regtest,
            Some("btczrpc".to_string()),
            Some("password".to_string()),
        )
    }
}

/// BitcoinZ RPC Client
pub struct BitcoinZRpcClient {
    config: BitcoinZRpcConfig,
    request_id: u64,
}

impl BitcoinZRpcClient {
    pub fn new(config: BitcoinZRpcConfig) -> Self {
        Self {
            config,
            request_id: 0,
        }
    }

    /// Make an RPC call to BitcoinZ node
    pub fn call(&mut self, method: &str, params: Value) -> Result<Value, Error> {
        self.request_id += 1;
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method,
            "params": params
        });

        let request_body = serde_json::to_string(&request)
            .map_err(|e| Error::ConfigError(format!("Failed to serialize request: {}", e)))?;

        let response = self.send_http_request(&request_body)?;
        let response_json: Value = serde_json::from_str(&response)
            .map_err(|e| Error::BitcoinZRpcError(format!("Failed to parse response: {}", e)))?;

        if let Some(error) = response_json.get("error") {
            if !error.is_null() {
                return Err(Error::BitcoinZRpcError(format!("RPC error: {}", error)));
            }
        }

        response_json.get("result")
            .cloned()
            .ok_or_else(|| Error::BitcoinZRpcError("No result in response".to_string()))
    }

    /// Send HTTP request to BitcoinZ RPC server
    fn send_http_request(&self, body: &str) -> Result<String, Error> {
        let mut stream = TcpStream::connect((&self.config.host[..], self.config.port))
            .map_err(|_e| Error::ConnectionError)?;

        stream.set_read_timeout(Some(self.config.timeout))
            .map_err(|_e| Error::ConnectionError)?;
        stream.set_write_timeout(Some(self.config.timeout))
            .map_err(|_e| Error::ConnectionError)?;

        // Prepare HTTP request
        let auth_header = if let (Some(username), Some(password)) =
            (&self.config.username, &self.config.password) {
            // Simple base64 encoding for HTTP Basic Auth
            let credentials = format!("{}:{}", username, password);
            let encoded = base64_encode(&credentials);
            format!("Authorization: Basic {}\r\n", encoded)
        } else {
            String::new()
        };

        let http_request = format!(
            "POST / HTTP/1.1\r\n\
             Host: {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             {}\r\n\
             {}",
            self.config.host,
            body.len(),
            auth_header,
            body
        );

        // Send request
        stream.write_all(http_request.as_bytes())
            .map_err(|_e| Error::ConnectionError)?;

        // Read response
        let mut response = String::new();
        stream.read_to_string(&mut response)
            .map_err(|_e| Error::ConnectionError)?;

        // Extract JSON from HTTP response
        if let Some(json_start) = response.find("\r\n\r\n") {
            Ok(response[json_start + 4..].to_string())
        } else {
            Err(Error::BitcoinZRpcError("Invalid HTTP response".to_string()))
        }
    }

    /// Get blockchain info from BitcoinZ node
    pub fn get_blockchain_info(&mut self) -> Result<Value, Error> {
        self.call("getblockchaininfo", json!([]))
    }

    /// Get block count from BitcoinZ node
    pub fn get_block_count(&mut self) -> Result<u64, Error> {
        let result = self.call("getblockcount", json!([]))?;
        result.as_u64()
            .ok_or_else(|| Error::BitcoinZRpcError("Invalid block count response".to_string()))
    }

    /// Get block hash by height
    pub fn get_block_hash(&mut self, height: u64) -> Result<String, Error> {
        let result = self.call("getblockhash", json!([height]))?;
        result.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::BitcoinZRpcError("Invalid block hash response".to_string()))
    }

    /// Get block by hash
    pub fn get_block(&mut self, hash: &str, verbosity: u32) -> Result<Value, Error> {
        self.call("getblock", json!([hash, verbosity]))
    }

    /// Get block by height
    pub fn get_block_by_height(&mut self, height: u64, verbosity: u32) -> Result<Value, Error> {
        let hash = self.get_block_hash(height)?;
        self.get_block(&hash, verbosity)
    }

    /// Get raw transaction
    pub fn get_raw_transaction(&mut self, txid: &str, verbose: bool) -> Result<Value, Error> {
        self.call("getrawtransaction", json!([txid, verbose]))
    }

    /// Get transaction by ID
    pub fn get_transaction(&mut self, txid: &str) -> Result<Value, Error> {
        self.call("gettransaction", json!([txid]))
    }

    /// Send raw transaction
    pub fn send_raw_transaction(&mut self, hex: &str) -> Result<String, Error> {
        let result = self.call("sendrawtransaction", json!([hex]))?;
        result.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::BitcoinZRpcError("Invalid sendrawtransaction response".to_string()))
    }

    /// Get network info
    pub fn get_network_info(&mut self) -> Result<Value, Error> {
        self.call("getnetworkinfo", json!([]))
    }

    /// Get peer info
    pub fn get_peer_info(&mut self) -> Result<Value, Error> {
        self.call("getpeerinfo", json!([]))
    }

    /// Get mining info
    pub fn get_mining_info(&mut self) -> Result<Value, Error> {
        self.call("getmininginfo", json!([]))
    }

    /// Validate address
    pub fn validate_address(&mut self, address: &str) -> Result<Value, Error> {
        self.call("validateaddress", json!([address]))
    }

    /// Get best block hash
    pub fn get_best_block_hash(&mut self) -> Result<String, Error> {
        let result = self.call("getbestblockhash", json!([]))?;
        result.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::BitcoinZRpcError("Invalid best block hash response".to_string()))
    }

    /// Get difficulty
    pub fn get_difficulty(&mut self) -> Result<f64, Error> {
        let result = self.call("getdifficulty", json!([]))?;
        result.as_f64()
            .ok_or_else(|| Error::BitcoinZRpcError("Invalid difficulty response".to_string()))
    }

    /// Test connection to BitcoinZ node
    pub fn test_connection(&mut self) -> Result<bool, Error> {
        match self.get_blockchain_info() {
            Ok(_) => Ok(true),
            Err(_e) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcoinz_rpc_config() {
        let config = BitcoinZRpcConfig::default_mainnet();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 1979);
        assert_eq!(config.network, BitcoinZNetworkType::Mainnet);
    }

    #[test]
    fn test_bitcoinz_rpc_config_testnet() {
        let config = BitcoinZRpcConfig::default_testnet();
        assert_eq!(config.port, 11979);
        assert_eq!(config.network, BitcoinZNetworkType::Testnet);
    }
}
