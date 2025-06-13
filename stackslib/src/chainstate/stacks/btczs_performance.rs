// BTCZS Performance Optimization
// This module implements performance optimizations for BTCZS operations

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use stacks_common::types::chainstate::StacksAddress;

use crate::chainstate::stacks::btczs_token::BTCZSBalance;
use crate::chainstate::stacks::btczs_stacking::BTCZSStackingState;
use crate::chainstate::stacks::Error as ChainstateError;

/// Performance metrics for BTCZS operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTCZSPerformanceMetrics {
    /// Transaction processing metrics
    pub transaction_metrics: TransactionMetrics,
    /// Stacking operation metrics
    pub stacking_metrics: StackingMetrics,
    /// Fee calculation metrics
    pub fee_metrics: FeeMetrics,
    /// Network metrics
    pub network_metrics: NetworkMetrics,
    /// Cache performance metrics
    pub cache_metrics: CacheMetrics,
}

/// Transaction processing performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMetrics {
    /// Average transaction processing time in milliseconds
    pub avg_processing_time_ms: f64,
    /// Transactions per second
    pub transactions_per_second: f64,
    /// Peak transactions per second
    pub peak_tps: f64,
    /// Total transactions processed
    pub total_transactions: u64,
    /// Failed transactions
    pub failed_transactions: u64,
}

/// Stacking operation performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackingMetrics {
    /// Average stacking operation time in milliseconds
    pub avg_stacking_time_ms: f64,
    /// Reward calculation time in milliseconds
    pub avg_reward_calc_time_ms: f64,
    /// Active stackers count
    pub active_stackers: u64,
    /// Total stacking operations
    pub total_stacking_ops: u64,
}

/// Fee calculation performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeMetrics {
    /// Average fee calculation time in microseconds
    pub avg_fee_calc_time_us: f64,
    /// Dynamic fee adjustments per hour
    pub fee_adjustments_per_hour: f64,
    /// Current network congestion factor
    pub current_congestion_factor: f64,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Average block time in seconds
    pub avg_block_time_s: f64,
    /// Network hash rate (estimated)
    pub estimated_hash_rate: f64,
    /// Peer connection count
    pub peer_connections: u32,
    /// Network bandwidth usage in MB/s
    pub bandwidth_usage_mbps: f64,
}

/// Cache performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Cache hit rate percentage
    pub hit_rate_percent: f64,
    /// Cache size in MB
    pub cache_size_mb: f64,
    /// Cache evictions per minute
    pub evictions_per_minute: f64,
}

/// BTCZS performance optimizer
pub struct BTCZSPerformanceOptimizer {
    /// Balance cache for fast lookups
    balance_cache: HashMap<StacksAddress, (BTCZSBalance, Instant)>,
    /// Stacking state cache
    stacking_cache: HashMap<StacksAddress, (BTCZSStackingState, Instant)>,
    /// Recent transaction times for TPS calculation
    recent_tx_times: VecDeque<Instant>,
    /// Performance metrics
    metrics: BTCZSPerformanceMetrics,
    /// Cache configuration
    cache_config: CacheConfig,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum cache size (number of entries)
    pub max_cache_size: usize,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            max_cache_size: 10000,
            cache_ttl_seconds: 300, // 5 minutes
            enable_monitoring: true,
            metrics_interval_seconds: 60, // 1 minute
        }
    }
}

impl BTCZSPerformanceOptimizer {
    /// Create a new performance optimizer
    pub fn new(config: CacheConfig) -> Self {
        BTCZSPerformanceOptimizer {
            balance_cache: HashMap::new(),
            stacking_cache: HashMap::new(),
            recent_tx_times: VecDeque::new(),
            metrics: BTCZSPerformanceMetrics::default(),
            cache_config: config,
        }
    }

