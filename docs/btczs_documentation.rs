// BTCZS Documentation Generation
// This module implements comprehensive documentation generation for BTCZS

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Documentation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentationType {
    /// Technical API documentation
    TechnicalAPI,
    /// User guide documentation
    UserGuide,
    /// Developer documentation
    Developer,
    /// Deployment guide
    Deployment,
    /// Security documentation
    Security,
    /// Architecture overview
    Architecture,
}

impl DocumentationType {
    /// Get documentation type name
    pub fn name(&self) -> &'static str {
        match self {
            DocumentationType::TechnicalAPI => "Technical API",
            DocumentationType::UserGuide => "User Guide",
            DocumentationType::Developer => "Developer Guide",
            DocumentationType::Deployment => "Deployment Guide",
            DocumentationType::Security => "Security Documentation",
            DocumentationType::Architecture => "Architecture Overview",
        }
    }

    /// Get file name for documentation type
    pub fn filename(&self) -> &'static str {
        match self {
            DocumentationType::TechnicalAPI => "technical-api.md",
            DocumentationType::UserGuide => "user-guide.md",
            DocumentationType::Developer => "developer-guide.md",
            DocumentationType::Deployment => "deployment-guide.md",
            DocumentationType::Security => "security-guide.md",
            DocumentationType::Architecture => "architecture.md",
        }
    }
}

/// Documentation section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSection {
    /// Section title
    pub title: String,
    /// Section content
    pub content: String,
    /// Subsections
    pub subsections: Vec<DocumentationSection>,
    /// Code examples
    pub code_examples: Vec<CodeExample>,
}

/// Code example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    /// Example title
    pub title: String,
    /// Programming language
    pub language: String,
    /// Code content
    pub code: String,
    /// Example description
    pub description: String,
}

/// Documentation generator
pub struct BTCZSDocumentationGenerator {
    /// Output directory
    output_dir: PathBuf,
    /// Generated documents
    documents: HashMap<DocumentationType, String>,
}

impl BTCZSDocumentationGenerator {
    /// Create a new documentation generator
    pub fn new(output_dir: PathBuf) -> Self {
        BTCZSDocumentationGenerator {
            output_dir,
            documents: HashMap::new(),
        }
    }

    /// Generate all documentation
    pub fn generate_all_documentation(&mut self) -> Result<(), std::io::Error> {
        // Generate each documentation type
        self.generate_technical_api_docs()?;
        self.generate_user_guide()?;
        self.generate_developer_guide()?;
        self.generate_deployment_guide()?;
        self.generate_security_documentation()?;
        self.generate_architecture_overview()?;

        // Write all documents to files
        self.write_documents_to_files()?;

        Ok(())
    }

