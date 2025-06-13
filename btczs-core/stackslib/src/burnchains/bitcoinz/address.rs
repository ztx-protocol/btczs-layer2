// Copyright (C) 2013-2020 Blockstack PBC, a public benefit corporation
// Copyright (C) 2020 Stacks Open Internet Foundation
// Copyright (C) 2025 BTCZS Project
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// BitcoinZ Address handling
// BitcoinZ uses similar address formats to Bitcoin/Zcash

use std::fmt;

use stacks_common::util::hash::{Hash160, Sha256Sum};
use stacks_common::util::HexError;

use super::{BitcoinZNetworkType, Error};

/// BitcoinZ address types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BitcoinZAddressType {
    /// Pay-to-Public-Key-Hash (P2PKH)
    PublicKeyHash,
    /// Pay-to-Script-Hash (P2SH)
    ScriptHash,
    /// Shielded address (Zcash-style)
    Shielded,
}

/// BitcoinZ address structure
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BitcoinZAddress {
    pub address_type: BitcoinZAddressType,
    pub network: BitcoinZNetworkType,
    pub bytes: Vec<u8>,
}

impl BitcoinZAddress {
    /// Create a new BitcoinZ address
    pub fn new(
        address_type: BitcoinZAddressType,
        network: BitcoinZNetworkType,
        bytes: Vec<u8>,
    ) -> Self {
        Self {
            address_type,
            network,
            bytes,
        }
    }

    /// Create P2PKH address from public key hash
    pub fn from_public_key_hash(
        network: BitcoinZNetworkType,
        pubkey_hash: &Hash160,
    ) -> Self {
        Self::new(
            BitcoinZAddressType::PublicKeyHash,
            network,
            pubkey_hash.as_bytes().to_vec(),
        )
    }

    /// Create P2SH address from script hash
    pub fn from_script_hash(
        network: BitcoinZNetworkType,
        script_hash: &Hash160,
    ) -> Self {
        Self::new(
            BitcoinZAddressType::ScriptHash,
            network,
            script_hash.as_bytes().to_vec(),
        )
    }

    /// Get address version byte for BitcoinZ network
    fn get_version_byte(&self) -> u8 {
        match (&self.address_type, &self.network) {
            (BitcoinZAddressType::PublicKeyHash, BitcoinZNetworkType::Mainnet) => 0x1C, // BitcoinZ mainnet P2PKH
            (BitcoinZAddressType::PublicKeyHash, BitcoinZNetworkType::Testnet) => 0x1D, // BitcoinZ testnet P2PKH
            (BitcoinZAddressType::PublicKeyHash, BitcoinZNetworkType::Regtest) => 0x1D, // Same as testnet
            (BitcoinZAddressType::ScriptHash, BitcoinZNetworkType::Mainnet) => 0x1C,    // BitcoinZ mainnet P2SH
            (BitcoinZAddressType::ScriptHash, BitcoinZNetworkType::Testnet) => 0x1D,    // BitcoinZ testnet P2SH
            (BitcoinZAddressType::ScriptHash, BitcoinZNetworkType::Regtest) => 0x1D,    // Same as testnet
            (BitcoinZAddressType::Shielded, _) => 0x00, // Shielded addresses use different encoding
        }
    }

    /// Encode address to Base58Check format
    pub fn to_base58check(&self) -> String {
        if self.address_type == BitcoinZAddressType::Shielded {
            // Shielded addresses use different encoding
            // For now, return a placeholder
            return format!("zs1{}", self.bytes[..8].iter().map(|b| format!("{:02x}", b)).collect::<String>());
        }

        let version = self.get_version_byte();
        let mut payload = vec![version];
        payload.extend_from_slice(&self.bytes);

        // Calculate double SHA256 checksum
        let checksum = Sha256Sum::from_data(Sha256Sum::from_data(&payload).as_bytes());
        payload.extend_from_slice(&checksum.as_bytes()[..4]);

        base58_encode(&payload)
    }

