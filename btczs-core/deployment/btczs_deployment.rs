// BTCZS Production Deployment Configuration
// This module implements deployment configurations and validation for BTCZS production

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::chainstate::stacks::btczs_network::{BTCZSNetworkConfig, BTCZSNetworkType};

/// BTCZS deployment environment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BTCZSDeploymentEnvironment {
    /// Production deployment
    Production,
    /// Staging deployment for final testing
    Staging,
    /// Development deployment
    Development,
    /// Local testing deployment
    Local,
}

impl BTCZSDeploymentEnvironment {
    /// Get the corresponding network type
    pub fn to_network_type(&self) -> BTCZSNetworkType {
        match self {
            BTCZSDeploymentEnvironment::Production => BTCZSNetworkType::Mainnet,
            BTCZSDeploymentEnvironment::Staging => BTCZSNetworkType::Testnet,
            BTCZSDeploymentEnvironment::Development => BTCZSNetworkType::Devnet,
            BTCZSDeploymentEnvironment::Local => BTCZSNetworkType::Regtest,
        }
    }

    /// Get environment name
    pub fn name(&self) -> &'static str {
        match self {
            BTCZSDeploymentEnvironment::Production => "production",
            BTCZSDeploymentEnvironment::Staging => "staging",
            BTCZSDeploymentEnvironment::Development => "development",
            BTCZSDeploymentEnvironment::Local => "local",
        }
    }
}

/// BTCZS deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTCZSDeploymentConfig {
    /// Deployment environment
    pub environment: BTCZSDeploymentEnvironment,
    /// Network configuration
    pub network_config: BTCZSNetworkConfig,
    /// Infrastructure configuration
    pub infrastructure: InfrastructureConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Backup configuration
    pub backup: BackupConfig,
}

/// Infrastructure deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureConfig {
    /// Number of validator nodes
    pub validator_nodes: u32,
    /// Number of seed nodes
    pub seed_nodes: u32,
    /// Number of RPC nodes
    pub rpc_nodes: u32,
    /// Load balancer configuration
    pub load_balancer: LoadBalancerConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Storage configuration
    pub storage: StorageConfig,
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Enable load balancing
    pub enabled: bool,
    /// Load balancer type
    pub lb_type: String,
    /// Health check interval in seconds
    pub health_check_interval: u64,
    /// Maximum connections per node
    pub max_connections_per_node: u32,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database type (e.g., "sqlite", "postgresql")
    pub db_type: String,
    /// Database connection string
    pub connection_string: String,
    /// Maximum connections
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Enable database replication
    pub replication_enabled: bool,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Data directory path
    pub data_dir: PathBuf,
    /// Maximum disk usage in GB
    pub max_disk_usage_gb: u64,
    /// Enable compression
    pub compression_enabled: bool,
    /// Retention policy in days
    pub retention_days: u32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// TLS configuration
    pub tls: TLSConfig,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,
    /// Firewall rules
    pub firewall_rules: Vec<FirewallRule>,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Certificate file path
    pub cert_file: PathBuf,
    /// Private key file path
    pub key_file: PathBuf,
    /// CA certificate file path
    pub ca_file: Option<PathBuf>,
    /// Minimum TLS version
    pub min_version: String,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Enable API key authentication
    pub api_key_enabled: bool,
    /// Enable JWT authentication
    pub jwt_enabled: bool,
    /// JWT secret key
    pub jwt_secret: Option<String>,
    /// Token expiration time in hours
    pub token_expiration_hours: u64,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst limit
    pub burst_limit: u32,
    /// Whitelist IPs
    pub whitelist_ips: Vec<String>,
}

/// Firewall rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    /// Rule name
    pub name: String,
    /// Source IP/CIDR
    pub source: String,
    /// Destination port
    pub port: u16,
    /// Protocol (tcp/udp)
    pub protocol: String,
    /// Action (allow/deny)
    pub action: String,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval: u64,
    /// Log level
    pub log_level: String,
    /// Log file path
    pub log_file: PathBuf,
    /// Enable alerting
    pub alerting_enabled: bool,
    /// Alert endpoints
    pub alert_endpoints: Vec<AlertEndpoint>,
}

