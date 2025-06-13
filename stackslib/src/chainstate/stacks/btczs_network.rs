// BTCZS Network Configuration
// This module implements network-specific configurations for BTCZS

use serde::{Deserialize, Serialize};
use stacks_common::types::chainstate::StacksAddress;
use stacks_common::util::hash::Hash160;

use crate::burnchains::bitcoinz::BitcoinZNetworkType;
use crate::chainstate::stacks::btczs_token::{BTCZS_TOTAL_SUPPLY, BTCZS_GENESIS_REWARD, BTCZS_HALVING_INTERVAL};
use crate::chainstate::stacks::Error as ChainstateError;

/// BTCZS network types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BTCZSNetworkType {
    /// BTCZS Mainnet - production network
    Mainnet,
    /// BTCZS Testnet - testing network
    Testnet,
    /// BTCZS Regtest - local development network
    Regtest,
    /// BTCZS Devnet - development network with custom parameters
    Devnet,
}

impl BTCZSNetworkType {
    /// Get the corresponding BitcoinZ network type
    pub fn to_bitcoinz_network(&self) -> BitcoinZNetworkType {
        match self {
            BTCZSNetworkType::Mainnet => BitcoinZNetworkType::Mainnet,
            BTCZSNetworkType::Testnet => BitcoinZNetworkType::Testnet,
            BTCZSNetworkType::Regtest => BitcoinZNetworkType::Regtest,
            BTCZSNetworkType::Devnet => BitcoinZNetworkType::Testnet, // Use testnet for devnet
        }
    }

    /// Get network magic bytes
    pub fn magic_bytes(&self) -> [u8; 4] {
        match self {
            BTCZSNetworkType::Mainnet => [0x24, 0xe9, 0x27, 0x64], // "BTCZ" in hex
            BTCZSNetworkType::Testnet => [0x74, 0x42, 0x54, 0x43], // "tBTC" in hex
            BTCZSNetworkType::Regtest => [0x72, 0x42, 0x54, 0x43], // "rBTC" in hex
            BTCZSNetworkType::Devnet => [0x64, 0x42, 0x54, 0x43],  // "dBTC" in hex
        }
    }

    /// Get network name
    pub fn name(&self) -> &'static str {
        match self {
            BTCZSNetworkType::Mainnet => "mainnet",
            BTCZSNetworkType::Testnet => "testnet",
            BTCZSNetworkType::Regtest => "regtest",
            BTCZSNetworkType::Devnet => "devnet",
        }
    }

    /// Get default RPC port
    pub fn default_rpc_port(&self) -> u16 {
        match self {
            BTCZSNetworkType::Mainnet => 20443,
            BTCZSNetworkType::Testnet => 20444,
            BTCZSNetworkType::Regtest => 20445,
            BTCZSNetworkType::Devnet => 20446,
        }
    }

    /// Get default P2P port
    pub fn default_p2p_port(&self) -> u16 {
        match self {
            BTCZSNetworkType::Mainnet => 20444,
            BTCZSNetworkType::Testnet => 20445,
            BTCZSNetworkType::Regtest => 20446,
            BTCZSNetworkType::Devnet => 20447,
        }
    }
}

/// BTCZS network configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BTCZSNetworkConfig {
    /// Network type
    pub network_type: BTCZSNetworkType,
    /// Chain ID for transactions
    pub chain_id: u32,
    /// Network magic bytes
    pub magic_bytes: [u8; 4],
    /// Genesis block configuration
    pub genesis_config: BTCZSGenesisConfig,
    /// Consensus parameters
    pub consensus_params: BTCZSConsensusParams,
    /// Network endpoints
    pub network_endpoints: BTCZSNetworkEndpoints,
    /// Fee configuration
    pub fee_config: BTCZSFeeConfig,
}

/// BTCZS genesis block configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BTCZSGenesisConfig {
    /// Genesis timestamp
    pub genesis_timestamp: u64,
    /// Genesis block hash
    pub genesis_block_hash: [u8; 32],
    /// Initial token distribution
    pub initial_distribution: Vec<(StacksAddress, u128)>,
    /// Genesis miners
    pub genesis_miners: Vec<StacksAddress>,
}

