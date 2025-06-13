// Copyright (C) 2013-2020 Blockstack PBC, a public benefit corporation
// Copyright (C) 2020 Stacks Open Internet Foundation
// Copyright (C) 2025 BTCZS Project
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// BitcoinZ Network configuration and constants

use super::BitcoinZNetworkType;

/// BitcoinZ network magic bytes (similar to Bitcoin)
pub const BITCOINZ_MAINNET_MAGIC: u32 = 0x24E92764;
pub const BITCOINZ_TESTNET_MAGIC: u32 = 0xFA1AF9BF;
pub const BITCOINZ_REGTEST_MAGIC: u32 = 0xAAB5BFFA;

/// BitcoinZ network configuration
#[derive(Debug, Clone)]
pub struct BitcoinZNetworkConfig {
    pub network_type: BitcoinZNetworkType,
    pub magic_bytes: u32,
    pub default_rpc_port: u16,
    pub default_p2p_port: u16,
    pub address_prefix: u8,
    pub script_prefix: u8,
    pub bech32_hrp: &'static str,
}

impl BitcoinZNetworkConfig {
    /// Get network configuration for mainnet
    pub fn mainnet() -> Self {
        Self {
            network_type: BitcoinZNetworkType::Mainnet,
            magic_bytes: BITCOINZ_MAINNET_MAGIC,
            default_rpc_port: 1979,
            default_p2p_port: 1989,
            address_prefix: 0x1C, // BitcoinZ mainnet address prefix
            script_prefix: 0x1C,  // BitcoinZ mainnet script prefix
            bech32_hrp: "bc",     // Bech32 human-readable part
        }
    }

    /// Get network configuration for testnet
    pub fn testnet() -> Self {
        Self {
            network_type: BitcoinZNetworkType::Testnet,
            magic_bytes: BITCOINZ_TESTNET_MAGIC,
            default_rpc_port: 11979,
            default_p2p_port: 11989,
            address_prefix: 0x1D, // BitcoinZ testnet address prefix
            script_prefix: 0x1D,  // BitcoinZ testnet script prefix
            bech32_hrp: "tb",     // Testnet bech32 HRP
        }
    }

    /// Get network configuration for regtest
    pub fn regtest() -> Self {
        Self {
            network_type: BitcoinZNetworkType::Regtest,
            magic_bytes: BITCOINZ_REGTEST_MAGIC,
            default_rpc_port: 11979,
            default_p2p_port: 11989,
            address_prefix: 0x1D, // Same as testnet
            script_prefix: 0x1D,  // Same as testnet
            bech32_hrp: "bcrt",   // Regtest bech32 HRP
        }
    }

    /// Get configuration for a specific network type
    pub fn for_network(network: BitcoinZNetworkType) -> Self {
        match network {
            BitcoinZNetworkType::Mainnet => Self::mainnet(),
            BitcoinZNetworkType::Testnet => Self::testnet(),
            BitcoinZNetworkType::Regtest => Self::regtest(),
        }
    }

    /// Check if magic bytes match this network
    pub fn matches_magic(&self, magic: u32) -> bool {
        self.magic_bytes == magic
    }

    /// Get network name as string
    pub fn network_name(&self) -> &'static str {
        match self.network_type {
            BitcoinZNetworkType::Mainnet => "mainnet",
            BitcoinZNetworkType::Testnet => "testnet",
            BitcoinZNetworkType::Regtest => "regtest",
        }
    }
}

/// BitcoinZ consensus parameters
#[derive(Debug, Clone)]
pub struct BitcoinZConsensusParams {
    pub network: BitcoinZNetworkType,
    pub pow_limit: [u8; 32],
    pub pow_target_timespan: u64,
    pub pow_target_spacing: u64,
    pub pow_allow_min_difficulty_blocks: bool,
    pub pow_no_retargeting: bool,
    pub subsidy_halving_interval: u64,
    pub coinbase_maturity: u64,
}