    /// Get cached balance or fetch if not available
    pub fn get_balance_cached(
        &mut self,
        address: &StacksAddress,
        block_height: u64,
    ) -> Result<BTCZSBalance, ChainstateError> {
        let now = Instant::now();
        
        // Check cache first
        if let Some((balance, cached_time)) = self.balance_cache.get(address) {
            if now.duration_since(*cached_time).as_secs() < self.cache_config.cache_ttl_seconds {
                self.metrics.cache_metrics.hit_rate_percent += 1.0;
                return Ok(balance.clone());
            }
        }
        
        // Cache miss - fetch from database
        // TODO: Implement actual database fetch
        let balance = BTCZSBalance::zero(block_height);
        
        // Update cache
        self.update_balance_cache(address.clone(), balance.clone(), now);
        
        Ok(balance)
    }

    /// Get cached stacking state or fetch if not available
    pub fn get_stacking_state_cached(
        &mut self,
        address: &StacksAddress,
        block_height: u64,
    ) -> Result<Option<BTCZSStackingState>, ChainstateError> {
        let now = Instant::now();
        
        // Check cache first
        if let Some((state, cached_time)) = self.stacking_cache.get(address) {
            if now.duration_since(*cached_time).as_secs() < self.cache_config.cache_ttl_seconds {
                self.metrics.cache_metrics.hit_rate_percent += 1.0;
                return Ok(Some(state.clone()));
            }
        }
        
        // Cache miss - fetch from database
        // TODO: Implement actual database fetch
        let state: Option<BTCZSStackingState> = None;
        
        // Update cache if state exists
        if let Some(ref stacking_state) = state {
            self.update_stacking_cache(address.clone(), stacking_state.clone(), now);
        }
        
        Ok(state)
    }

    /// Record transaction processing time
    pub fn record_transaction_time(&mut self, processing_time: Duration) {
        let now = Instant::now();
        self.recent_tx_times.push_back(now);
        
        // Keep only recent transactions (last minute)
        while let Some(&front_time) = self.recent_tx_times.front() {
            if now.duration_since(front_time).as_secs() > 60 {
                self.recent_tx_times.pop_front();
            } else {
                break;
            }
        }
        
        // Update metrics
        self.metrics.transaction_metrics.total_transactions += 1;
        self.metrics.transaction_metrics.transactions_per_second = self.recent_tx_times.len() as f64 / 60.0;
        
        if self.metrics.transaction_metrics.transactions_per_second > self.metrics.transaction_metrics.peak_tps {
            self.metrics.transaction_metrics.peak_tps = self.metrics.transaction_metrics.transactions_per_second;
        }
        
        // Update average processing time
        let processing_ms = processing_time.as_millis() as f64;
        self.metrics.transaction_metrics.avg_processing_time_ms = 
            (self.metrics.transaction_metrics.avg_processing_time_ms + processing_ms) / 2.0;
    }

    /// Record stacking operation time
    pub fn record_stacking_time(&mut self, operation_time: Duration) {
        let operation_ms = operation_time.as_millis() as f64;
        self.metrics.stacking_metrics.avg_stacking_time_ms = 
            (self.metrics.stacking_metrics.avg_stacking_time_ms + operation_ms) / 2.0;
        self.metrics.stacking_metrics.total_stacking_ops += 1;
    }

    /// Record fee calculation time
    pub fn record_fee_calculation_time(&mut self, calc_time: Duration) {
        let calc_us = calc_time.as_micros() as f64;
        self.metrics.fee_metrics.avg_fee_calc_time_us = 
            (self.metrics.fee_metrics.avg_fee_calc_time_us + calc_us) / 2.0;
    }

    /// Update network metrics
    pub fn update_network_metrics(
        &mut self,
        block_time: Duration,
        peer_count: u32,
        bandwidth_mbps: f64,
    ) {
        let block_time_s = block_time.as_secs_f64();
        self.metrics.network_metrics.avg_block_time_s = 
            (self.metrics.network_metrics.avg_block_time_s + block_time_s) / 2.0;
        self.metrics.network_metrics.peer_connections = peer_count;
        self.metrics.network_metrics.bandwidth_usage_mbps = bandwidth_mbps;
    }

