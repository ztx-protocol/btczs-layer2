#!/bin/bash

# Real BitcoinZ Integration Test Script
# This script runs BTCZS integration tests with real BitcoinZ transactions

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BITCOINZ_RPC_URL="${BITCOINZ_RPC_URL:-http://localhost:1979}"
BITCOINZ_RPC_USER="${BITCOINZ_RPC_USER:-btczs}"
BITCOINZ_RPC_PASS="${BITCOINZ_RPC_PASS:-btczs}"
TEST_AMOUNT="${TEST_AMOUNT:-1}" # 1 BTCZ for safety

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Help function
show_help() {
    cat << EOF
Real BitcoinZ Integration Test Script

Usage: $0 [OPTIONS]

Options:
    --rpc-url URL        BitcoinZ RPC URL (default: http://localhost:1979)
    --rpc-user USER      BitcoinZ RPC username (default: btczs)
    --rpc-pass PASS      BitcoinZ RPC password (default: btczs)
    --test-amount BTCZ   Test amount in BTCZ (default: 1)
    --dry-run            Run without actual transactions
    -h, --help           Show this help message

Environment Variables:
    BITCOINZ_RPC_URL     BitcoinZ RPC endpoint
    BITCOINZ_RPC_USER    BitcoinZ RPC username
    BITCOINZ_RPC_PASS    BitcoinZ RPC password
    TEST_AMOUNT          Amount of BTCZ to use for testing

Examples:
    $0                                    # Run with default settings
    $0 --test-amount 0.5                  # Test with 0.5 BTCZ
    $0 --rpc-url http://192.168.1.100:1979  # Use remote BitcoinZ node
    $0 --dry-run                          # Test without real transactions

WARNING: This script will use real BTCZ tokens for testing!
Make sure you understand the risks before proceeding.

EOF
}

# Parse command line arguments
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --rpc-url)
            BITCOINZ_RPC_URL="$2"
            shift 2
            ;;
        --rpc-user)
            BITCOINZ_RPC_USER="$2"
            shift 2
            ;;
        --rpc-pass)
            BITCOINZ_RPC_PASS="$2"
            shift 2
            ;;
        --test-amount)
            TEST_AMOUNT="$2"
            shift 2
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Safety confirmation
confirm_test() {
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "Running in DRY RUN mode - no real transactions will be made"
        return 0
    fi

    echo
    log_warning "⚠️  REAL BITCOINZ INTEGRATION TEST WARNING ⚠️"
    echo
    echo "This test will:"
    echo "  - Use REAL BTCZ tokens ($TEST_AMOUNT BTCZ)"
    echo "  - Send transactions to the BitcoinZ blockchain"
    echo "  - Create test addresses in your wallet"
    echo "  - Perform burn operations (tokens will be destroyed)"
    echo
    echo "Configuration:"
    echo "  BitcoinZ RPC: $BITCOINZ_RPC_URL"
    echo "  Test Amount: $TEST_AMOUNT BTCZ"
    echo "  RPC User: $BITCOINZ_RPC_USER"
    echo
    echo "Make sure:"
    echo "  ✓ Your BitcoinZ node is running and synced"
    echo "  ✓ You have sufficient BTCZ balance (>$TEST_AMOUNT BTCZ)"
    echo "  ✓ You understand this is experimental software"
    echo "  ✓ You can afford to lose the test amount"
    echo
    read -p "Do you want to proceed with REAL BitcoinZ testing? (yes/no): " -r
    echo
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        log_info "Test cancelled by user"
        exit 0
    fi
}

# Check BitcoinZ node connectivity
check_bitcoinz_node() {
    log_info "Checking BitcoinZ node connectivity..."
    
    local response
    if response=$(curl -s -u "$BITCOINZ_RPC_USER:$BITCOINZ_RPC_PASS" \
        -d '{"jsonrpc":"1.0","id":"test","method":"getblockchaininfo","params":[]}' \
        -H 'content-type: text/plain;' \
        "$BITCOINZ_RPC_URL" 2>/dev/null); then
        
        if echo "$response" | grep -q '"result"'; then
            log_success "BitcoinZ node is accessible and responding"
            
            # Extract and display blockchain info
            local blocks=$(echo "$response" | grep -o '"blocks":[0-9]*' | cut -d':' -f2)
            local chain=$(echo "$response" | grep -o '"chain":"[^"]*"' | cut -d'"' -f4)
            
            log_info "Blockchain Info:"
            log_info "  Chain: $chain"
            log_info "  Blocks: $blocks"
            
            return 0
        else
            log_error "BitcoinZ node returned error: $response"
            return 1
        fi
    else
        log_error "Cannot connect to BitcoinZ node at $BITCOINZ_RPC_URL"
        log_error "Make sure:"
        log_error "  - BitcoinZ node is running"
        log_error "  - RPC is enabled"
        log_error "  - Credentials are correct"
        return 1
    fi
}