    /// Generate technical API documentation
    fn generate_technical_api_docs(&mut self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        
        content.push_str("# BTCZS Technical API Documentation\n\n");
        content.push_str("## Overview\n\n");
        content.push_str("BTCZS (BitcoinZ Stacks) is a Layer 2 solution that uses BitcoinZ as a burnchain for the Stacks protocol.\n\n");
        
        content.push_str("## Core APIs\n\n");
        
        // BitcoinZ Integration API
        content.push_str("### BitcoinZ Integration API\n\n");
        content.push_str("#### BitcoinZ RPC Configuration\n\n");
        content.push_str("```rust\n");
        content.push_str("use btczs_core::burnchains::bitcoinz::rpc::BitcoinZRpcConfig;\n\n");
        content.push_str("let config = BitcoinZRpcConfig {\n");
        content.push_str("    endpoint: \"http://localhost:1979\".to_string(),\n");
        content.push_str("    username: \"user\".to_string(),\n");
        content.push_str("    password: \"pass\".to_string(),\n");
        content.push_str("    network: BitcoinZNetworkType::Mainnet,\n");
        content.push_str("    timeout: 30,\n");
        content.push_str("};\n");
        content.push_str("```\n\n");
        
        // Token Economics API
        content.push_str("### BTCZS Token Economics API\n\n");
        content.push_str("#### Token Balance Operations\n\n");
        content.push_str("```rust\n");
        content.push_str("use btczs_core::chainstate::stacks::btczs_token::BTCZSAccount;\n\n");
        content.push_str("// Get balance\n");
        content.push_str("let balance = BTCZSAccount::get_balance(&address, block_height)?;\n\n");
        content.push_str("// Transfer tokens\n");
        content.push_str("BTCZSAccount::transfer(&from, &to, amount, block_height)?;\n\n");
        content.push_str("// Lock for stacking\n");
        content.push_str("BTCZSAccount::lock_for_stacking(&address, amount, block_height)?;\n");
        content.push_str("```\n\n");
        
        // Stacking API
        content.push_str("### Stacking API\n\n");
        content.push_str("#### Stacking Operations\n\n");
        content.push_str("```rust\n");
        content.push_str("use btczs_core::chainstate::stacks::btczs_stacking::BTCZSStackingManager;\n\n");
        content.push_str("// Validate stacking operation\n");
        content.push_str("BTCZSStackingManager::validate_stacking_operation(\n");
        content.push_str("    &stacker,\n");
        content.push_str("    stacked_amount,\n");
        content.push_str("    &reward_address,\n");
        content.push_str("    lock_period,\n");
        content.push_str("    current_height,\n");
        content.push_str(")?;\n");
        content.push_str("```\n\n");
        
        // Network Configuration API
        content.push_str("### Network Configuration API\n\n");
        content.push_str("#### Network Setup\n\n");
        content.push_str("```rust\n");
        content.push_str("use btczs_core::chainstate::stacks::btczs_network::BTCZSNetworkConfig;\n\n");
        content.push_str("// Create mainnet configuration\n");
        content.push_str("let mainnet_config = BTCZSNetworkConfig::mainnet();\n\n");
        content.push_str("// Create testnet configuration\n");
        content.push_str("let testnet_config = BTCZSNetworkConfig::testnet();\n\n");
        content.push_str("// Validate configuration\n");
        content.push_str("config.validate()?;\n");
        content.push_str("```\n\n");
        
        content.push_str("## Error Handling\n\n");
        content.push_str("All BTCZS APIs use the `ChainstateError` type for error handling:\n\n");
        content.push_str("```rust\n");
        content.push_str("use btczs_core::chainstate::stacks::Error as ChainstateError;\n\n");
        content.push_str("match result {\n");
        content.push_str("    Ok(value) => println!(\"Success: {:?}\", value),\n");
        content.push_str("    Err(ChainstateError::InvalidStacksBlock(msg)) => {\n");
        content.push_str("        eprintln!(\"Invalid block: {}\", msg);\n");
        content.push_str("    }\n");
        content.push_str("    Err(e) => eprintln!(\"Error: {:?}\", e),\n");
        content.push_str("}\n");
        content.push_str("```\n\n");
        
        self.documents.insert(DocumentationType::TechnicalAPI, content);
        Ok(())
    }