/// BTCZS consensus parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BTCZSConsensusParams {
    /// Target block time in seconds
    pub target_block_time: u64,
    /// Difficulty adjustment interval in blocks
    pub difficulty_adjustment_interval: u64,
    /// Maximum block size in bytes
    pub max_block_size: u64,
    /// Reward cycle length in blocks
    pub reward_cycle_length: u64,
    /// Prepare cycle length in blocks
    pub prepare_cycle_length: u64,
    /// Minimum burn amount for operations
    pub min_burn_amount: u64,
    /// Stacking threshold (minimum percentage of supply to enable stacking)
    pub stacking_threshold_percent: u8,
}

/// BTCZS network endpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BTCZSNetworkEndpoints {
    /// RPC endpoint
    pub rpc_endpoint: String,
    /// P2P endpoint
    pub p2p_endpoint: String,
    /// BitcoinZ RPC endpoint
    pub bitcoinz_rpc_endpoint: String,
    /// Bootstrap nodes
    pub bootstrap_nodes: Vec<String>,
}

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
}

impl BTCZSNetworkConfig {
    /// Create mainnet configuration
    pub fn mainnet() -> Self {
        BTCZSNetworkConfig {
            network_type: BTCZSNetworkType::Mainnet,
            chain_id: 0x80000000,
            magic_bytes: BTCZSNetworkType::Mainnet.magic_bytes(),
            genesis_config: BTCZSGenesisConfig::mainnet(),
            consensus_params: BTCZSConsensusParams::mainnet(),
            network_endpoints: BTCZSNetworkEndpoints::mainnet(),
            fee_config: BTCZSFeeConfig::mainnet(),
        }
    }

    /// Create testnet configuration
    pub fn testnet() -> Self {
        BTCZSNetworkConfig {
            network_type: BTCZSNetworkType::Testnet,
            chain_id: 0x80000001,
            magic_bytes: BTCZSNetworkType::Testnet.magic_bytes(),
            genesis_config: BTCZSGenesisConfig::testnet(),
            consensus_params: BTCZSConsensusParams::testnet(),
            network_endpoints: BTCZSNetworkEndpoints::testnet(),
            fee_config: BTCZSFeeConfig::testnet(),
        }
    }

    /// Create regtest configuration
    pub fn regtest() -> Self {
        BTCZSNetworkConfig {
            network_type: BTCZSNetworkType::Regtest,
            chain_id: 0x80000002,
            magic_bytes: BTCZSNetworkType::Regtest.magic_bytes(),
            genesis_config: BTCZSGenesisConfig::regtest(),
            consensus_params: BTCZSConsensusParams::regtest(),
            network_endpoints: BTCZSNetworkEndpoints::regtest(),
            fee_config: BTCZSFeeConfig::regtest(),
        }
    }

    /// Create devnet configuration with custom parameters
    pub fn devnet(custom_params: Option<BTCZSConsensusParams>) -> Self {
        BTCZSNetworkConfig {
            network_type: BTCZSNetworkType::Devnet,
            chain_id: 0x80000003,
            magic_bytes: BTCZSNetworkType::Devnet.magic_bytes(),
            genesis_config: BTCZSGenesisConfig::devnet(),
            consensus_params: custom_params.unwrap_or_else(BTCZSConsensusParams::devnet),
            network_endpoints: BTCZSNetworkEndpoints::devnet(),
            fee_config: BTCZSFeeConfig::devnet(),
        }
    }

    /// Validate network configuration
    pub fn validate(&self) -> Result<(), ChainstateError> {
        // Validate chain ID
        if self.chain_id == 0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Invalid chain ID: cannot be zero".to_string()
            ));
        }

        // Validate consensus parameters
        self.consensus_params.validate()?;

        // Validate genesis configuration
        self.genesis_config.validate()?;

        // Validate fee configuration
        self.fee_config.validate()?;

        Ok(())
    }

    /// Get network identifier string
    pub fn network_id(&self) -> String {
        format!("btczs-{}", self.network_type.name())
    }

    /// Check if this is a production network
    pub fn is_production(&self) -> bool {
        matches!(self.network_type, BTCZSNetworkType::Mainnet)
    }

    /// Check if this is a test network
    pub fn is_test_network(&self) -> bool {
        !self.is_production()
    }
}