# Check wallet balance
check_wallet_balance() {
    log_info "Checking wallet balance..."
    
    local response
    if response=$(curl -s -u "$BITCOINZ_RPC_USER:$BITCOINZ_RPC_PASS" \
        -d '{"jsonrpc":"1.0","id":"balance","method":"getbalance","params":[]}' \
        -H 'content-type: text/plain;' \
        "$BITCOINZ_RPC_URL" 2>/dev/null); then
        
        if echo "$response" | grep -q '"result"'; then
            local balance=$(echo "$response" | grep -o '"result":[0-9.]*' | cut -d':' -f2)
            log_info "Wallet Balance: $balance BTCZ"
            
            # Check if balance is sufficient
            if (( $(echo "$balance >= $TEST_AMOUNT" | bc -l) )); then
                log_success "Sufficient balance for testing"
                return 0
            else
                log_error "Insufficient balance for testing"
                log_error "Required: $TEST_AMOUNT BTCZ, Available: $balance BTCZ"
                return 1
            fi
        else
            log_error "Failed to get wallet balance: $response"
            return 1
        fi
    else
        log_error "Cannot get wallet balance"
        return 1
    fi
}

# Run BTCZS integration test
run_btczs_test() {
    log_info "Running BTCZS integration test..."
    
    cd "$PROJECT_ROOT/stackslib"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "DRY RUN: Would run real BitcoinZ integration test"
        log_info "Test parameters:"
        log_info "  RPC URL: $BITCOINZ_RPC_URL"
        log_info "  Test Amount: $TEST_AMOUNT BTCZ"
        log_info "  RPC User: $BITCOINZ_RPC_USER"
        return 0
    fi
    
    # Set environment variables for the test
    export BITCOINZ_RPC_URL
    export BITCOINZ_RPC_USER
    export BITCOINZ_RPC_PASS
    export TEST_AMOUNT
    
    # Run the Rust integration test
    if cargo test real_bitcoinz_integration -- --nocapture; then
        log_success "BTCZS integration test completed successfully"
        return 0
    else
        log_error "BTCZS integration test failed"
        return 1
    fi
}

# Cleanup function
cleanup() {
    log_info "Cleaning up test environment..."
    
    if [[ "$DRY_RUN" == "false" ]]; then
        log_info "Test addresses and transactions remain in BitcoinZ blockchain"
        log_info "This is normal for blockchain testing"
    fi
    
    log_info "Cleanup completed"
}

# Main execution
main() {
    echo "🧪 BTCZS Real BitcoinZ Integration Test"
    echo "======================================"
    
    # Safety confirmation
    confirm_test
    
    # Pre-flight checks
    log_info "Running pre-flight checks..."
    
    if ! check_bitcoinz_node; then
        log_error "BitcoinZ node check failed"
        exit 1
    fi
    
    if [[ "$DRY_RUN" == "false" ]] && ! check_wallet_balance; then
        log_error "Wallet balance check failed"
        exit 1
    fi
    
    log_success "Pre-flight checks passed"
    
    # Run the integration test
    if run_btczs_test; then
        log_success "🎉 Real BitcoinZ integration test completed successfully!"
        echo
        log_info "Next steps:"
        log_info "  1. Review test results and logs"
        log_info "  2. Verify BTCZS functionality"
        log_info "  3. Consider running additional tests"
        log_info "  4. Prepare for testnet deployment"
    else
        log_error "❌ Real BitcoinZ integration test failed"
        echo
        log_info "Troubleshooting:"
        log_info "  1. Check BitcoinZ node logs"
        log_info "  2. Verify network connectivity"
        log_info "  3. Check wallet balance and permissions"
        log_info "  4. Review BTCZS error logs"
        exit 1
    fi
    
    # Cleanup
    cleanup
}

# Handle script interruption
trap cleanup EXIT

# Run main function
main "$@"
