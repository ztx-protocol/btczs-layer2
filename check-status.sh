#!/bin/bash

# BTCZS Quick Status Check
# Fast overview of system status

echo "🔍 BTCZS System Status Check"
echo "============================"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[ONLINE]${NC} $1"; }
log_error() { echo -e "${RED}[OFFLINE]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }

# Quick RPC helper
quick_rpc() {
    curl -s -m 5 "$@" 2>/dev/null
}

echo ""
echo "🔗 Node Connectivity"
echo "===================="

# Check BTCZS Node
if quick_rpc http://localhost:20443/v2/info | grep -q "stacks_tip_height"; then
    BTCZS_HEIGHT=$(quick_rpc http://localhost:20443/v2/info | jq -r '.stacks_tip_height' 2>/dev/null)
    log_success "BTCZS Node (Height: $BTCZS_HEIGHT)"
else
    log_error "BTCZS Node (localhost:20443)"
fi

# Check BitcoinZ Node
if quick_rpc -u test:test -H "Content-Type: application/json" -d '{"jsonrpc":"1.0","method":"getblockcount","id":1}' http://localhost:1979 | grep -q "result"; then
    BITCOINZ_HEIGHT=$(quick_rpc -u test:test -H "Content-Type: application/json" -d '{"jsonrpc":"1.0","method":"getblockcount","id":1}' http://localhost:1979 | jq -r '.result' 2>/dev/null)
    log_success "BitcoinZ Node (Height: $BITCOINZ_HEIGHT)"
else
    log_error "BitcoinZ Node (localhost:1979)"
fi

# Check Web Interface
if quick_rpc http://localhost:3000 | grep -q "BTCZS"; then
    log_success "Web Interface (localhost:3000)"
else
    log_error "Web Interface (localhost:3000)"
fi

echo ""
echo "⚙️ Process Status"
echo "================="

# Check for running processes
BTCZS_PID=$(ps aux | grep "stacks-node" | grep -v grep | awk '{print $2}' | head -1)
if [ -n "$BTCZS_PID" ]; then
    log_success "BTCZS Process (PID: $BTCZS_PID)"
else
    log_error "BTCZS Process (not found)"
fi

BITCOINZ_PID=$(ps aux | grep "bitcoinzd" | grep -v grep | awk '{print $2}' | head -1)
if [ -n "$BITCOINZ_PID" ]; then
    log_success "BitcoinZ Process (PID: $BITCOINZ_PID)"
else
    log_error "BitcoinZ Process (not found)"
fi

WEB_PID=$(ps aux | grep "npm.*start\|node.*start" | grep -v grep | awk '{print $2}' | head -1)
if [ -n "$WEB_PID" ]; then
    log_success "Web Interface Process (PID: $WEB_PID)"
else
    log_error "Web Interface Process (not found)"
fi

echo ""
echo "📊 Quick Stats"
echo "=============="

if [ -n "$BTCZS_HEIGHT" ] && [ -n "$BITCOINZ_HEIGHT" ]; then
    SYNC_DIFF=$((BITCOINZ_HEIGHT - BTCZS_HEIGHT))
    if [ $SYNC_DIFF -le 5 ]; then
        log_success "Synchronization (diff: $SYNC_DIFF blocks)"
    else
        log_warning "Synchronization (diff: $SYNC_DIFF blocks)"
    fi
fi

echo ""
echo "🚀 Ready for Testing?"
echo "===================="

READY=true

if [ -z "$BTCZS_PID" ]; then
    echo "❌ BTCZS node not running"
    READY=false
fi

if [ -z "$BITCOINZ_PID" ]; then
    echo "❌ BitcoinZ node not running"
    READY=false
fi

if [ -z "$BTCZS_HEIGHT" ]; then
    echo "❌ BTCZS node not responding"
    READY=false
fi

if [ -z "$BITCOINZ_HEIGHT" ]; then
    echo "❌ BitcoinZ node not responding"
    READY=false
fi

if [ "$READY" = true ]; then
    echo ""
    log_success "✅ System ready for testing!"
    echo ""
    echo "Run comprehensive tests with:"
    echo "  ./run-all-tests.sh"
    echo ""
    echo "Or run individual tests:"
    echo "  ./test-btczs-functions.sh    # Basic functionality"
    echo "  ./test-pox-functions.sh      # PoX system"
    echo "  ./test-performance.sh        # Performance"
else
    echo ""
    log_error "❌ System not ready for testing"
    echo ""
    echo "Please start missing services and try again."
fi