impl BTCZSGenesisConfig {
    /// Create mainnet genesis configuration
    pub fn mainnet() -> Self {
        BTCZSGenesisConfig {
            genesis_timestamp: 1640995200, // January 1, 2022 00:00:00 UTC
            genesis_block_hash: [0x00; 32], // Will be set during genesis block creation
            initial_distribution: Self::create_mainnet_distribution(),
            genesis_miners: Self::create_mainnet_miners(),
        }
    }

    /// Create testnet genesis configuration
    pub fn testnet() -> Self {
        BTCZSGenesisConfig {
            genesis_timestamp: 1640995200,
            genesis_block_hash: [0x01; 32],
            initial_distribution: Self::create_testnet_distribution(),
            genesis_miners: Self::create_testnet_miners(),
        }
    }

    /// Create regtest genesis configuration
    pub fn regtest() -> Self {
        BTCZSGenesisConfig {
            genesis_timestamp: 1640995200,
            genesis_block_hash: [0x02; 32],
            initial_distribution: Self::create_regtest_distribution(),
            genesis_miners: Self::create_regtest_miners(),
        }
    }

    /// Create devnet genesis configuration
    pub fn devnet() -> Self {
        BTCZSGenesisConfig {
            genesis_timestamp: 1640995200,
            genesis_block_hash: [0x03; 32],
            initial_distribution: Self::create_devnet_distribution(),
            genesis_miners: Self::create_devnet_miners(),
        }
    }

    /// Validate genesis configuration
    pub fn validate(&self) -> Result<(), ChainstateError> {
        // Validate timestamp
        if self.genesis_timestamp == 0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Invalid genesis timestamp".to_string()
            ));
        }

        // Validate initial distribution
        let total_distributed: u128 = self.initial_distribution.iter()
            .map(|(_, amount)| *amount)
            .sum();
        
        if total_distributed > BTCZS_TOTAL_SUPPLY {
            return Err(ChainstateError::InvalidStacksBlock(
                "Initial distribution exceeds total supply".to_string()
            ));
        }

        // Validate miners
        if self.genesis_miners.is_empty() {
            return Err(ChainstateError::InvalidStacksBlock(
                "No genesis miners specified".to_string()
            ));
        }

        Ok(())
    }

    /// Create mainnet initial distribution
    fn create_mainnet_distribution() -> Vec<(StacksAddress, u128)> {
        // TODO: Replace with actual mainnet addresses
        vec![
            // Development fund (10%)
            (StacksAddress::new(0, Hash160([1u8; 20])).unwrap(), BTCZS_TOTAL_SUPPLY / 10),
            // Community fund (20%)
            (StacksAddress::new(0, Hash160([2u8; 20])).unwrap(), BTCZS_TOTAL_SUPPLY / 5),
        ]
    }

    /// Create testnet initial distribution
    fn create_testnet_distribution() -> Vec<(StacksAddress, u128)> {
        vec![
            (StacksAddress::new(1, Hash160([1u8; 20])).unwrap(), BTCZS_TOTAL_SUPPLY / 10),
            (StacksAddress::new(1, Hash160([2u8; 20])).unwrap(), BTCZS_TOTAL_SUPPLY / 5),
        ]
    }

    /// Create regtest initial distribution
    fn create_regtest_distribution() -> Vec<(StacksAddress, u128)> {
        vec![
            (StacksAddress::new(2, Hash160([1u8; 20])).unwrap(), BTCZS_TOTAL_SUPPLY / 2),
        ]
    }

    /// Create devnet initial distribution
    fn create_devnet_distribution() -> Vec<(StacksAddress, u128)> {
        vec![
            (StacksAddress::new(3, Hash160([1u8; 20])).unwrap(), BTCZS_TOTAL_SUPPLY / 2),
        ]
    }

    /// Create mainnet genesis miners
    fn create_mainnet_miners() -> Vec<StacksAddress> {
        // TODO: Replace with actual mainnet miner addresses
        vec![
            StacksAddress::new(0, Hash160([10u8; 20])).unwrap(),
            StacksAddress::new(0, Hash160([11u8; 20])).unwrap(),
            StacksAddress::new(0, Hash160([12u8; 20])).unwrap(),
        ]
    }

    /// Create testnet genesis miners
    fn create_testnet_miners() -> Vec<StacksAddress> {
        vec![
            StacksAddress::new(1, Hash160([10u8; 20])).unwrap(),
            StacksAddress::new(1, Hash160([11u8; 20])).unwrap(),
        ]
    }

    /// Create regtest genesis miners
    fn create_regtest_miners() -> Vec<StacksAddress> {
        vec![
            StacksAddress::new(2, Hash160([10u8; 20])).unwrap(),
        ]
    }

    /// Create devnet genesis miners
    fn create_devnet_miners() -> Vec<StacksAddress> {
        vec![
            StacksAddress::new(3, Hash160([10u8; 20])).unwrap(),
        ]
    }
}