    /// Generate user guide
    fn generate_user_guide(&mut self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        
        content.push_str("# BTCZS User Guide\n\n");
        content.push_str("## Introduction\n\n");
        content.push_str("Welcome to BTCZS (BitcoinZ Stacks), a Layer 2 solution that brings smart contracts and DeFi capabilities to the BitcoinZ ecosystem.\n\n");
        
        content.push_str("## Getting Started\n\n");
        content.push_str("### Prerequisites\n\n");
        content.push_str("- BitcoinZ wallet with BTCZ tokens\n");
        content.push_str("- BTCZS-compatible wallet\n");
        content.push_str("- Basic understanding of blockchain concepts\n\n");
        
        content.push_str("### Setting Up Your Wallet\n\n");
        content.push_str("1. Download a BTCZS-compatible wallet\n");
        content.push_str("2. Create a new wallet or import existing keys\n");
        content.push_str("3. Connect to the BTCZS network\n");
        content.push_str("4. Verify your connection status\n\n");
        
        content.push_str("### Getting BTCZS Tokens\n\n");
        content.push_str("BTCZS tokens can be obtained through:\n\n");
        content.push_str("- **Mining**: Participate in BTCZS mining by burning BTCZ\n");
        content.push_str("- **Stacking**: Earn BTCZS rewards by stacking STX tokens\n");
        content.push_str("- **Trading**: Exchange other tokens for BTCZS on supported exchanges\n\n");
        
        content.push_str("## Core Features\n\n");
        content.push_str("### Stacking\n\n");
        content.push_str("Stacking allows you to earn BTCZS rewards by locking up STX tokens:\n\n");
        content.push_str("1. **Choose Amount**: Minimum 1,000 BTCZS required\n");
        content.push_str("2. **Select Duration**: 1-12 reward cycles (each ~2 weeks)\n");
        content.push_str("3. **Set Reward Address**: BitcoinZ address for receiving rewards\n");
        content.push_str("4. **Confirm Transaction**: Submit stacking transaction\n\n");
        
        content.push_str("### Smart Contracts\n\n");
        content.push_str("BTCZS supports Clarity smart contracts:\n\n");
        content.push_str("- **Deploy Contracts**: Upload your smart contracts to BTCZS\n");
        content.push_str("- **Interact with DApps**: Use decentralized applications\n");
        content.push_str("- **DeFi Protocols**: Access lending, trading, and yield farming\n\n");
        
        content.push_str("### Token Transfers\n\n");
        content.push_str("Send and receive BTCZS tokens:\n\n");
        content.push_str("1. **Enter Recipient**: BTCZS address or .btczs name\n");
        content.push_str("2. **Specify Amount**: Amount in BTCZS tokens\n");
        content.push_str("3. **Set Fee**: Transaction fee (auto-calculated)\n");
        content.push_str("4. **Confirm**: Review and submit transaction\n\n");
        
        content.push_str("## Security Best Practices\n\n");
        content.push_str("- **Backup Your Keys**: Store private keys securely offline\n");
        content.push_str("- **Verify Addresses**: Always double-check recipient addresses\n");
        content.push_str("- **Use Hardware Wallets**: For large amounts, use hardware wallets\n");
        content.push_str("- **Keep Software Updated**: Use latest wallet versions\n");
        content.push_str("- **Be Cautious with DApps**: Only use trusted applications\n\n");
        
        content.push_str("## Troubleshooting\n\n");
        content.push_str("### Common Issues\n\n");
        content.push_str("**Transaction Stuck**: Check network congestion and fee amount\n");
        content.push_str("**Wallet Not Syncing**: Verify network connection and node status\n");
        content.push_str("**Stacking Not Working**: Ensure minimum amount and valid duration\n\n");
        
        content.push_str("### Getting Help\n\n");
        content.push_str("- **Documentation**: Check official BTCZS documentation\n");
        content.push_str("- **Community**: Join BTCZS Discord or Telegram\n");
        content.push_str("- **Support**: Contact official support channels\n\n");
        
        self.documents.insert(DocumentationType::UserGuide, content);
        Ok(())
    }