impl BitcoinZConsensusParams {
    /// Get consensus parameters for mainnet
    pub fn mainnet() -> Self {
        Self {
            network: BitcoinZNetworkType::Mainnet,
            pow_limit: [
                0x00, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks
            pow_target_spacing: 150,  // 2.5 minutes in seconds
            pow_allow_min_difficulty_blocks: false,
            pow_no_retargeting: false,
            subsidy_halving_interval: 840000, // BitcoinZ halving interval
            coinbase_maturity: 100,
        }
    }

    /// Get consensus parameters for testnet
    pub fn testnet() -> Self {
        let mut params = Self::mainnet();
        params.network = BitcoinZNetworkType::Testnet;
        params.pow_allow_min_difficulty_blocks = true;
        params
    }

    /// Get consensus parameters for regtest
    pub fn regtest() -> Self {
        let mut params = Self::mainnet();
        params.network = BitcoinZNetworkType::Regtest;
        params.pow_allow_min_difficulty_blocks = true;
        params.pow_no_retargeting = true;
        params.subsidy_halving_interval = 150; // Faster halving for testing
        params.coinbase_maturity = 100;
        params
    }

    /// Get parameters for a specific network
    pub fn for_network(network: BitcoinZNetworkType) -> Self {
        match network {
            BitcoinZNetworkType::Mainnet => Self::mainnet(),
            BitcoinZNetworkType::Testnet => Self::testnet(),
            BitcoinZNetworkType::Regtest => Self::regtest(),
        }
    }

    /// Calculate next difficulty target
    pub fn calculate_next_work_required(
        &self,
        last_block_time: u64,
        first_block_time: u64,
        current_target: &[u8; 32],
    ) -> [u8; 32] {
        if self.pow_no_retargeting {
            return *current_target;
        }

        let actual_timespan = last_block_time.saturating_sub(first_block_time);
        let mut adjusted_timespan = actual_timespan;

        // Limit adjustment to 4x in either direction
        let max_timespan = self.pow_target_timespan * 4;
        let min_timespan = self.pow_target_timespan / 4;

        if adjusted_timespan < min_timespan {
            adjusted_timespan = min_timespan;
        } else if adjusted_timespan > max_timespan {
            adjusted_timespan = max_timespan;
        }

        // Calculate new target
        // new_target = current_target * adjusted_timespan / target_timespan
        // For simplicity, return current target (full implementation would do big integer math)
        *current_target
    }

    /// Check if target meets difficulty requirement
    pub fn check_proof_of_work(&self, hash: &[u8; 32], target: &[u8; 32]) -> bool {
        // Compare hash with target (hash must be less than target)
        for i in 0..32 {
            if hash[i] < target[i] {
                return true;
            } else if hash[i] > target[i] {
                return false;
            }
        }
        false // Equal is not valid
    }
}

/// Get magic bytes for network type
pub fn get_magic_bytes(network: BitcoinZNetworkType) -> u32 {
    match network {
        BitcoinZNetworkType::Mainnet => BITCOINZ_MAINNET_MAGIC,
        BitcoinZNetworkType::Testnet => BITCOINZ_TESTNET_MAGIC,
        BitcoinZNetworkType::Regtest => BITCOINZ_REGTEST_MAGIC,
    }
}

/// Parse network type from magic bytes
pub fn parse_network_from_magic(magic: u32) -> Option<BitcoinZNetworkType> {
    match magic {
        BITCOINZ_MAINNET_MAGIC => Some(BitcoinZNetworkType::Mainnet),
        BITCOINZ_TESTNET_MAGIC => Some(BitcoinZNetworkType::Testnet),
        BITCOINZ_REGTEST_MAGIC => Some(BitcoinZNetworkType::Regtest),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config() {
        let mainnet = BitcoinZNetworkConfig::mainnet();
        assert_eq!(mainnet.network_type, BitcoinZNetworkType::Mainnet);
        assert_eq!(mainnet.default_rpc_port, 1979);
        assert_eq!(mainnet.magic_bytes, BITCOINZ_MAINNET_MAGIC);

        let testnet = BitcoinZNetworkConfig::testnet();
        assert_eq!(testnet.network_type, BitcoinZNetworkType::Testnet);
        assert_eq!(testnet.default_rpc_port, 11979);
    }

    #[test]
    fn test_consensus_params() {
        let mainnet_params = BitcoinZConsensusParams::mainnet();
        assert_eq!(mainnet_params.network, BitcoinZNetworkType::Mainnet);
        assert_eq!(mainnet_params.subsidy_halving_interval, 840000);
        assert!(!mainnet_params.pow_allow_min_difficulty_blocks);

        let regtest_params = BitcoinZConsensusParams::regtest();
        assert!(regtest_params.pow_allow_min_difficulty_blocks);
        assert!(regtest_params.pow_no_retargeting);
    }

    #[test]
    fn test_magic_bytes() {
        assert_eq!(get_magic_bytes(BitcoinZNetworkType::Mainnet), BITCOINZ_MAINNET_MAGIC);
        assert_eq!(parse_network_from_magic(BITCOINZ_MAINNET_MAGIC), Some(BitcoinZNetworkType::Mainnet));
        assert_eq!(parse_network_from_magic(0x12345678), None);
    }
}
