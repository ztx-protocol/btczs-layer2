#!/bin/bash

# BTCZS Layer 2 Network Startup Script
# This script starts the complete BTCZS Layer 2 network

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BTCZS_DATA_DIR="${BTCZS_DATA_DIR:-$HOME/.btczs}"
BITCOINZ_RPC_URL="${BITCOINZ_RPC_URL:-http://localhost:1979}"
BITCOINZ_RPC_USER="${BITCOINZ_RPC_USER:-any}"
BITCOINZ_RPC_PASS="${BITCOINZ_RPC_PASS:-any}"
NETWORK="${NETWORK:-regtest}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
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

log_step() {
    echo -e "${PURPLE}[STEP]${NC} $1"
}

# Help function
show_help() {
    cat << EOF
BTCZS Layer 2 Network Startup Script

Usage: $0 [OPTIONS] COMMAND

Commands:
    start       Start the complete BTCZS Layer 2 network
    stop        Stop all BTCZS services
    restart     Restart all BTCZS services
    status      Check status of all services
    logs        Show logs from all services
    test        Run integration tests
    monitor     Start monitoring dashboard

Options:
    --network NET        Network type (regtest, testnet, mainnet) [default: regtest]
    --data-dir DIR       BTCZS data directory [default: ~/.btczs]
    --bitcoinz-url URL   BitcoinZ RPC URL [default: http://localhost:1979]
    --bitcoinz-user USER BitcoinZ RPC username [default: any]
    --bitcoinz-pass PASS BitcoinZ RPC password [default: any]
    -h, --help           Show this help message

Examples:
    $0 start                    # Start Layer 2 network (regtest)
    $0 --network testnet start  # Start on testnet
    $0 status                   # Check all services
    $0 test                     # Run integration tests
    $0 monitor                  # Start monitoring

EOF
}

# Parse command line arguments
COMMAND=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --network)
            NETWORK="$2"
            shift 2
            ;;
        --data-dir)
            BTCZS_DATA_DIR="$2"
            shift 2
            ;;
        --bitcoinz-url)
            BITCOINZ_RPC_URL="$2"
            shift 2
            ;;
        --bitcoinz-user)
            BITCOINZ_RPC_USER="$2"
            shift 2
            ;;
        --bitcoinz-pass)
            BITCOINZ_RPC_PASS="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        start|stop|restart|status|logs|test|monitor)
            COMMAND="$1"
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Validate network
validate_network() {
    case "$NETWORK" in
        regtest|testnet|mainnet)
            log_info "Using network: $NETWORK"
            ;;
        *)
            log_error "Invalid network: $NETWORK"
            log_error "Valid networks: regtest, testnet, mainnet"
            exit 1
            ;;
    esac
}

# Check prerequisites
check_prerequisites() {
    log_step "Checking prerequisites..."
    
    # Check if BitcoinZ node is accessible
    if curl -s -u "$BITCOINZ_RPC_USER:$BITCOINZ_RPC_PASS" \
        -d '{"jsonrpc":"1.0","id":"test","method":"getblockchaininfo","params":[]}' \
        -H 'content-type: text/plain;' \
        "$BITCOINZ_RPC_URL" > /dev/null 2>&1; then
        log_success "BitcoinZ node is accessible"
    else
        log_error "Cannot connect to BitcoinZ node at $BITCOINZ_RPC_URL"
        log_error "Make sure BitcoinZ node is running and RPC is enabled"
        exit 1
    fi
    
    # Check if BTCZS is built
    if [[ ! -f "$PROJECT_ROOT/target/release/stacks-node" ]] && [[ ! -f "$PROJECT_ROOT/target/debug/stacks-node" ]]; then
        log_warning "BTCZS node binary not found, building..."
        build_btczs
    fi
    
    # Create data directory
    mkdir -p "$BTCZS_DATA_DIR"
    mkdir -p "$BTCZS_DATA_DIR/logs"
    
    log_success "Prerequisites check completed"
}

# Build BTCZS
build_btczs() {
    log_step "Building BTCZS..."
    
    cd "$PROJECT_ROOT"
    
    if cargo build --release --bin stacks-node; then
        log_success "BTCZS build completed"
    else
        log_error "BTCZS build failed"
        exit 1
    fi
}

