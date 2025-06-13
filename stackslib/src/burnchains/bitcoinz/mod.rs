// Copyright (C) 2013-2020 Blockstack PBC, a public benefit corporation
// Copyright (C) 2020 Stacks Open Internet Foundation
// Copyright (C) 2025 BTCZS Project
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// This module implements the BitcoinZ integration layer for BTCZS
// It adapts the Bitcoin indexer to work with BitcoinZ nodes

use std::sync::Arc;
use std::{error, fmt, io};

use stacks_common::deps_common::bitcoin::network::serialize::Error as btc_serialize_error;
use stacks_common::types::chainstate::BurnchainHeaderHash;
use stacks_common::util::HexError as btc_hex_error;

use crate::burnchains::bitcoin::address::BitcoinAddress;
use crate::burnchains::bitcoin::keys::BitcoinPublicKey;
use crate::burnchains::Txid;
use crate::chainstate::burn::operations::BlockstackOperationType;
use crate::deps;
use crate::util_lib::db::Error as db_error;

pub mod address;
pub mod burn;
pub mod indexer;
pub mod network;
pub mod rpc;

#[cfg(test)]
mod tests;

pub type PeerMessage = stacks_common::deps_common::bitcoin::network::message::NetworkMessage;

/// BitcoinZ Network Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BitcoinZNetworkType {
    Mainnet,
    Testnet,
    Regtest,
}

/// BitcoinZ Network Constants
pub const BITCOINZ_MAINNET_RPC_PORT: u16 = 1979;
pub const BITCOINZ_TESTNET_RPC_PORT: u16 = 11979;
pub const BITCOINZ_REGTEST_RPC_PORT: u16 = 11979;

pub const BITCOINZ_MAINNET_P2P_PORT: u16 = 1989;
pub const BITCOINZ_TESTNET_P2P_PORT: u16 = 11989;
pub const BITCOINZ_REGTEST_P2P_PORT: u16 = 11989;

/// BitcoinZ Network error types (adapted from Bitcoin module)
#[derive(Debug)]
pub enum Error {
    /// I/O error
    Io(io::Error),
    /// Not connected to peer
    SocketNotConnectedToPeer,
    /// Serialization error
    SerializationError(btc_serialize_error),
    /// Invalid Message to peer
    InvalidMessage(PeerMessage),
    /// Invalid Reply from peer
    InvalidReply,
    /// Invalid magic
    InvalidMagic,
    /// Unhandled message
    UnhandledMessage(PeerMessage),
    /// Connection is broken and ought to be re-established
    ConnectionBroken,
    /// Connection could not be (re-)established
    ConnectionError,
    /// general filesystem error
    FilesystemError(io::Error),
    /// Database error
    DBError(db_error),
    /// Hashing error
    HashError(btc_hex_error),
    /// Non-contiguous header
    NoncontiguousHeader,
    /// Missing header
    MissingHeader,
    /// Invalid header proof-of-work
    InvalidPoW,
    /// Chainwork would decrease by including a given header
    InvalidChainWork,
    /// Wrong number of bytes for constructing an address
    InvalidByteSequence,
    /// Configuration error
    ConfigError(String),
    /// Tried to synchronize to a point above the chain tip
    BlockchainHeight,
    /// Request timed out
    TimedOut,
    /// BitcoinZ specific errors
    BitcoinZRpcError(String),
    /// Invalid BitcoinZ transaction format
    InvalidBitcoinZTransaction,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => fmt::Display::fmt(e, f),
            Error::SocketNotConnectedToPeer => write!(f, "not connected to BitcoinZ peer"),
            Error::SerializationError(ref e) => fmt::Display::fmt(e, f),
            Error::InvalidMessage(ref _msg) => write!(f, "Invalid message to send to BitcoinZ"),
            Error::InvalidReply => write!(f, "invalid reply from BitcoinZ node"),
            Error::InvalidMagic => write!(f, "invalid BitcoinZ network magic"),
            Error::UnhandledMessage(ref _msg) => write!(f, "Unhandled BitcoinZ message"),
            Error::ConnectionBroken => write!(f, "connection to BitcoinZ node is broken"),
            Error::ConnectionError => write!(f, "connection to BitcoinZ node could not be established"),
            Error::FilesystemError(ref e) => fmt::Display::fmt(e, f),
            Error::DBError(ref e) => fmt::Display::fmt(e, f),
            Error::HashError(ref e) => fmt::Display::fmt(e, f),
            Error::NoncontiguousHeader => write!(f, "Non-contiguous BitcoinZ header"),
            Error::MissingHeader => write!(f, "Missing BitcoinZ header"),
            Error::InvalidPoW => write!(f, "Invalid BitcoinZ proof of work"),
            Error::InvalidChainWork => write!(f, "BitcoinZ chain difficulty cannot decrease"),
            Error::InvalidByteSequence => write!(f, "Invalid sequence of bytes"),
            Error::ConfigError(ref e_str) => fmt::Display::fmt(e_str, f),
            Error::BlockchainHeight => write!(f, "Value is beyond the end of the BitcoinZ blockchain"),
            Error::TimedOut => write!(f, "BitcoinZ request timed out"),
            Error::BitcoinZRpcError(ref e_str) => write!(f, "BitcoinZ RPC error: {}", e_str),
            Error::InvalidBitcoinZTransaction => write!(f, "Invalid BitcoinZ transaction format"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::SocketNotConnectedToPeer => None,
            Error::SerializationError(ref e) => Some(e),
            Error::InvalidMessage(ref _msg) => None,
            Error::InvalidReply => None,
            Error::InvalidMagic => None,
            Error::UnhandledMessage(ref _msg) => None,
            Error::ConnectionBroken => None,
            Error::ConnectionError => None,
            Error::FilesystemError(ref e) => Some(e),
            Error::DBError(ref e) => Some(e),
            Error::HashError(ref e) => Some(e),
            Error::NoncontiguousHeader => None,
            Error::MissingHeader => None,
            Error::InvalidPoW => None,
            Error::InvalidChainWork => None,
            Error::InvalidByteSequence => None,
            Error::ConfigError(ref _e_str) => None,
            Error::BlockchainHeight => None,
            Error::TimedOut => None,
            Error::BitcoinZRpcError(ref _e_str) => None,
            Error::InvalidBitcoinZTransaction => None,
        }
    }
}

