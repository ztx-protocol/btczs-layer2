// Test program to validate BitcoinZ RPC integration
// This will test our RPC client against a local BitcoinZ node

use stackslib::burnchains::bitcoinz::rpc::{BitcoinZRpcClient, BitcoinZRpcConfig};
use stackslib::burnchains::bitcoinz::{BitcoinZNetworkType, get_bitcoinz_rpc_port};

fn main() {
    println!("🚀 Testing BitcoinZ RPC Integration");
    println!("=====================================");

    // Test different network configurations
    test_mainnet_connection();
    test_testnet_connection();
    test_regtest_connection();
}

fn test_mainnet_connection() {
    println!("\n📡 Testing BitcoinZ Mainnet Connection");
    println!("--------------------------------------");
    
    let config = BitcoinZRpcConfig::new(
        "127.0.0.1".to_string(),
        BitcoinZNetworkType::Mainnet,
        Some("any".to_string()),
        Some("any".to_string()),
    );
    
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);
    println!("Network: {:?}", config.network);
    
    let mut client = BitcoinZRpcClient::new(config);
    
    // Test basic connectivity
    match client.test_connection() {
        Ok(true) => {
            println!("✅ Connection successful!");
            test_rpc_calls(&mut client);
        }
        Ok(false) => println!("❌ Connection failed - node not responding"),
        Err(e) => println!("❌ Connection error: {:?}", e),
    }
}

fn test_testnet_connection() {
    println!("\n📡 Testing BitcoinZ Testnet Connection");
    println!("--------------------------------------");
    
    let config = BitcoinZRpcConfig::new(
        "127.0.0.1".to_string(),
        BitcoinZNetworkType::Testnet,
        Some("any".to_string()),
        Some("any".to_string()),
    );
    
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);
    println!("Network: {:?}", config.network);
    
    let mut client = BitcoinZRpcClient::new(config);
    
    match client.test_connection() {
        Ok(true) => {
            println!("✅ Connection successful!");
            test_rpc_calls(&mut client);
        }
        Ok(false) => println!("❌ Connection failed - node not responding"),
        Err(e) => println!("❌ Connection error: {:?}", e),
    }
}

fn test_regtest_connection() {
    println!("\n📡 Testing BitcoinZ Regtest Connection");
    println!("--------------------------------------");
    
    let config = BitcoinZRpcConfig::new(
        "127.0.0.1".to_string(),
        BitcoinZNetworkType::Regtest,
        Some("any".to_string()),
        Some("any".to_string()),
    );
    
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);
    println!("Network: {:?}", config.network);
    
    let mut client = BitcoinZRpcClient::new(config);
    
    match client.test_connection() {
        Ok(true) => {
            println!("✅ Connection successful!");
            test_rpc_calls(&mut client);
        }
        Ok(false) => println!("❌ Connection failed - node not responding"),
        Err(e) => println!("❌ Connection error: {:?}", e),
    }
}

fn test_rpc_calls(client: &mut BitcoinZRpcClient) {
    println!("\n🔍 Testing RPC Calls");
    println!("--------------------");
    
    // Test getblockchaininfo
    match client.get_blockchain_info() {
        Ok(info) => {
            println!("✅ getblockchaininfo: Success");
            if let Some(chain) = info.get("chain") {
                println!("   Chain: {}", chain);
            }
            if let Some(blocks) = info.get("blocks") {
                println!("   Blocks: {}", blocks);
            }
            if let Some(difficulty) = info.get("difficulty") {
                println!("   Difficulty: {}", difficulty);
            }
        }
        Err(e) => println!("❌ getblockchaininfo failed: {:?}", e),
    }
    
    // Test getblockcount
    match client.get_block_count() {
        Ok(count) => {
            println!("✅ getblockcount: {}", count);
            
            // Test getting recent blocks
            if count > 0 {
                test_block_retrieval(client, count);
            }
        }
        Err(e) => println!("❌ getblockcount failed: {:?}", e),
    }
    
    // Test getnetworkinfo
    match client.get_network_info() {
        Ok(info) => {
            println!("✅ getnetworkinfo: Success");
            if let Some(version) = info.get("version") {
                println!("   Version: {}", version);
            }
            if let Some(subversion) = info.get("subversion") {
                println!("   Subversion: {}", subversion);
            }
        }
        Err(e) => println!("❌ getnetworkinfo failed: {:?}", e),
    }
    
    // Test getmininginfo
    match client.get_mining_info() {
        Ok(info) => {
            println!("✅ getmininginfo: Success");
            if let Some(hashps) = info.get("networkhashps") {
                println!("   Network Hash/s: {}", hashps);
            }
        }
        Err(e) => println!("❌ getmininginfo failed: {:?}", e),
    }
}

fn test_block_retrieval(client: &mut BitcoinZRpcClient, block_count: u64) {
    println!("\n🧱 Testing Block Retrieval");
    println!("--------------------------");
    
    // Test getting the latest block
    let test_height = if block_count > 10 { block_count - 5 } else { 1 };
    
    match client.get_block_hash(test_height) {
        Ok(hash) => {
            println!("✅ Block hash at height {}: {}", test_height, hash);
            
            // Test getting block by hash
            match client.get_block(&hash, 1) {
                Ok(block) => {
                    println!("✅ Block retrieval successful");
                    if let Some(height) = block.get("height") {
                        println!("   Height: {}", height);
                    }
                    if let Some(time) = block.get("time") {
                        println!("   Time: {}", time);
                    }
                    if let Some(tx_count) = block.get("tx").and_then(|tx| tx.as_array()) {
                        println!("   Transactions: {}", tx_count.len());
                    }
                }
                Err(e) => println!("❌ Block retrieval failed: {:?}", e),
            }
        }
        Err(e) => println!("❌ Block hash retrieval failed: {:?}", e),
    }
    
    // Test getting block by height directly
    match client.get_block_by_height(test_height, 2) {
        Ok(block) => {
            println!("✅ Block by height retrieval successful");
            if let Some(confirmations) = block.get("confirmations") {
                println!("   Confirmations: {}", confirmations);
            }
        }
        Err(e) => println!("❌ Block by height retrieval failed: {:?}", e),
    }
}