impl BTCZSConsensusParams {
    /// Create mainnet consensus parameters
    pub fn mainnet() -> Self {
        BTCZSConsensusParams {
            target_block_time: 150, // 2.5 minutes (same as BitcoinZ)
            difficulty_adjustment_interval: 2016, // 2 weeks worth of blocks
            max_block_size: 2_000_000, // 2MB
            reward_cycle_length: 8064, // ~2 weeks at 2.5min blocks (2016 * 4)
            prepare_cycle_length: 400, // ~16 hours preparation at 2.5min blocks
            min_burn_amount: 5000, // 5000 zatoshis minimum burn
            stacking_threshold_percent: 25, // 25% of supply needed for stacking
        }
    }

    /// Create testnet consensus parameters
    pub fn testnet() -> Self {
        BTCZSConsensusParams {
            target_block_time: 60, // 1 minute for faster testing (faster than mainnet's 2.5min)
            difficulty_adjustment_interval: 144, // 1 day worth of blocks
            max_block_size: 2_000_000,
            reward_cycle_length: 1440, // ~1 day at 1min blocks
            prepare_cycle_length: 10, // ~20 minutes preparation
            min_burn_amount: 1000, // Lower minimum for testing
            stacking_threshold_percent: 10, // Lower threshold for testing
        }
    }

    /// Create regtest consensus parameters
    pub fn regtest() -> Self {
        BTCZSConsensusParams {
            target_block_time: 10, // 10 seconds for rapid development
            difficulty_adjustment_interval: 10, // Adjust every 10 blocks
            max_block_size: 2_000_000,
            reward_cycle_length: 10, // Very short cycles
            prepare_cycle_length: 2, // Minimal preparation
            min_burn_amount: 100, // Very low minimum
            stacking_threshold_percent: 1, // Very low threshold
        }
    }

    /// Create devnet consensus parameters
    pub fn devnet() -> Self {
        BTCZSConsensusParams {
            target_block_time: 30, // 30 seconds for development
            difficulty_adjustment_interval: 20, // Adjust every 20 blocks
            max_block_size: 2_000_000,
            reward_cycle_length: 20, // Short cycles for testing
            prepare_cycle_length: 2, // Minimal preparation
            min_burn_amount: 500, // Low minimum for development
            stacking_threshold_percent: 5, // Low threshold for development
        }
    }

    /// Validate consensus parameters
    pub fn validate(&self) -> Result<(), ChainstateError> {
        if self.target_block_time == 0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Target block time cannot be zero".to_string()
            ));
        }

        if self.difficulty_adjustment_interval == 0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Difficulty adjustment interval cannot be zero".to_string()
            ));
        }

        if self.max_block_size == 0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Max block size cannot be zero".to_string()
            ));
        }

        if self.reward_cycle_length == 0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Reward cycle length cannot be zero".to_string()
            ));
        }

        if self.prepare_cycle_length >= self.reward_cycle_length {
            return Err(ChainstateError::InvalidStacksBlock(
                "Prepare cycle length must be less than reward cycle length".to_string()
            ));
        }

        if self.stacking_threshold_percent > 100 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Stacking threshold cannot exceed 100%".to_string()
            ));
        }

        Ok(())
    }
}

impl BTCZSNetworkEndpoints {
    /// Create mainnet network endpoints
    pub fn mainnet() -> Self {
        BTCZSNetworkEndpoints {
            rpc_endpoint: "https://rpc.btczs.org".to_string(),
            p2p_endpoint: "btczs.org:20444".to_string(),
            bitcoinz_rpc_endpoint: "https://bitcoinz-rpc.btczs.org".to_string(),
            bootstrap_nodes: vec![
                "seed1.btczs.org:20444".to_string(),
                "seed2.btczs.org:20444".to_string(),
                "seed3.btczs.org:20444".to_string(),
            ],
        }
    }