    /// Generate developer guide
    fn generate_developer_guide(&mut self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        
        content.push_str("# BTCZS Developer Guide\n\n");
        content.push_str("## Development Environment Setup\n\n");
        content.push_str("### Prerequisites\n\n");
        content.push_str("- Rust 1.70+ with Cargo\n");
        content.push_str("- BitcoinZ node (for testing)\n");
        content.push_str("- Git\n");
        content.push_str("- Docker (optional)\n\n");
        
        content.push_str("### Building BTCZS\n\n");
        content.push_str("```bash\n");
        content.push_str("# Clone the repository\n");
        content.push_str("git clone https://github.com/btczs/btczs-core.git\n");
        content.push_str("cd btczs-core\n\n");
        content.push_str("# Build the project\n");
        content.push_str("cargo build --release\n\n");
        content.push_str("# Run tests\n");
        content.push_str("cargo test\n");
        content.push_str("```\n\n");
        
        content.push_str("### Running a Local Node\n\n");
        content.push_str("```bash\n");
        content.push_str("# Start BitcoinZ node (regtest)\n");
        content.push_str("bitcoinzd -regtest -rpcuser=test -rpcpassword=test\n\n");
        content.push_str("# Start BTCZS node\n");
        content.push_str("./target/release/btczs-node --config=regtest.toml\n");
        content.push_str("```\n\n");
        
        content.push_str("## Smart Contract Development\n\n");
        content.push_str("### Clarity Language\n\n");
        content.push_str("BTCZS uses Clarity for smart contracts:\n\n");
        content.push_str("```clarity\n");
        content.push_str(";; Simple token contract\n");
        content.push_str("(define-fungible-token my-token)\n\n");
        content.push_str("(define-public (transfer (amount uint) (recipient principal))\n");
        content.push_str("  (ft-transfer? my-token amount tx-sender recipient))\n\n");
        content.push_str("(define-read-only (get-balance (account principal))\n");
        content.push_str("  (ft-get-balance my-token account))\n");
        content.push_str("```\n\n");
        
        content.push_str("### Contract Deployment\n\n");
        content.push_str("```bash\n");
        content.push_str("# Deploy contract\n");
        content.push_str("btczs-cli contract deploy my-contract contract.clar\n\n");
        content.push_str("# Call contract function\n");
        content.push_str("btczs-cli contract call my-contract transfer 100 SP123...\n");
        content.push_str("```\n\n");
        
        content.push_str("## Integration Examples\n\n");
        content.push_str("### Rust Integration\n\n");
        content.push_str("```rust\n");
        content.push_str("use btczs_core::chainstate::stacks::btczs_token::BTCZSAccount;\n");
        content.push_str("use btczs_core::chainstate::stacks::btczs_network::BTCZSNetworkConfig;\n\n");
        content.push_str("fn main() -> Result<(), Box<dyn std::error::Error>> {\n");
        content.push_str("    // Initialize network\n");
        content.push_str("    let config = BTCZSNetworkConfig::testnet();\n");
        content.push_str("    config.validate()?;\n\n");
        content.push_str("    // Get token balance\n");
        content.push_str("    let balance = BTCZSAccount::get_balance(&address, height)?;\n");
        content.push_str("    println!(\"Balance: {} BTCZS\", balance.total);\n\n");
        content.push_str("    Ok(())\n");
        content.push_str("}\n");
        content.push_str("```\n\n");
        
        content.push_str("## Testing\n\n");
        content.push_str("### Unit Tests\n\n");
        content.push_str("```bash\n");
        content.push_str("# Run all tests\n");
        content.push_str("cargo test\n\n");
        content.push_str("# Run specific test module\n");
        content.push_str("cargo test btczs_token\n\n");
        content.push_str("# Run with output\n");
        content.push_str("cargo test -- --nocapture\n");
        content.push_str("```\n\n");
        
        content.push_str("### Integration Tests\n\n");
        content.push_str("```bash\n");
        content.push_str("# Run integration tests\n");
        content.push_str("cargo test --test integration\n\n");
        content.push_str("# Run with BitcoinZ node\n");
        content.push_str("BITCOINZ_RPC_URL=http://localhost:1979 cargo test\n");
        content.push_str("```\n\n");
        
        content.push_str("## Contributing\n\n");
        content.push_str("### Code Style\n\n");
        content.push_str("- Follow Rust standard formatting (`cargo fmt`)\n");
        content.push_str("- Use meaningful variable and function names\n");
        content.push_str("- Add comprehensive documentation\n");
        content.push_str("- Include unit tests for new features\n\n");
        
        content.push_str("### Pull Request Process\n\n");
        content.push_str("1. Fork the repository\n");
        content.push_str("2. Create a feature branch\n");
        content.push_str("3. Make your changes with tests\n");
        content.push_str("4. Ensure all tests pass\n");
        content.push_str("5. Submit a pull request\n\n");
        
        self.documents.insert(DocumentationType::Developer, content);
        Ok(())
    }