/// Alert endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEndpoint {
    /// Endpoint name
    pub name: String,
    /// Endpoint type (email, slack, webhook)
    pub endpoint_type: String,
    /// Endpoint URL or address
    pub endpoint: String,
    /// Alert severity levels
    pub severity_levels: Vec<String>,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable backups
    pub enabled: bool,
    /// Backup interval in hours
    pub backup_interval_hours: u64,
    /// Backup retention in days
    pub retention_days: u32,
    /// Backup storage location
    pub storage_location: String,
    /// Enable encryption
    pub encryption_enabled: bool,
    /// Compression level (0-9)
    pub compression_level: u8,
}

impl BTCZSDeploymentConfig {
    /// Create production deployment configuration
    pub fn production() -> Self {
        BTCZSDeploymentConfig {
            environment: BTCZSDeploymentEnvironment::Production,
            network_config: BTCZSNetworkConfig::mainnet(),
            infrastructure: InfrastructureConfig::production(),
            security: SecurityConfig::production(),
            monitoring: MonitoringConfig::production(),
            backup: BackupConfig::production(),
        }
    }

    /// Create staging deployment configuration
    pub fn staging() -> Self {
        BTCZSDeploymentConfig {
            environment: BTCZSDeploymentEnvironment::Staging,
            network_config: BTCZSNetworkConfig::testnet(),
            infrastructure: InfrastructureConfig::staging(),
            security: SecurityConfig::staging(),
            monitoring: MonitoringConfig::staging(),
            backup: BackupConfig::staging(),
        }
    }

    /// Create development deployment configuration
    pub fn development() -> Self {
        BTCZSDeploymentConfig {
            environment: BTCZSDeploymentEnvironment::Development,
            network_config: BTCZSNetworkConfig::devnet(None),
            infrastructure: InfrastructureConfig::development(),
            security: SecurityConfig::development(),
            monitoring: MonitoringConfig::development(),
            backup: BackupConfig::development(),
        }
    }

    /// Create local deployment configuration
    pub fn local() -> Self {
        BTCZSDeploymentConfig {
            environment: BTCZSDeploymentEnvironment::Local,
            network_config: BTCZSNetworkConfig::regtest(),
            infrastructure: InfrastructureConfig::local(),
            security: SecurityConfig::local(),
            monitoring: MonitoringConfig::local(),
            backup: BackupConfig::local(),
        }
    }

    /// Validate deployment configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate network configuration
        self.network_config.validate()
            .map_err(|e| format!("Network config validation failed: {:?}", e))?;

        // Validate infrastructure
        self.infrastructure.validate()?;

        // Validate security
        self.security.validate()?;

        // Validate monitoring
        self.monitoring.validate()?;

        // Validate backup
        self.backup.validate()?;

        Ok(())
    }

    /// Get deployment summary
    pub fn get_summary(&self) -> DeploymentSummary {
        DeploymentSummary {
            environment: self.environment,
            network_type: self.network_config.network_type,
            total_nodes: self.infrastructure.validator_nodes 
                + self.infrastructure.seed_nodes 
                + self.infrastructure.rpc_nodes,
            security_enabled: self.security.tls.enabled,
            monitoring_enabled: self.monitoring.enabled,
            backup_enabled: self.backup.enabled,
        }
    }
}

/// Deployment summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSummary {
    pub environment: BTCZSDeploymentEnvironment,
    pub network_type: BTCZSNetworkType,
    pub total_nodes: u32,
    pub security_enabled: bool,
    pub monitoring_enabled: bool,
    pub backup_enabled: bool,
}

/// Deployment validation trait
pub trait DeploymentValidation {
    fn validate(&self) -> Result<(), String>;
}

impl DeploymentValidation for InfrastructureConfig {
    fn validate(&self) -> Result<(), String> {
        if self.validator_nodes == 0 {
            return Err("At least one validator node is required".to_string());
        }
        if self.seed_nodes == 0 {
            return Err("At least one seed node is required".to_string());
        }
        if self.rpc_nodes == 0 {
            return Err("At least one RPC node is required".to_string());
        }
        Ok(())
    }
}