    /// Create testnet network endpoints
    pub fn testnet() -> Self {
        BTCZSNetworkEndpoints {
            rpc_endpoint: "https://testnet-rpc.btczs.org".to_string(),
            p2p_endpoint: "testnet.btczs.org:20445".to_string(),
            bitcoinz_rpc_endpoint: "https://testnet-bitcoinz-rpc.btczs.org".to_string(),
            bootstrap_nodes: vec![
                "testnet-seed1.btczs.org:20445".to_string(),
                "testnet-seed2.btczs.org:20445".to_string(),
            ],
        }
    }

    /// Create regtest network endpoints
    pub fn regtest() -> Self {
        BTCZSNetworkEndpoints {
            rpc_endpoint: "http://localhost:20445".to_string(),
            p2p_endpoint: "localhost:20446".to_string(),
            bitcoinz_rpc_endpoint: "http://localhost:1979".to_string(), // Local BitcoinZ node
            bootstrap_nodes: vec![],
        }
    }

    /// Create devnet network endpoints
    pub fn devnet() -> Self {
        BTCZSNetworkEndpoints {
            rpc_endpoint: "http://localhost:20446".to_string(),
            p2p_endpoint: "localhost:20447".to_string(),
            bitcoinz_rpc_endpoint: "http://localhost:1979".to_string(),
            bootstrap_nodes: vec![],
        }
    }
}

impl BTCZSFeeConfig {
    /// Create mainnet fee configuration
    pub fn mainnet() -> Self {
        BTCZSFeeConfig {
            base_fee_rate: 100, // 100 microBTCZS per byte
            min_fee: 1000,      // 0.001 BTCZS minimum
            max_fee: 1000 * 1_000_000, // 1000 BTCZS maximum
            bitcoinz_operation_multiplier: 1.5,
        }
    }

    /// Create testnet fee configuration
    pub fn testnet() -> Self {
        BTCZSFeeConfig {
            base_fee_rate: 50,  // Lower fees for testing
            min_fee: 500,
            max_fee: 100 * 1_000_000, // 100 BTCZS maximum
            bitcoinz_operation_multiplier: 1.2,
        }
    }

    /// Create regtest fee configuration
    pub fn regtest() -> Self {
        BTCZSFeeConfig {
            base_fee_rate: 10,  // Very low fees for development
            min_fee: 100,
            max_fee: 10 * 1_000_000, // 10 BTCZS maximum
            bitcoinz_operation_multiplier: 1.0,
        }
    }

    /// Create devnet fee configuration
    pub fn devnet() -> Self {
        BTCZSFeeConfig {
            base_fee_rate: 25,  // Low fees for development
            min_fee: 250,
            max_fee: 50 * 1_000_000, // 50 BTCZS maximum
            bitcoinz_operation_multiplier: 1.1,
        }
    }