    /// Generate deployment guide
    fn generate_deployment_guide(&mut self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        
        content.push_str("# BTCZS Deployment Guide\n\n");
        content.push_str("## Production Deployment\n\n");
        content.push_str("### Infrastructure Requirements\n\n");
        content.push_str("**Minimum Requirements:**\n");
        content.push_str("- 5 Validator nodes (8 CPU, 32GB RAM, 1TB SSD)\n");
        content.push_str("- 3 Seed nodes (4 CPU, 16GB RAM, 500GB SSD)\n");
        content.push_str("- 3 RPC nodes (8 CPU, 32GB RAM, 1TB SSD)\n");
        content.push_str("- Load balancer (2 CPU, 8GB RAM)\n");
        content.push_str("- PostgreSQL database (4 CPU, 16GB RAM, 500GB SSD)\n\n");
        
        content.push_str("### Security Configuration\n\n");
        content.push_str("```yaml\n");
        content.push_str("security:\n");
        content.push_str("  tls:\n");
        content.push_str("    enabled: true\n");
        content.push_str("    cert_file: /etc/ssl/certs/btczs.crt\n");
        content.push_str("    key_file: /etc/ssl/private/btczs.key\n");
        content.push_str("    min_version: \"1.3\"\n");
        content.push_str("  auth:\n");
        content.push_str("    api_key_enabled: true\n");
        content.push_str("    jwt_enabled: true\n");
        content.push_str("  rate_limiting:\n");
        content.push_str("    enabled: true\n");
        content.push_str("    requests_per_minute: 100\n");
        content.push_str("```\n\n");
        
        content.push_str("### Docker Deployment\n\n");
        content.push_str("```yaml\n");
        content.push_str("# docker-compose.yml\n");
        content.push_str("version: '3.8'\n");
        content.push_str("services:\n");
        content.push_str("  btczs-node:\n");
        content.push_str("    image: btczs/btczs-node:latest\n");
        content.push_str("    ports:\n");
        content.push_str("      - \"20443:20443\"\n");
        content.push_str("      - \"20444:20444\"\n");
        content.push_str("    volumes:\n");
        content.push_str("      - btczs-data:/var/lib/btczs\n");
        content.push_str("      - ./config:/etc/btczs\n");
        content.push_str("    environment:\n");
        content.push_str("      - BTCZS_NETWORK=mainnet\n");
        content.push_str("      - BITCOINZ_RPC_URL=http://bitcoinz:1979\n");
        content.push_str("```\n\n");
        
        content.push_str("### Monitoring Setup\n\n");
        content.push_str("```yaml\n");
        content.push_str("monitoring:\n");
        content.push_str("  enabled: true\n");
        content.push_str("  metrics_interval: 60\n");
        content.push_str("  log_level: info\n");
        content.push_str("  alerting_enabled: true\n");
        content.push_str("  alert_endpoints:\n");
        content.push_str("    - name: slack\n");
        content.push_str("      type: webhook\n");
        content.push_str("      endpoint: https://hooks.slack.com/...\n");
        content.push_str("```\n\n");
        
        self.documents.insert(DocumentationType::Deployment, content);
        Ok(())
    }

    /// Generate security documentation
    fn generate_security_documentation(&mut self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        
        content.push_str("# BTCZS Security Documentation\n\n");
        content.push_str("## Security Architecture\n\n");
        content.push_str("BTCZS implements multiple layers of security:\n\n");
        content.push_str("1. **Cryptographic Security**: Uses proven cryptographic algorithms\n");
        content.push_str("2. **Consensus Security**: PoX consensus with BitcoinZ finality\n");
        content.push_str("3. **Network Security**: TLS encryption and peer authentication\n");
        content.push_str("4. **Smart Contract Security**: Clarity language safety features\n\n");
        
        content.push_str("## Security Best Practices\n\n");
        content.push_str("### Node Security\n\n");
        content.push_str("- Use TLS for all communications\n");
        content.push_str("- Implement proper firewall rules\n");
        content.push_str("- Regular security updates\n");
        content.push_str("- Monitor for suspicious activity\n\n");
        
        content.push_str("### Key Management\n\n");
        content.push_str("- Use hardware security modules (HSMs)\n");
        content.push_str("- Implement key rotation policies\n");
        content.push_str("- Multi-signature controls for critical operations\n");
        content.push_str("- Secure backup and recovery procedures\n\n");
        
        self.documents.insert(DocumentationType::Security, content);
        Ok(())
    }

