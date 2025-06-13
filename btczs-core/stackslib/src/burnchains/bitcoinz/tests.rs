// Integration tests for BitcoinZ RPC client
// Tests against local BitcoinZ node

#[cfg(test)]
mod bitcoinz_integration_tests {
    use super::super::rpc::{BitcoinZRpcClient, BitcoinZRpcConfig};
    use super::super::{BitcoinZNetworkType, get_bitcoinz_rpc_port};
    use super::super::indexer::{BitcoinZIndexer, BitcoinZIndexerConfig};

    fn create_test_rpc_client(network: BitcoinZNetworkType) -> BitcoinZRpcClient {
        let config = BitcoinZRpcConfig::new(
            "127.0.0.1".to_string(),
            network,
            Some("any".to_string()),
            Some("any".to_string()),
        );
        BitcoinZRpcClient::new(config)
    }

    #[test]
    #[ignore] // Use --ignored to run this test when BitcoinZ node is available
    fn test_bitcoinz_mainnet_connection() {
        println!("🚀 Testing BitcoinZ Mainnet Connection");

        let mut client = create_test_rpc_client(BitcoinZNetworkType::Mainnet);

        // Test basic connectivity by directly calling getblockchaininfo
        match client.get_blockchain_info() {
            Ok(info) => {
                println!("✅ Connection successful!");
                println!("   Chain: {:?}", info.get("chain"));
                println!("   Blocks: {:?}", info.get("blocks"));
                test_basic_rpc_calls(&mut client);
            }
            Err(e) => {
                println!("❌ Connection failed: {:?}", e);
                println!("   Make sure BitcoinZ node is running on port 1979");
                println!("   Check RPC credentials and network settings");
            }
        }
    }

    #[test]
    #[ignore] // Use --ignored to run this test when BitcoinZ node is available
    fn test_bitcoinz_testnet_connection() {
        println!("🚀 Testing BitcoinZ Testnet Connection");
        
        let mut client = create_test_rpc_client(BitcoinZNetworkType::Testnet);
        
        match client.test_connection() {
            Ok(true) => {
                println!("✅ Testnet connection successful!");
                test_basic_rpc_calls(&mut client);
            }
            Ok(false) => {
                println!("❌ Testnet connection failed");
                println!("   Make sure BitcoinZ testnet node is running on port 11979");
            }
            Err(e) => {
                println!("❌ Testnet connection error: {:?}", e);
            }
        }
    }

    #[test]
    #[ignore] // Use --ignored to run this test when BitcoinZ node is available
    fn test_bitcoinz_regtest_connection() {
        println!("🚀 Testing BitcoinZ Regtest Connection");
        
        let mut client = create_test_rpc_client(BitcoinZNetworkType::Regtest);
        
        match client.test_connection() {
            Ok(true) => {
                println!("✅ Regtest connection successful!");
                test_basic_rpc_calls(&mut client);
            }
            Ok(false) => {
                println!("❌ Regtest connection failed");
                println!("   Make sure BitcoinZ regtest node is running");
            }
            Err(e) => {
                println!("❌ Regtest connection error: {:?}", e);
            }
        }
    }