    /// Clean expired cache entries
    pub fn cleanup_cache(&mut self) {
        let now = Instant::now();
        let ttl = Duration::from_secs(self.cache_config.cache_ttl_seconds);
        
        // Clean balance cache
        self.balance_cache.retain(|_, (_, cached_time)| {
            now.duration_since(*cached_time) < ttl
        });
        
        // Clean stacking cache
        self.stacking_cache.retain(|_, (_, cached_time)| {
            now.duration_since(*cached_time) < ttl
        });
        
        // Update cache metrics
        self.update_cache_metrics();
    }

    /// Get current performance metrics
    pub fn get_metrics(&self) -> &BTCZSPerformanceMetrics {
        &self.metrics
    }

    /// Reset performance metrics
    pub fn reset_metrics(&mut self) {
        self.metrics = BTCZSPerformanceMetrics::default();
        self.recent_tx_times.clear();
    }

    /// Optimize cache based on usage patterns
    pub fn optimize_cache(&mut self) {
        // If cache is too large, remove least recently used entries
        if self.balance_cache.len() > self.cache_config.max_cache_size {
            let mut entries: Vec<_> = self.balance_cache.iter()
                .map(|(addr, (_, time))| (addr.clone(), *time))
                .collect();
            entries.sort_by_key(|(_, time)| *time);

            let remove_count = self.balance_cache.len() - self.cache_config.max_cache_size;
            for (addr, _) in entries.iter().take(remove_count) {
                self.balance_cache.remove(addr);
            }
        }

        // Same for stacking cache
        if self.stacking_cache.len() > self.cache_config.max_cache_size {
            let mut entries: Vec<_> = self.stacking_cache.iter()
                .map(|(addr, (_, time))| (addr.clone(), *time))
                .collect();
            entries.sort_by_key(|(_, time)| *time);

            let remove_count = self.stacking_cache.len() - self.cache_config.max_cache_size;
            for (addr, _) in entries.iter().take(remove_count) {
                self.stacking_cache.remove(addr);
            }
        }
    }

    /// Update balance cache
    fn update_balance_cache(&mut self, address: StacksAddress, balance: BTCZSBalance, time: Instant) {
        self.balance_cache.insert(address, (balance, time));
        
        // Enforce cache size limit
        if self.balance_cache.len() > self.cache_config.max_cache_size {
            self.optimize_cache();
        }
    }

    /// Update stacking cache
    fn update_stacking_cache(&mut self, address: StacksAddress, state: BTCZSStackingState, time: Instant) {
        self.stacking_cache.insert(address, (state, time));
        
        // Enforce cache size limit
        if self.stacking_cache.len() > self.cache_config.max_cache_size {
            self.optimize_cache();
        }
    }

    /// Update cache performance metrics
    fn update_cache_metrics(&mut self) {
        let total_entries = self.balance_cache.len() + self.stacking_cache.len();
        self.metrics.cache_metrics.cache_size_mb = (total_entries * 1024) as f64 / (1024.0 * 1024.0); // Rough estimate
        
        // Calculate hit rate (simplified)
        if self.metrics.cache_metrics.hit_rate_percent > 0.0 {
            self.metrics.cache_metrics.hit_rate_percent = 
                (self.metrics.cache_metrics.hit_rate_percent / 
                 (self.metrics.cache_metrics.hit_rate_percent + 1.0)) * 100.0;
        }
    }
}

impl Default for BTCZSPerformanceMetrics {
    fn default() -> Self {
        BTCZSPerformanceMetrics {
            transaction_metrics: TransactionMetrics::default(),
            stacking_metrics: StackingMetrics::default(),
            fee_metrics: FeeMetrics::default(),
            network_metrics: NetworkMetrics::default(),
            cache_metrics: CacheMetrics::default(),
        }
    }
}

impl Default for TransactionMetrics {
    fn default() -> Self {
        TransactionMetrics {
            avg_processing_time_ms: 0.0,
            transactions_per_second: 0.0,
            peak_tps: 0.0,
            total_transactions: 0,
            failed_transactions: 0,
        }
    }
}