    /// Generate architecture overview
    fn generate_architecture_overview(&mut self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        
        content.push_str("# BTCZS Architecture Overview\n\n");
        content.push_str("## System Architecture\n\n");
        content.push_str("BTCZS consists of several key components:\n\n");
        content.push_str("### Core Components\n\n");
        content.push_str("1. **BitcoinZ Integration Layer**: Interfaces with BitcoinZ blockchain\n");
        content.push_str("2. **Consensus Engine**: Implements PoX consensus mechanism\n");
        content.push_str("3. **Token Economics**: Manages BTCZS token operations\n");
        content.push_str("4. **Smart Contract VM**: Executes Clarity smart contracts\n");
        content.push_str("5. **Network Layer**: Handles P2P communications\n\n");
        
        content.push_str("### Data Flow\n\n");
        content.push_str("```\n");
        content.push_str("BitcoinZ Blockchain\n");
        content.push_str("        ↓\n");
        content.push_str("BitcoinZ Integration Layer\n");
        content.push_str("        ↓\n");
        content.push_str("BTCZS Consensus Engine\n");
        content.push_str("        ↓\n");
        content.push_str("BTCZS Blockchain\n");
        content.push_str("```\n\n");
        
        self.documents.insert(DocumentationType::Architecture, content);
        Ok(())
    }

    /// Write all documents to files
    fn write_documents_to_files(&self) -> Result<(), std::io::Error> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)?;

        // Write each document to its file
        for (doc_type, content) in &self.documents {
            let file_path = self.output_dir.join(doc_type.filename());
            fs::write(file_path, content)?;
        }

        // Generate index file
        self.generate_index_file()?;

        Ok(())
    }

    /// Generate documentation index file
    fn generate_index_file(&self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        
        content.push_str("# BTCZS Documentation Index\n\n");
        content.push_str("Welcome to the BTCZS documentation. Choose a guide below:\n\n");
        
        for doc_type in [
            DocumentationType::UserGuide,
            DocumentationType::Developer,
            DocumentationType::TechnicalAPI,
            DocumentationType::Deployment,
            DocumentationType::Security,
            DocumentationType::Architecture,
        ] {
            content.push_str(&format!(
                "- [{}]({})\n",
                doc_type.name(),
                doc_type.filename()
            ));
        }
        
        content.push_str("\n## Quick Links\n\n");
        content.push_str("- [GitHub Repository](https://github.com/btczs/btczs-core)\n");
        content.push_str("- [Community Discord](https://discord.gg/btczs)\n");
        content.push_str("- [Official Website](https://btczs.org)\n");
        
        let index_path = self.output_dir.join("README.md");
        fs::write(index_path, content)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_documentation_generation() {
        let temp_dir = env::temp_dir().join("btczs-docs-test");
        let mut generator = BTCZSDocumentationGenerator::new(temp_dir.clone());
        
        // Generate documentation
        assert!(generator.generate_all_documentation().is_ok());
        
        // Check that files were created
        assert!(temp_dir.join("README.md").exists());
        assert!(temp_dir.join("user-guide.md").exists());
        assert!(temp_dir.join("developer-guide.md").exists());
        
        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_documentation_types() {
        assert_eq!(DocumentationType::TechnicalAPI.name(), "Technical API");
        assert_eq!(DocumentationType::UserGuide.filename(), "user-guide.md");
    }
}