    /// Parse address from Base58Check string
    pub fn from_base58check(
        address_str: &str,
        network: BitcoinZNetworkType,
    ) -> Result<Self, Error> {
        // Handle shielded addresses
        if address_str.starts_with("zs1") {
            // Simplified shielded address parsing
            let hex_part = &address_str[3..];
            if hex_part.len() >= 8 {
                let bytes = (0..4).map(|i| {
                    u8::from_str_radix(&hex_part[i*2..i*2+2], 16)
                        .map_err(|_| Error::InvalidByteSequence)
                }).collect::<Result<Vec<u8>, _>>()?;
                return Ok(Self::new(
                    BitcoinZAddressType::Shielded,
                    network,
                    bytes,
                ));
            }
        }

        // Decode Base58Check
        let decoded = base58_decode(address_str)
            .map_err(|_| Error::InvalidByteSequence)?;

        if decoded.len() < 25 {
            return Err(Error::InvalidByteSequence);
        }

        // Verify checksum
        let payload = &decoded[..decoded.len() - 4];
        let checksum = &decoded[decoded.len() - 4..];
        let calculated_checksum = Sha256Sum::from_data(Sha256Sum::from_data(payload).as_bytes());

        if checksum != &calculated_checksum.as_bytes()[..4] {
            return Err(Error::InvalidByteSequence);
        }

        let version = payload[0];
        let hash_bytes = payload[1..].to_vec();

        let address_type = match version {
            0x1C | 0x1D => BitcoinZAddressType::PublicKeyHash, // Simplified version check
            _ => return Err(Error::InvalidByteSequence),
        };

        Ok(Self::new(address_type, network, hash_bytes))
    }

    /// Check if address is valid for the given network
    pub fn is_valid_for_network(&self, network: BitcoinZNetworkType) -> bool {
        self.network == network
    }

    /// Get address as hex string
    pub fn to_hex(&self) -> String {
        self.bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    }
}

impl fmt::Display for BitcoinZAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_base58check())
    }
}

/// Simple Base58 encoding (Bitcoin-style)
fn base58_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    
    if input.is_empty() {
        return String::new();
    }

    // Count leading zeros
    let leading_zeros = input.iter().take_while(|&&b| b == 0).count();

    // Convert to base58
    let mut num = input.iter().fold(0u128, |acc, &b| acc * 256 + b as u128);
    let mut encoded = Vec::new();

    while num > 0 {
        encoded.push(ALPHABET[(num % 58) as usize]);
        num /= 58;
    }

    // Add leading '1's for leading zeros
    let mut result = vec![b'1'; leading_zeros];
    result.extend(encoded.iter().rev());

    String::from_utf8(result).unwrap_or_default()
}

/// Simple Base58 decoding
fn base58_decode(input: &str) -> Result<Vec<u8>, Error> {
    const ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    
    if input.is_empty() {
        return Ok(Vec::new());
    }

    // Count leading '1's
    let leading_ones = input.chars().take_while(|&c| c == '1').count();

    // Convert from base58
    let mut num = 0u128;
    for c in input.chars() {
        if let Some(pos) = ALPHABET.find(c) {
            num = num * 58 + pos as u128;
        } else {
            return Err(Error::InvalidByteSequence);
        }
    }

    // Convert to bytes
    let mut bytes = Vec::new();
    while num > 0 {
        bytes.push((num % 256) as u8);
        num /= 256;
    }
    bytes.reverse();

    // Add leading zeros
    let mut result = vec![0u8; leading_ones];
    result.extend(bytes);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcoinz_address_creation() {
        let hash = Hash160::from_data(b"test");
        let address = BitcoinZAddress::from_public_key_hash(
            BitcoinZNetworkType::Mainnet,
            &hash,
        );
        
        assert_eq!(address.address_type, BitcoinZAddressType::PublicKeyHash);
        assert_eq!(address.network, BitcoinZNetworkType::Mainnet);
        assert_eq!(address.bytes, hash.as_bytes());
    }

    #[test]
    fn test_base58_encoding() {
        let input = b"hello world";
        let encoded = base58_encode(input);
        let decoded = base58_decode(&encoded).unwrap();
        assert_eq!(input.to_vec(), decoded);
    }
}