    fn test_basic_rpc_calls(client: &mut BitcoinZRpcClient) {
        println!("\n🔍 Testing Basic RPC Calls");
        println!("---------------------------");
        
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
                if let Some(version) = info.get("version") {
                    println!("   Version: {}", version);
                }
            }
            Err(e) => println!("❌ getblockchaininfo failed: {:?}", e),
        }
        
        // Test getblockcount
        match client.get_block_count() {
            Ok(count) => {
                println!("✅ getblockcount: {}", count);
                
                // Test getting recent blocks if we have any
                if count > 0 {
                    test_block_operations(client, count);
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
                if let Some(connections) = info.get("connections") {
                    println!("   Connections: {}", connections);
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
                if let Some(difficulty) = info.get("difficulty") {
                    println!("   Difficulty: {}", difficulty);
                }
            }
            Err(e) => println!("❌ getmininginfo failed: {:?}", e),
        }
    }

    fn test_block_operations(client: &mut BitcoinZRpcClient, block_count: u64) {
        println!("\n🧱 Testing Block Operations");
        println!("---------------------------");
        
        // Test getting a recent block (not the very latest to avoid timing issues)
        let test_height = if block_count > 10 { block_count - 5 } else { 1 };
        
        // Test getting block hash by height
        match client.get_block_hash(test_height) {
            Ok(hash) => {
                println!("✅ Block hash at height {}: {}", test_height, hash);
                
                // Test getting block by hash with different verbosity levels
                test_block_retrieval_by_hash(client, &hash);
            }
            Err(e) => println!("❌ Block hash retrieval failed: {:?}", e),
        }
        
        // Test getting block by height directly
        match client.get_block_by_height(test_height, 1) {
            Ok(block) => {
                println!("✅ Block by height retrieval successful");
                if let Some(height) = block.get("height") {
                    println!("   Height: {}", height);
                }
                if let Some(confirmations) = block.get("confirmations") {
                    println!("   Confirmations: {}", confirmations);
                }
                if let Some(time) = block.get("time") {
                    println!("   Time: {}", time);
                }
            }
            Err(e) => println!("❌ Block by height retrieval failed: {:?}", e),
        }
    }

    fn test_block_retrieval_by_hash(client: &mut BitcoinZRpcClient, hash: &str) {
        println!("\n📦 Testing Block Retrieval by Hash");
        println!("----------------------------------");
        
        // Test with verbosity 0 (raw hex)
        match client.get_block(hash, 0) {
            Ok(block) => {
                if let Some(hex) = block.as_str() {
                    println!("✅ Raw block (verbosity 0): {} bytes", hex.len() / 2);
                }
            }
            Err(e) => println!("❌ Raw block retrieval failed: {:?}", e),
        }
        
        // Test with verbosity 1 (basic info)
        match client.get_block(hash, 1) {
            Ok(block) => {
                println!("✅ Block info (verbosity 1): Success");
                if let Some(tx_array) = block.get("tx").and_then(|tx| tx.as_array()) {
                    println!("   Transactions: {}", tx_array.len());
                }
                if let Some(merkleroot) = block.get("merkleroot") {
                    println!("   Merkle Root: {}", merkleroot);
                }
            }
            Err(e) => println!("❌ Block info retrieval failed: {:?}", e),
        }
        
        // Test with verbosity 2 (full transaction details)
        match client.get_block(hash, 2) {
            Ok(block) => {
                println!("✅ Full block (verbosity 2): Success");
                if let Some(tx_array) = block.get("tx").and_then(|tx| tx.as_array()) {
                    println!("   Transactions with details: {}", tx_array.len());
                    
                    // Test transaction parsing
                    if let Some(first_tx) = tx_array.get(0) {
                        test_transaction_parsing(first_tx);
                    }
                }
            }
            Err(e) => println!("❌ Full block retrieval failed: {:?}", e),
        }
    }

    fn test_transaction_parsing(tx: &serde_json::Value) {
        println!("\n💰 Testing Transaction Parsing");
        println!("------------------------------");
        
        if let Some(txid) = tx.get("txid") {
            println!("✅ Transaction ID: {}", txid);
        }
        
        if let Some(version) = tx.get("version") {
            println!("✅ Version: {}", version);
        }
        
        if let Some(vin) = tx.get("vin").and_then(|v| v.as_array()) {
            println!("✅ Inputs: {}", vin.len());
        }
        
        if let Some(vout) = tx.get("vout").and_then(|v| v.as_array()) {
            println!("✅ Outputs: {}", vout.len());
            
            // Check for OP_RETURN outputs (potential Stacks operations)
            for (i, output) in vout.iter().enumerate() {
                if let Some(script_pub_key) = output.get("scriptPubKey") {
                    if let Some(script_type) = script_pub_key.get("type") {
                        if script_type == "nulldata" {
                            println!("   Found OP_RETURN output at index {}", i);
                            if let Some(hex) = script_pub_key.get("hex") {
                                println!("   Data: {}", hex);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    #[ignore] // Use --ignored to run this test when BitcoinZ node is available
    fn test_bitcoinz_indexer_integration() {
        println!("🚀 Testing BitcoinZ Indexer Integration");
        
        let config = BitcoinZIndexerConfig::default_mainnet(1);
        
        match BitcoinZIndexer::new(config) {
            Ok(mut indexer) => {
                println!("✅ BitcoinZ indexer created successfully");
                
                // Test connection
                match indexer.test_connection() {
                    Ok(true) => {
                        println!("✅ Indexer connection successful");
                        
                        // Test getting current block height
                        match indexer.get_block_height() {
                            Ok(height) => {
                                println!("✅ Current block height: {}", height);
                                
                                // Test getting a specific block
                                if height > 0 {
                                    let test_height = if height > 10 { height - 5 } else { 1 };
                                    match indexer.get_block_by_height(test_height) {
                                        Ok(block) => {
                                            println!("✅ Retrieved block at height {}", test_height);
                                            println!("   Block hash: {:?}", block.block_hash);
                                            println!("   Transactions: {}", block.txs.len());
                                            println!("   Timestamp: {}", block.timestamp);
                                        }
                                        Err(e) => println!("❌ Block retrieval failed: {:?}", e),
                                    }
                                }
                            }
                            Err(e) => println!("❌ Block height retrieval failed: {:?}", e),
                        }
                    }
                    Ok(false) => println!("❌ Indexer connection failed"),
                    Err(e) => println!("❌ Indexer connection error: {:?}", e),
                }
            }
            Err(e) => println!("❌ Failed to create BitcoinZ indexer: {:?}", e),
        }
    }

    #[test]
    fn test_bitcoinz_network_constants() {
        println!("🚀 Testing BitcoinZ Network Constants");

        // Test mainnet
        assert_eq!(get_bitcoinz_rpc_port(BitcoinZNetworkType::Mainnet), 1979);
        println!("✅ Mainnet RPC port: 1979");

        // Test testnet
        assert_eq!(get_bitcoinz_rpc_port(BitcoinZNetworkType::Testnet), 11979);
        println!("✅ Testnet RPC port: 11979");

        // Test regtest
        assert_eq!(get_bitcoinz_rpc_port(BitcoinZNetworkType::Regtest), 11979);
        println!("✅ Regtest RPC port: 11979");
    }

    #[test]
    fn test_bitcoinz_burn_operations() {
        use super::super::burn::*;
        use super::super::address::{BitcoinZAddress, BitcoinZAddressType};
        use crate::chainstate::stacks::address::PoxAddress;
        use stacks_common::types::chainstate::StacksAddress;
        use stacks_common::util::hash::Hash160;
        use crate::burnchains::Txid;

        println!("🚀 Testing BitcoinZ Burn Operations");

        // Test burn address constants
        let mainnet_burn = get_bitcoinz_burn_address(BitcoinZNetworkType::Mainnet);
        assert_eq!(mainnet_burn, BITCOINZ_MAINNET_BURN_ADDRESS);
        println!("✅ Mainnet burn address: {}", mainnet_burn);

        let testnet_burn = get_bitcoinz_burn_address(BitcoinZNetworkType::Testnet);
        assert_eq!(testnet_burn, BITCOINZ_TESTNET_BURN_ADDRESS);
        println!("✅ Testnet burn address: {}", testnet_burn);

        // Test burn amount validation
        let sender = BitcoinZAddress::new(
            BitcoinZAddressType::PublicKeyHash,
            BitcoinZNetworkType::Mainnet,
            vec![1u8; 20],
        );

        let reward_address = PoxAddress::Standard(
            StacksAddress::new(0, Hash160([0u8; 20])).unwrap(),
            Some(stacks_common::address::AddressHashMode::SerializeP2PKH),
        );

        // Test valid burn operation
        let burn_op = BitcoinZBurnOp::new(
            sender.clone(),
            MIN_BITCOINZ_BURN_AMOUNT,
            reward_address.clone(),
            Txid([0u8; 32]),
            0,
            100,
            [0u8; 32],
        );
        assert!(burn_op.is_ok());
        println!("✅ Valid burn operation created");

        // Test burn operation validation
        let burn_op = burn_op.unwrap();
        assert!(burn_op.check().is_ok());
        println!("✅ Burn operation validation passed");

        // Test address conversion
        let pox_addr = bitcoinz_address_to_pox_address(&sender).unwrap();
        let converted_back = pox_address_to_bitcoinz_address(&pox_addr, BitcoinZNetworkType::Mainnet).unwrap();
        assert_eq!(sender.address_type, converted_back.address_type);
        assert_eq!(sender.network, converted_back.network);
        println!("✅ Address conversion working correctly");
    }

    #[test]
    fn test_bitcoinz_pox_operations() {
        use crate::chainstate::burn::operations::bitcoinz_burn::*;
        use super::super::address::{BitcoinZAddress, BitcoinZAddressType};
        use crate::chainstate::stacks::address::PoxAddress;
        use stacks_common::types::chainstate::{StacksAddress, BurnchainHeaderHash};
        use stacks_common::util::hash::Hash160;
        use crate::burnchains::Txid;

        println!("🚀 Testing BitcoinZ PoX Operations");

        // Test BitcoinZ Leader Block Commit
        let sender = BitcoinZAddress::new(
            BitcoinZAddressType::PublicKeyHash,
            BitcoinZNetworkType::Mainnet,
            vec![1u8; 20],
        );

        let commit_op = BitcoinZLeaderBlockCommitOp::new(
            sender.clone(),
            super::super::burn::MIN_BITCOINZ_BURN_AMOUNT,
            vec![],
            Txid([0u8; 32]),
            0,
            100,
            BurnchainHeaderHash([0u8; 32]),
            [0u8; 32],
            [0u8; 32],
            0,
            0,
            0,
            0,
        );

        assert!(commit_op.is_ok());
        let commit_op = commit_op.unwrap();
        assert!(commit_op.check().is_ok());
        println!("✅ BitcoinZ Leader Block Commit operation working");

        // Test BitcoinZ Stack STX
        let stacks_sender = StacksAddress::new(0, Hash160([1u8; 20])).unwrap();
        let reward_addr = BitcoinZAddress::new(
            BitcoinZAddressType::PublicKeyHash,
            BitcoinZNetworkType::Mainnet,
            vec![2u8; 20],
        );

        let stack_op = BitcoinZStackStxOp::new(
            stacks_sender,
            reward_addr,
            1_000_000, // 1 STX
            1,         // 1 cycle
            Txid([0u8; 32]),
            0,
            100,
            BurnchainHeaderHash([0u8; 32]),
        );

        assert!(stack_op.is_ok());
        let stack_op = stack_op.unwrap();
        assert!(stack_op.check().is_ok());
        assert!(stack_op.get_pox_reward_address().is_ok());
        println!("✅ BitcoinZ Stack STX operation working");

        // Test BitcoinZ Burn Operation enum
        let burn_op = BitcoinZBurnOperation::LeaderBlockCommit(commit_op);
        assert!(burn_op.check().is_ok());
        assert_eq!(burn_op.burn_amount(), super::super::burn::MIN_BITCOINZ_BURN_AMOUNT);
        println!("✅ BitcoinZ Burn Operation enum working");
    }
}