impl DeploymentValidation for SecurityConfig {
    fn validate(&self) -> Result<(), String> {
        if self.tls.enabled {
            if !self.tls.cert_file.exists() {
                return Err("TLS certificate file not found".to_string());
            }
            if !self.tls.key_file.exists() {
                return Err("TLS private key file not found".to_string());
            }
        }
        Ok(())
    }
}

impl DeploymentValidation for MonitoringConfig {
    fn validate(&self) -> Result<(), String> {
        if self.enabled && self.metrics_interval == 0 {
            return Err("Metrics interval must be greater than 0".to_string());
        }
        Ok(())
    }
}

impl DeploymentValidation for BackupConfig {
    fn validate(&self) -> Result<(), String> {
        if self.enabled {
            if self.backup_interval_hours == 0 {
                return Err("Backup interval must be greater than 0".to_string());
            }
            if self.retention_days == 0 {
                return Err("Backup retention must be greater than 0".to_string());
            }
        }
        Ok(())
    }
}

impl InfrastructureConfig {
    /// Production infrastructure configuration
    pub fn production() -> Self {
        InfrastructureConfig {
            validator_nodes: 5,
            seed_nodes: 3,
            rpc_nodes: 3,
            load_balancer: LoadBalancerConfig {
                enabled: true,
                lb_type: "nginx".to_string(),
                health_check_interval: 30,
                max_connections_per_node: 1000,
            },
            database: DatabaseConfig {
                db_type: "postgresql".to_string(),
                connection_string: "postgresql://btczs:password@localhost:5432/btczs_mainnet".to_string(),
                max_connections: 100,
                connection_timeout: 30,
                replication_enabled: true,
            },
            storage: StorageConfig {
                data_dir: PathBuf::from("/var/lib/btczs"),
                max_disk_usage_gb: 1000,
                compression_enabled: true,
                retention_days: 365,
            },
        }
    }

    /// Staging infrastructure configuration
    pub fn staging() -> Self {
        InfrastructureConfig {
            validator_nodes: 3,
            seed_nodes: 2,
            rpc_nodes: 2,
            load_balancer: LoadBalancerConfig {
                enabled: true,
                lb_type: "nginx".to_string(),
                health_check_interval: 60,
                max_connections_per_node: 500,
            },
            database: DatabaseConfig {
                db_type: "postgresql".to_string(),
                connection_string: "postgresql://btczs:password@localhost:5432/btczs_testnet".to_string(),
                max_connections: 50,
                connection_timeout: 30,
                replication_enabled: false,
            },
            storage: StorageConfig {
                data_dir: PathBuf::from("/var/lib/btczs-staging"),
                max_disk_usage_gb: 500,
                compression_enabled: true,
                retention_days: 90,
            },
        }
    }

    /// Development infrastructure configuration
    pub fn development() -> Self {
        InfrastructureConfig {
            validator_nodes: 2,
            seed_nodes: 1,
            rpc_nodes: 1,
            load_balancer: LoadBalancerConfig {
                enabled: false,
                lb_type: "none".to_string(),
                health_check_interval: 120,
                max_connections_per_node: 100,
            },
            database: DatabaseConfig {
                db_type: "sqlite".to_string(),
                connection_string: "sqlite:///tmp/btczs_dev.db".to_string(),
                max_connections: 10,
                connection_timeout: 10,
                replication_enabled: false,
            },
            storage: StorageConfig {
                data_dir: PathBuf::from("/tmp/btczs-dev"),
                max_disk_usage_gb: 100,
                compression_enabled: false,
                retention_days: 30,
            },
        }
    }

    /// Local infrastructure configuration
    pub fn local() -> Self {
        InfrastructureConfig {
            validator_nodes: 1,
            seed_nodes: 1,
            rpc_nodes: 1,
            load_balancer: LoadBalancerConfig {
                enabled: false,
                lb_type: "none".to_string(),
                health_check_interval: 300,
                max_connections_per_node: 50,
            },
            database: DatabaseConfig {
                db_type: "sqlite".to_string(),
                connection_string: "sqlite:///tmp/btczs_local.db".to_string(),
                max_connections: 5,
                connection_timeout: 5,
                replication_enabled: false,
            },
            storage: StorageConfig {
                data_dir: PathBuf::from("/tmp/btczs-local"),
                max_disk_usage_gb: 10,
                compression_enabled: false,
                retention_days: 7,
            },
        }
    }
}
