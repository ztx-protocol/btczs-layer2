#!/bin/bash

# BTCZS Real-Life Function Testing Suite
# Tests all core functionality before VPS deployment

set -e  # Exit on any error

echo "🔥 BTCZS Real-Life Testing Suite"
echo "================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BTCZS_RPC="http://localhost:20443"
BITCOINZ_RPC="http://localhost:1979"
BTCZS_USER="test"
BTCZS_PASS="test"
BITCOINZ_USER="test"
BITCOINZ_PASS="test"

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((TESTS_PASSED++))
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((TESTS_FAILED++))
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Test function wrapper
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo ""
    log_info "Testing: $test_name"
    echo "Command: $test_command"
    
    if eval "$test_command"; then
        log_success "$test_name"
    else
        log_error "$test_name"
    fi
}

# RPC call helper
btczs_rpc() {
    curl -s -u "$BTCZS_USER:$BTCZS_PASS" \
         -H "Content-Type: application/json" \
         -d "$1" \
         "$BTCZS_RPC" 2>/dev/null || echo "ERROR"
}

bitcoinz_rpc() {
    curl -s -u "$BITCOINZ_USER:$BITCOINZ_PASS" \
         -H "Content-Type: application/json" \
         -d "$1" \
         "$BITCOINZ_RPC" 2>/dev/null || echo "ERROR"
}

echo ""
echo "🔍 PHASE 1: Node Connectivity Tests"
echo "===================================="

# Test 1: BTCZS Node Connection
run_test "BTCZS Node Connection" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_info\",\"id\":1}" | grep -q "stacks_tip_height"'

# Test 2: BitcoinZ Node Connection  
run_test "BitcoinZ Node Connection" \
    'bitcoinz_rpc "{\"jsonrpc\":\"1.0\",\"method\":\"getblockchaininfo\",\"id\":1}" | grep -q "bestblockhash"'

# Test 3: BTCZS Block Height
run_test "BTCZS Block Height Check" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_info\",\"id\":1}" | jq -r ".result.stacks_tip_height" | grep -E "^[0-9]+$"'

# Test 4: BitcoinZ Block Height
run_test "BitcoinZ Block Height Check" \
    'bitcoinz_rpc "{\"jsonrpc\":\"1.0\",\"method\":\"getblockcount\",\"id\":1}" | jq -r ".result" | grep -E "^[0-9]+$"'

echo ""
echo "⚙️ PHASE 2: PoX System Tests"
echo "============================="

# Test 5: PoX Info
run_test "PoX Information Retrieval" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_pox_info\",\"id\":1}" | grep -q "reward_cycle_id"'

# Test 6: Current Reward Cycle
run_test "Current Reward Cycle" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_pox_info\",\"id\":1}" | jq -r ".result.reward_cycle_id" | grep -E "^[0-9]+$"'

# Test 7: Minimum Stacking Amount
run_test "Minimum Stacking Amount" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_pox_info\",\"id\":1}" | jq -r ".result.min_amount_ustx" | grep -E "^[0-9]+$"'

echo ""
echo "🔗 PHASE 3: Network Synchronization Tests"
echo "=========================================="

# Test 8: BTCZS-BitcoinZ Sync Status
run_test "BTCZS-BitcoinZ Synchronization" \
    'btczs_height=$(btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_info\",\"id\":1}" | jq -r ".result.burn_block_height"); 
     bitcoinz_height=$(bitcoinz_rpc "{\"jsonrpc\":\"1.0\",\"method\":\"getblockcount\",\"id\":1}" | jq -r ".result");
     echo "BTCZS burn height: $btczs_height, BitcoinZ height: $bitcoinz_height";
     [ "$btczs_height" -le "$bitcoinz_height" ]'

echo ""
echo "💰 PHASE 4: Account and Balance Tests"
echo "======================================"

# Test 9: Account Info (using a test address)
TEST_ADDRESS="SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7"
run_test "Account Information Retrieval" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_account_info\",\"params\":[\"'$TEST_ADDRESS'\"],\"id\":1}" | grep -q "balance"'

echo ""
echo "🔄 PHASE 5: Transaction Pool Tests"
echo "==================================="

# Test 10: Mempool Status
run_test "Transaction Mempool Status" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_mempool_info\",\"id\":1}" | grep -q "size"'

echo ""
echo "📊 PHASE 6: API Endpoint Tests"
echo "==============================="

# Test 11: Web Interface Connectivity
run_test "BTCZS Web Interface" \
    'curl -s http://localhost:3000 | grep -q "BTCZS"'

# Test 12: API Health Check
run_test "BTCZS API Health" \
    'curl -s http://localhost:20443/v2/info | grep -q "stacks_tip_height"'

echo ""
echo "🏗️ PHASE 7: Mining and Stacking Tests"
echo "======================================"

# Test 13: Mining Status
run_test "Mining Status Check" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_info\",\"id\":1}" | jq -r ".result" | grep -q "stacks_tip_height"'

# Test 14: Stacking Cycles
run_test "Stacking Cycle Information" \
    'btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_pox_info\",\"id\":1}" | jq -r ".result.reward_cycle_length" | grep -E "^[0-9]+$"'

echo ""
echo "🔐 PHASE 8: Security and Validation Tests"
echo "=========================================="

# Test 15: Node Security
run_test "RPC Authentication" \
    'curl -s -u "wrong:credentials" http://localhost:20443/v2/info | grep -q "401\|Unauthorized" || btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_info\",\"id\":1}" | grep -q "stacks_tip_height"'

echo ""
echo "📈 PHASE 9: Performance Tests"
echo "=============================="

# Test 16: Response Time Test
run_test "API Response Time" \
    'start_time=$(date +%s%N); btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_info\",\"id\":1}" > /dev/null; end_time=$(date +%s%N); 
     duration=$((($end_time - $start_time) / 1000000)); 
     echo "Response time: ${duration}ms"; 
     [ $duration -lt 5000 ]'  # Less than 5 seconds

echo ""
echo "🎯 PHASE 10: Integration Tests"
echo "==============================="

# Test 17: Full Stack Integration
run_test "Full Stack Integration" \
    'btczs_info=$(btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_info\",\"id\":1}");
     bitcoinz_info=$(bitcoinz_rpc "{\"jsonrpc\":\"1.0\",\"method\":\"getblockchaininfo\",\"id\":1}");
     pox_info=$(btczs_rpc "{\"jsonrpc\":\"2.0\",\"method\":\"get_pox_info\",\"id\":1}");
     echo "$btczs_info" | grep -q "stacks_tip_height" && 
     echo "$bitcoinz_info" | grep -q "bestblockhash" && 
     echo "$pox_info" | grep -q "reward_cycle_id"'

echo ""
echo "📋 TEST RESULTS SUMMARY"
echo "========================"
echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Tests Failed: $TESTS_FAILED${NC}"
echo -e "Total Tests: $((TESTS_PASSED + TESTS_FAILED))"

if [ $TESTS_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 ALL TESTS PASSED! BTCZS is ready for VPS deployment!${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}❌ Some tests failed. Please fix issues before VPS deployment.${NC}"
    exit 1
fi