impl Default for StackingMetrics {
    fn default() -> Self {
        StackingMetrics {
            avg_stacking_time_ms: 0.0,
            avg_reward_calc_time_ms: 0.0,
            active_stackers: 0,
            total_stacking_ops: 0,
        }
    }
}

impl Default for FeeMetrics {
    fn default() -> Self {
        FeeMetrics {
            avg_fee_calc_time_us: 0.0,
            fee_adjustments_per_hour: 0.0,
            current_congestion_factor: 0.0,
        }
    }
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        NetworkMetrics {
            avg_block_time_s: 0.0,
            estimated_hash_rate: 0.0,
            peer_connections: 0,
            bandwidth_usage_mbps: 0.0,
        }
    }
}

impl Default for CacheMetrics {
    fn default() -> Self {
        CacheMetrics {
            hit_rate_percent: 0.0,
            cache_size_mb: 0.0,
            evictions_per_minute: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stacks_common::util::hash::Hash160;

    #[test]
    fn test_performance_optimizer_creation() {
        let config = CacheConfig::default();
        let optimizer = BTCZSPerformanceOptimizer::new(config);
        
        assert_eq!(optimizer.balance_cache.len(), 0);
        assert_eq!(optimizer.stacking_cache.len(), 0);
        assert_eq!(optimizer.recent_tx_times.len(), 0);
    }

    #[test]
    fn test_cache_operations() {
        let mut optimizer = BTCZSPerformanceOptimizer::new(CacheConfig::default());
        let address = StacksAddress::new(0, Hash160([1u8; 20])).unwrap();
        
        // Test cache miss and population
        let balance = optimizer.get_balance_cached(&address, 100).unwrap();
        assert_eq!(balance.total, 0);
        assert_eq!(optimizer.balance_cache.len(), 1);
        
        // Test cache hit
        let cached_balance = optimizer.get_balance_cached(&address, 100).unwrap();
        assert_eq!(cached_balance.total, balance.total);
    }

    #[test]
    fn test_transaction_metrics() {
        let mut optimizer = BTCZSPerformanceOptimizer::new(CacheConfig::default());
        
        // Record some transaction times
        optimizer.record_transaction_time(Duration::from_millis(50));
        optimizer.record_transaction_time(Duration::from_millis(75));
        optimizer.record_transaction_time(Duration::from_millis(100));
        
        let metrics = optimizer.get_metrics();
        assert!(metrics.transaction_metrics.avg_processing_time_ms > 0.0);
        assert_eq!(metrics.transaction_metrics.total_transactions, 3);
    }

    #[test]
    fn test_cache_cleanup() {
        let mut config = CacheConfig::default();
        config.cache_ttl_seconds = 1; // Very short TTL for testing
        
        let mut optimizer = BTCZSPerformanceOptimizer::new(config);
        let address = StacksAddress::new(0, Hash160([1u8; 20])).unwrap();
        
        // Add entry to cache
        let _ = optimizer.get_balance_cached(&address, 100).unwrap();
        assert_eq!(optimizer.balance_cache.len(), 1);
        
        // Wait for TTL to expire
        std::thread::sleep(Duration::from_secs(2));
        
        // Cleanup should remove expired entries
        optimizer.cleanup_cache();
        assert_eq!(optimizer.balance_cache.len(), 0);
    }

    #[test]
    fn test_cache_size_limit() {
        let mut config = CacheConfig::default();
        config.max_cache_size = 2; // Very small cache for testing
        
        let mut optimizer = BTCZSPerformanceOptimizer::new(config);
        
        // Add entries beyond cache limit
        for i in 0..5 {
            let address = StacksAddress::new(0, Hash160([i as u8; 20])).unwrap();
            let _ = optimizer.get_balance_cached(&address, 100).unwrap();
        }
        
        // Cache should not exceed max size
        assert!(optimizer.balance_cache.len() <= 2);
    }
}