impl From<db_error> for Error {
    fn from(e: db_error) -> Error {
        Error::DBError(e)
    }
}

/// BitcoinZ transaction output (compatible with Bitcoin format)
#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub struct BitcoinZTxOutput {
    pub address: BitcoinAddress, // Reuse Bitcoin address format for compatibility
    pub units: u64,
}

/// BitcoinZ transaction input (adapted for BTCZ format)
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinZTxInput {
    pub scriptSig: Vec<u8>,
    pub witness: Vec<Vec<u8>>, // BitcoinZ may not use witness, but keep for compatibility
    pub tx_ref: (Txid, u32),
}

/// BitcoinZ transaction structure
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BitcoinZTransaction {
    pub txid: Txid,
    pub vtxindex: u32,
    pub opcode: u8,
    pub data: Vec<u8>,
    /// how much BTCZ was sent to the data output
    pub data_amt: u64,
    pub inputs: Vec<BitcoinZTxInput>,
    pub outputs: Vec<BitcoinZTxOutput>,
}

/// BitcoinZ block structure
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BitcoinZBlock {
    pub block_height: u64,
    pub block_hash: BurnchainHeaderHash,
    pub parent_block_hash: BurnchainHeaderHash,
    pub txs: Vec<BitcoinZTransaction>,
    pub timestamp: u64,
}

impl BitcoinZBlock {
    pub fn new(
        height: u64,
        hash: &BurnchainHeaderHash,
        parent: &BurnchainHeaderHash,
        txs: Vec<BitcoinZTransaction>,
        timestamp: u64,
    ) -> BitcoinZBlock {
        BitcoinZBlock {
            block_height: height,
            block_hash: hash.clone(),
            parent_block_hash: parent.clone(),
            txs,
            timestamp,
        }
    }
}

/// Get default RPC port for BitcoinZ network type
pub fn get_bitcoinz_rpc_port(network: BitcoinZNetworkType) -> u16 {
    match network {
        BitcoinZNetworkType::Mainnet => BITCOINZ_MAINNET_RPC_PORT,
        BitcoinZNetworkType::Testnet => BITCOINZ_TESTNET_RPC_PORT,
        BitcoinZNetworkType::Regtest => BITCOINZ_REGTEST_RPC_PORT,
    }
}

/// Get default P2P port for BitcoinZ network type
pub fn get_bitcoinz_p2p_port(network: BitcoinZNetworkType) -> u16 {
    match network {
        BitcoinZNetworkType::Mainnet => BITCOINZ_MAINNET_P2P_PORT,
        BitcoinZNetworkType::Testnet => BITCOINZ_TESTNET_P2P_PORT,
        BitcoinZNetworkType::Regtest => BITCOINZ_REGTEST_P2P_PORT,
    }
}

/// Convert BitcoinZ network type to string
pub fn bitcoinz_network_to_string(network: BitcoinZNetworkType) -> &'static str {
    match network {
        BitcoinZNetworkType::Mainnet => "mainnet",
        BitcoinZNetworkType::Testnet => "testnet",
        BitcoinZNetworkType::Regtest => "regtest",
    }
}

/// Parse BitcoinZ network type from string
pub fn parse_bitcoinz_network(network_str: &str) -> Result<BitcoinZNetworkType, Error> {
    match network_str.to_lowercase().as_str() {
        "mainnet" => Ok(BitcoinZNetworkType::Mainnet),
        "testnet" => Ok(BitcoinZNetworkType::Testnet),
        "regtest" => Ok(BitcoinZNetworkType::Regtest),
        _ => Err(Error::ConfigError(format!("Invalid BitcoinZ network: {}", network_str))),
    }
}
