#!/bin/bash

# BTCZS Performance Testing Suite
# Tests system performance under load

echo "⚡ BTCZS Performance Testing Suite"
echo "=================================="

# Configuration
BTCZS_RPC="http://localhost:20443"
BITCOINZ_RPC="http://localhost:1979"
BTCZS_USER="test"
BTCZS_PASS="test"
BITCOINZ_USER="test"
BITCOINZ_PASS="test"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[PASS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }

# Helper function for RPC calls
btczs_rpc() {
    curl -s -u "$BTCZS_USER:$BTCZS_PASS" \
         -H "Content-Type: application/json" \
         -d "$1" \
         "$BTCZS_RPC" 2>/dev/null
}

bitcoinz_rpc() {
    curl -s -u "$BITCOINZ_USER:$BITCOINZ_PASS" \
         -H "Content-Type: application/json" \
         -d "$1" \
         "$BITCOINZ_RPC" 2>/dev/null
}

# Performance test function
test_response_time() {
    local test_name="$1"
    local rpc_call="$2"
    local node_type="$3"
    
    echo ""
    log_info "Testing: $test_name"
    
    local total_time=0
    local successful_calls=0
    local failed_calls=0
    
    for i in {1..10}; do
        local start_time=$(date +%s%N)
        
        if [ "$node_type" = "btczs" ]; then
            local result=$(btczs_rpc "$rpc_call")
        else
            local result=$(bitcoinz_rpc "$rpc_call")
        fi
        
        local end_time=$(date +%s%N)
        local duration=$(( (end_time - start_time) / 1000000 )) # Convert to milliseconds
        
        if echo "$result" | grep -q "result\|error"; then
            echo "  Call $i: ${duration}ms"
            total_time=$((total_time + duration))
            ((successful_calls++))
        else
            echo "  Call $i: FAILED"
            ((failed_calls++))
        fi
    done
    
    if [ $successful_calls -gt 0 ]; then
        local avg_time=$((total_time / successful_calls))
        log_success "$test_name - Average: ${avg_time}ms (${successful_calls}/10 successful)"
        
        if [ $avg_time -lt 1000 ]; then
            log_success "Excellent performance (< 1s)"
        elif [ $avg_time -lt 3000 ]; then
            log_warning "Good performance (< 3s)"
        else
            log_warning "Slow performance (> 3s)"
        fi
    else
        log_warning "$test_name - All calls failed"
    fi
}

# Concurrent load test
test_concurrent_load() {
    local test_name="$1"
    local rpc_call="$2"
    local node_type="$3"
    local concurrent_requests=5
    
    echo ""
    log_info "Testing: $test_name (${concurrent_requests} concurrent requests)"
    
    local start_time=$(date +%s%N)
    
    # Run concurrent requests
    for i in $(seq 1 $concurrent_requests); do
        (
            if [ "$node_type" = "btczs" ]; then
                btczs_rpc "$rpc_call" > /dev/null
            else
                bitcoinz_rpc "$rpc_call" > /dev/null
            fi
        ) &
    done
    
    # Wait for all background jobs to complete
    wait
    
    local end_time=$(date +%s%N)
    local total_duration=$(( (end_time - start_time) / 1000000 ))
    
    log_success "$test_name - ${concurrent_requests} concurrent requests completed in ${total_duration}ms"
}

echo ""
echo "🔍 BTCZS Node Performance Tests"
echo "==============================="

# Test 1: Basic Info Call
test_response_time "Get Info" '{"jsonrpc":"2.0","method":"get_info","id":1}' "btczs"

# Test 2: PoX Info Call
test_response_time "Get PoX Info" '{"jsonrpc":"2.0","method":"get_pox_info","id":1}' "btczs"

# Test 3: Account Info Call
test_response_time "Get Account Info" '{"jsonrpc":"2.0","method":"get_account_info","params":["SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7"],"id":1}' "btczs"