    /// Validate fee configuration
    pub fn validate(&self) -> Result<(), ChainstateError> {
        if self.base_fee_rate == 0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Base fee rate cannot be zero".to_string()
            ));
        }

        if self.min_fee == 0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "Minimum fee cannot be zero".to_string()
            ));
        }

        if self.max_fee <= self.min_fee {
            return Err(ChainstateError::InvalidStacksBlock(
                "Maximum fee must be greater than minimum fee".to_string()
            ));
        }

        if self.bitcoinz_operation_multiplier <= 0.0 {
            return Err(ChainstateError::InvalidStacksBlock(
                "BitcoinZ operation multiplier must be positive".to_string()
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_types() {
        // Test network type properties
        assert_eq!(BTCZSNetworkType::Mainnet.name(), "mainnet");
        assert_eq!(BTCZSNetworkType::Testnet.name(), "testnet");
        assert_eq!(BTCZSNetworkType::Regtest.name(), "regtest");
        assert_eq!(BTCZSNetworkType::Devnet.name(), "devnet");

        // Test magic bytes are unique
        let mainnet_magic = BTCZSNetworkType::Mainnet.magic_bytes();
        let testnet_magic = BTCZSNetworkType::Testnet.magic_bytes();
        let regtest_magic = BTCZSNetworkType::Regtest.magic_bytes();
        let devnet_magic = BTCZSNetworkType::Devnet.magic_bytes();

        assert_ne!(mainnet_magic, testnet_magic);
        assert_ne!(mainnet_magic, regtest_magic);
        assert_ne!(mainnet_magic, devnet_magic);
        assert_ne!(testnet_magic, regtest_magic);

        // Test port assignments are unique
        assert_ne!(BTCZSNetworkType::Mainnet.default_rpc_port(),
                   BTCZSNetworkType::Testnet.default_rpc_port());
        assert_ne!(BTCZSNetworkType::Mainnet.default_p2p_port(),
                   BTCZSNetworkType::Testnet.default_p2p_port());
    }

    #[test]
    fn test_bitcoinz_network_mapping() {
        // Test BitcoinZ network mapping
        assert_eq!(BTCZSNetworkType::Mainnet.to_bitcoinz_network(),
                   BitcoinZNetworkType::Mainnet);
        assert_eq!(BTCZSNetworkType::Testnet.to_bitcoinz_network(),
                   BitcoinZNetworkType::Testnet);
        assert_eq!(BTCZSNetworkType::Regtest.to_bitcoinz_network(),
                   BitcoinZNetworkType::Regtest);
        assert_eq!(BTCZSNetworkType::Devnet.to_bitcoinz_network(),
                   BitcoinZNetworkType::Testnet);
    }

    #[test]
    fn test_network_config_creation() {
        // Test mainnet config
        let mainnet = BTCZSNetworkConfig::mainnet();
        assert_eq!(mainnet.network_type, BTCZSNetworkType::Mainnet);
        assert_eq!(mainnet.chain_id, 0x80000000);
        assert!(mainnet.is_production());
        assert!(!mainnet.is_test_network());

        // Test testnet config
        let testnet = BTCZSNetworkConfig::testnet();
        assert_eq!(testnet.network_type, BTCZSNetworkType::Testnet);
        assert_eq!(testnet.chain_id, 0x80000001);
        assert!(!testnet.is_production());
        assert!(testnet.is_test_network());

        // Test regtest config
        let regtest = BTCZSNetworkConfig::regtest();
        assert_eq!(regtest.network_type, BTCZSNetworkType::Regtest);
        assert_eq!(regtest.chain_id, 0x80000002);

        // Test devnet config
        let devnet = BTCZSNetworkConfig::devnet(None);
        assert_eq!(devnet.network_type, BTCZSNetworkType::Devnet);
        assert_eq!(devnet.chain_id, 0x80000003);
    }

    #[test]
    fn test_network_config_validation() {
        // Test valid configurations
        assert!(BTCZSNetworkConfig::mainnet().validate().is_ok());
        assert!(BTCZSNetworkConfig::testnet().validate().is_ok());
        assert!(BTCZSNetworkConfig::regtest().validate().is_ok());
        assert!(BTCZSNetworkConfig::devnet(None).validate().is_ok());

        // Test invalid chain ID
        let mut invalid_config = BTCZSNetworkConfig::mainnet();
        invalid_config.chain_id = 0;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_consensus_params() {
        let mainnet_params = BTCZSConsensusParams::mainnet();
        let testnet_params = BTCZSConsensusParams::testnet();
        let regtest_params = BTCZSConsensusParams::regtest();

        // Test mainnet has longer block times
        assert!(mainnet_params.target_block_time > testnet_params.target_block_time);
        assert!(testnet_params.target_block_time > regtest_params.target_block_time);

        // Test all configurations are valid
        assert!(mainnet_params.validate().is_ok());
        assert!(testnet_params.validate().is_ok());
        assert!(regtest_params.validate().is_ok());

        // Test validation catches invalid parameters
        let mut invalid_params = BTCZSConsensusParams::mainnet();
        invalid_params.target_block_time = 0;
        assert!(invalid_params.validate().is_err());

        invalid_params = BTCZSConsensusParams::mainnet();
        invalid_params.prepare_cycle_length = invalid_params.reward_cycle_length;
        assert!(invalid_params.validate().is_err());

        invalid_params = BTCZSConsensusParams::mainnet();
        invalid_params.stacking_threshold_percent = 101;
        assert!(invalid_params.validate().is_err());
    }

    #[test]
    fn test_genesis_config() {
        let mainnet_genesis = BTCZSGenesisConfig::mainnet();
        let testnet_genesis = BTCZSGenesisConfig::testnet();

        // Test genesis configurations are valid
        assert!(mainnet_genesis.validate().is_ok());
        assert!(testnet_genesis.validate().is_ok());

        // Test genesis has initial distribution
        assert!(!mainnet_genesis.initial_distribution.is_empty());
        assert!(!mainnet_genesis.genesis_miners.is_empty());

        // Test total distribution doesn't exceed supply
        let total_distributed: u128 = mainnet_genesis.initial_distribution.iter()
            .map(|(_, amount)| *amount)
            .sum();
        assert!(total_distributed <= BTCZS_TOTAL_SUPPLY);

        // Test validation catches invalid genesis
        let mut invalid_genesis = BTCZSGenesisConfig::mainnet();
        invalid_genesis.genesis_timestamp = 0;
        assert!(invalid_genesis.validate().is_err());

        invalid_genesis = BTCZSGenesisConfig::mainnet();
        invalid_genesis.genesis_miners.clear();
        assert!(invalid_genesis.validate().is_err());
    }

    #[test]
    fn test_network_endpoints() {
        let mainnet_endpoints = BTCZSNetworkEndpoints::mainnet();
        let testnet_endpoints = BTCZSNetworkEndpoints::testnet();
        let regtest_endpoints = BTCZSNetworkEndpoints::regtest();

        // Test mainnet has production endpoints
        assert!(mainnet_endpoints.rpc_endpoint.contains("btczs.org"));
        assert!(!mainnet_endpoints.bootstrap_nodes.is_empty());

        // Test testnet has testnet endpoints
        assert!(testnet_endpoints.rpc_endpoint.contains("testnet"));

        // Test regtest has localhost endpoints
        assert!(regtest_endpoints.rpc_endpoint.contains("localhost"));
        assert!(regtest_endpoints.bootstrap_nodes.is_empty());
    }

    #[test]
    fn test_fee_config() {
        let mainnet_fees = BTCZSFeeConfig::mainnet();
        let testnet_fees = BTCZSFeeConfig::testnet();
        let regtest_fees = BTCZSFeeConfig::regtest();

        // Test fee configurations are valid
        assert!(mainnet_fees.validate().is_ok());
        assert!(testnet_fees.validate().is_ok());
        assert!(regtest_fees.validate().is_ok());

        // Test mainnet has higher fees than test networks
        assert!(mainnet_fees.base_fee_rate >= testnet_fees.base_fee_rate);
        assert!(testnet_fees.base_fee_rate >= regtest_fees.base_fee_rate);

        // Test validation catches invalid fees
        let mut invalid_fees = BTCZSFeeConfig::mainnet();
        invalid_fees.base_fee_rate = 0;
        assert!(invalid_fees.validate().is_err());

        invalid_fees = BTCZSFeeConfig::mainnet();
        invalid_fees.max_fee = invalid_fees.min_fee;
        assert!(invalid_fees.validate().is_err());

        invalid_fees = BTCZSFeeConfig::mainnet();
        invalid_fees.bitcoinz_operation_multiplier = 0.0;
        assert!(invalid_fees.validate().is_err());
    }

    #[test]
    fn test_network_identifiers() {
        let mainnet = BTCZSNetworkConfig::mainnet();
        let testnet = BTCZSNetworkConfig::testnet();
        let regtest = BTCZSNetworkConfig::regtest();
        let devnet = BTCZSNetworkConfig::devnet(None);

        assert_eq!(mainnet.network_id(), "btczs-mainnet");
        assert_eq!(testnet.network_id(), "btczs-testnet");
        assert_eq!(regtest.network_id(), "btczs-regtest");
        assert_eq!(devnet.network_id(), "btczs-devnet");
    }

    #[test]
    fn test_custom_devnet_params() {
        let custom_params = BTCZSConsensusParams {
            target_block_time: 5,
            difficulty_adjustment_interval: 5,
            max_block_size: 1_000_000,
            reward_cycle_length: 5,
            prepare_cycle_length: 1,
            min_burn_amount: 50,
            stacking_threshold_percent: 1,
        };

        let devnet = BTCZSNetworkConfig::devnet(Some(custom_params.clone()));
        assert_eq!(devnet.consensus_params.target_block_time, 5);
        assert_eq!(devnet.consensus_params.reward_cycle_length, 5);
        assert!(devnet.validate().is_ok());
    }
}