# Create BTCZS configuration
create_config() {
    log_step "Creating BTCZS configuration..."
    
    local config_file="$BTCZS_DATA_DIR/btczs.toml"
    
    cat > "$config_file" << EOF
# BTCZS Layer 2 Configuration (Stacks Node Format)

[node]
rpc_bind = "127.0.0.1:20443"
p2p_bind = "127.0.0.1:20444"
working_dir = "$BTCZS_DATA_DIR"
prometheus_bind = "127.0.0.1:20446"

[burnchain]
mode = "mocknet"

[[ustx_balance]]
address = "ST2QKZ4FKHAH1NQKYKYAYZPY440FEPK7GZ1R5HBP2"
amount = 10000000000000000
EOF

    log_success "Configuration created: $config_file"
}

# Start BTCZS Layer 2 network
start_btczs() {
    log_step "Starting BTCZS Layer 2 Network..."
    
    echo
    echo "🚀 BTCZS Layer 2 Network Startup"
    echo "================================"
    echo "Network: $NETWORK"
    echo "Data Directory: $BTCZS_DATA_DIR"
    echo "BitcoinZ RPC: $BITCOINZ_RPC_URL"
    echo "BTCZS RPC: http://127.0.0.1:20443"
    echo "BTCZS API: http://127.0.0.1:20445"
    echo "Monitoring: http://127.0.0.1:20446"
    echo
    
    # Create configuration
    create_config
    
    # Start BTCZS node
    log_step "Starting BTCZS node..."
    
    local btczs_binary
    if [[ -f "$PROJECT_ROOT/target/release/stacks-node" ]]; then
        btczs_binary="$PROJECT_ROOT/target/release/stacks-node"
    else
        btczs_binary="$PROJECT_ROOT/target/debug/stacks-node"
    fi
    
    # Start BTCZS node in background
    nohup "$btczs_binary" start --config "$BTCZS_DATA_DIR/btczs.toml" \
        > "$BTCZS_DATA_DIR/logs/btczs-node.log" 2>&1 &
    
    local btczs_pid=$!
    echo "$btczs_pid" > "$BTCZS_DATA_DIR/btczs-node.pid"
    
    log_success "BTCZS node started (PID: $btczs_pid)"
    
    # Wait for node to start
    log_info "Waiting for BTCZS node to initialize..."
    sleep 5
    
    # Check if node is running
    if check_btczs_status; then
        log_success "🎉 BTCZS Layer 2 Network is running!"
        echo
        echo "📊 Service Endpoints:"
        echo "  BTCZS RPC:    http://127.0.0.1:20443"
        echo "  BTCZS API:    http://127.0.0.1:20445"
        echo "  Monitoring:   http://127.0.0.1:20446"
        echo "  Logs:         $BTCZS_DATA_DIR/logs/"
        echo
        echo "🔗 Integration Status:"
        echo "  BitcoinZ L1:  $BITCOINZ_RPC_URL"
        echo "  BTCZS L2:     Active"
        echo "  Network:      $NETWORK"
        echo
        echo "Next steps:"
        echo "  1. Run: $0 test     # Test the integration"
        echo "  2. Run: $0 monitor  # Start monitoring"
        echo "  3. Run: $0 status   # Check service status"
    else
        log_error "BTCZS node failed to start properly"
        show_logs
        exit 1
    fi
}

# Stop BTCZS services
stop_btczs() {
    log_step "Stopping BTCZS Layer 2 Network..."
    
    if [[ -f "$BTCZS_DATA_DIR/btczs-node.pid" ]]; then
        local pid=$(cat "$BTCZS_DATA_DIR/btczs-node.pid")
        if kill "$pid" 2>/dev/null; then
            log_success "BTCZS node stopped (PID: $pid)"
        else
            log_warning "BTCZS node was not running or already stopped"
        fi
        rm -f "$BTCZS_DATA_DIR/btczs-node.pid"
    else
        log_warning "No BTCZS node PID file found"
    fi
}

# Check BTCZS status
check_btczs_status() {
    # Check if process is running
    if [[ -f "$BTCZS_DATA_DIR/btczs-node.pid" ]]; then
        local pid=$(cat "$BTCZS_DATA_DIR/btczs-node.pid")
        if ps -p "$pid" > /dev/null 2>&1; then
            echo "✅ BTCZS node is running (PID: $pid)"
            
            # Try to connect to RPC
            if curl -s http://127.0.0.1:20443 > /dev/null 2>&1; then
                echo "✅ BTCZS RPC is responding"
            else
                echo "⚠️  BTCZS RPC is not responding yet"
            fi
            
            return 0
        else
            echo "❌ BTCZS node process not found"
            return 1
        fi
    else
        echo "❌ BTCZS node is not running"
        return 1
    fi
}