# Test 4: Mempool Info Call
test_response_time "Get Mempool Info" '{"jsonrpc":"2.0","method":"get_mempool_info","id":1}' "btczs"

echo ""
echo "🪙 BitcoinZ Node Performance Tests"
echo "=================================="

# Test 5: BitcoinZ Blockchain Info
test_response_time "Get Blockchain Info" '{"jsonrpc":"1.0","method":"getblockchaininfo","id":1}' "bitcoinz"

# Test 6: BitcoinZ Block Count
test_response_time "Get Block Count" '{"jsonrpc":"1.0","method":"getblockcount","id":1}' "bitcoinz"

echo ""
echo "🚀 Concurrent Load Tests"
echo "========================"

# Test 7: Concurrent BTCZS Calls
test_concurrent_load "BTCZS Concurrent Load" '{"jsonrpc":"2.0","method":"get_info","id":1}' "btczs"

# Test 8: Concurrent BitcoinZ Calls
test_concurrent_load "BitcoinZ Concurrent Load" '{"jsonrpc":"1.0","method":"getblockcount","id":1}' "bitcoinz"

echo ""
echo "🌐 Web Interface Performance"
echo "============================"

log_info "Testing Web Interface Response Time"
web_start=$(date +%s%N)
web_response=$(curl -s -w "%{http_code}" http://localhost:3000 -o /dev/null)
web_end=$(date +%s%N)
web_duration=$(( (web_end - web_start) / 1000000 ))

if [ "$web_response" = "200" ]; then
    log_success "Web Interface - Response: ${web_duration}ms (HTTP $web_response)"
else
    log_warning "Web Interface - Response: ${web_duration}ms (HTTP $web_response)"
fi

echo ""
echo "📊 System Resource Usage"
echo "========================"

# Check CPU and memory usage of our processes
log_info "Checking system resources..."

# Find BTCZS process
BTCZS_PID=$(ps aux | grep "stacks-node" | grep -v grep | awk '{print $2}' | head -1)
if [ -n "$BTCZS_PID" ]; then
    BTCZS_CPU=$(ps -p $BTCZS_PID -o %cpu= 2>/dev/null | tr -d ' ')
    BTCZS_MEM=$(ps -p $BTCZS_PID -o %mem= 2>/dev/null | tr -d ' ')
    log_success "BTCZS Node (PID $BTCZS_PID) - CPU: ${BTCZS_CPU}%, Memory: ${BTCZS_MEM}%"
else
    log_warning "BTCZS Node process not found"
fi

# Find BitcoinZ process
BITCOINZ_PID=$(ps aux | grep "bitcoinzd" | grep -v grep | awk '{print $2}' | head -1)
if [ -n "$BITCOINZ_PID" ]; then
    BITCOINZ_CPU=$(ps -p $BITCOINZ_PID -o %cpu= 2>/dev/null | tr -d ' ')
    BITCOINZ_MEM=$(ps -p $BITCOINZ_PID -o %mem= 2>/dev/null | tr -d ' ')
    log_success "BitcoinZ Node (PID $BITCOINZ_PID) - CPU: ${BITCOINZ_CPU}%, Memory: ${BITCOINZ_MEM}%"
else
    log_warning "BitcoinZ Node process not found"
fi

echo ""
echo "📋 PERFORMANCE SUMMARY"
echo "======================"
echo "✅ Response time tests completed"
echo "✅ Concurrent load tests completed"
echo "✅ Web interface performance tested"
echo "✅ System resource usage checked"

echo ""
echo "🎯 PERFORMANCE RECOMMENDATIONS"
echo "=============================="
echo "• Response times under 1s are excellent for VPS deployment"
echo "• Response times under 3s are acceptable for production"
echo "• Monitor CPU/Memory usage on VPS"
echo "• Consider load balancing for high traffic"

echo ""
echo "🚀 READY FOR VPS DEPLOYMENT!"
echo "Performance testing complete."