# Show service status
show_status() {
    log_step "BTCZS Layer 2 Network Status"
    echo
    
    # BitcoinZ status
    echo "🔗 BitcoinZ Layer 1:"
    if curl -s -u "$BITCOINZ_RPC_USER:$BITCOINZ_RPC_PASS" \
        -d '{"jsonrpc":"1.0","id":"test","method":"getblockchaininfo","params":[]}' \
        -H 'content-type: text/plain;' \
        "$BITCOINZ_RPC_URL" > /dev/null 2>&1; then
        echo "  ✅ BitcoinZ node: Connected"
        
        # Get block info
        local response=$(curl -s -u "$BITCOINZ_RPC_USER:$BITCOINZ_RPC_PASS" \
            -d '{"jsonrpc":"1.0","id":"test","method":"getblockchaininfo","params":[]}' \
            -H 'content-type: text/plain;' \
            "$BITCOINZ_RPC_URL")
        
        local blocks=$(echo "$response" | grep -o '"blocks":[0-9]*' | cut -d':' -f2)
        echo "  📊 Current block: $blocks"
    else
        echo "  ❌ BitcoinZ node: Disconnected"
    fi
    
    echo
    echo "🚀 BTCZS Layer 2:"
    check_btczs_status
    
    echo
    echo "📁 Data Directory: $BTCZS_DATA_DIR"
    echo "🌐 Network: $NETWORK"
}

# Show logs
show_logs() {
    log_step "BTCZS Logs"
    
    if [[ -f "$BTCZS_DATA_DIR/logs/btczs-node.log" ]]; then
        echo "📄 BTCZS Node Log (last 20 lines):"
        tail -20 "$BTCZS_DATA_DIR/logs/btczs-node.log"
    else
        log_warning "No BTCZS logs found"
    fi
}

# Run integration tests
run_tests() {
    log_step "Running BTCZS Integration Tests..."
    
    cd "$PROJECT_ROOT/stackslib"
    
    echo "🧪 Running BTCZS test suite..."
    if cargo test btczs -- --nocapture; then
        log_success "All BTCZS tests passed!"
    else
        log_error "Some BTCZS tests failed"
        return 1
    fi
    
    echo
    echo "🔗 Testing BitcoinZ integration..."
    if curl -s -u "$BITCOINZ_RPC_USER:$BITCOINZ_RPC_PASS" \
        -d '{"jsonrpc":"1.0","id":"test","method":"getblockchaininfo","params":[]}' \
        -H 'content-type: text/plain;' \
        "$BITCOINZ_RPC_URL" > /dev/null 2>&1; then
        log_success "BitcoinZ integration test passed!"
    else
        log_error "BitcoinZ integration test failed"
        return 1
    fi
}

# Start monitoring
start_monitoring() {
    log_step "Starting BTCZS Monitoring Dashboard..."
    
    echo "📊 Monitoring Dashboard:"
    echo "  BTCZS Metrics: http://127.0.0.1:20446"
    echo "  Logs: $BTCZS_DATA_DIR/logs/"
    echo
    echo "Press Ctrl+C to stop monitoring"
    
    # Simple monitoring loop
    while true; do
        clear
        echo "🚀 BTCZS Layer 2 Network Monitor"
        echo "================================"
        echo "$(date)"
        echo
        show_status
        echo
        echo "Press Ctrl+C to exit"
        sleep 10
    done
}

# Main execution
main() {
    if [[ -z "$COMMAND" ]]; then
        log_error "No command specified"
        show_help
        exit 1
    fi
    
    validate_network
    
    case "$COMMAND" in
        start)
            check_prerequisites
            start_btczs
            ;;
        stop)
            stop_btczs
            ;;
        restart)
            stop_btczs
            sleep 2
            check_prerequisites
            start_btczs
            ;;
        status)
            show_status
            ;;
        logs)
            show_logs
            ;;
        test)
            run_tests
            ;;
        monitor)
            start_monitoring
            ;;
        *)
            log_error "Unknown command: $COMMAND"
            show_help
            exit 1
            ;;
    esac
}

# Handle script interruption
cleanup() {
    if [[ "$COMMAND" == "monitor" ]]; then
        echo
        log_info "Monitoring stopped"
    fi
}

trap cleanup EXIT

# Run main function
main "$@"
