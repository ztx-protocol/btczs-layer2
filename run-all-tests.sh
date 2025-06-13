#!/bin/bash

# BTCZS Complete Testing Suite Runner
# Runs all tests in sequence for comprehensive validation

echo "🔥 BTCZS Complete Testing Suite"
echo "==============================="
echo "Running comprehensive tests before VPS deployment"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }

# Test suite files
BASIC_TESTS="$SCRIPT_DIR/test-btczs-functions.sh"
POX_TESTS="$SCRIPT_DIR/test-pox-functions.sh"
PERFORMANCE_TESTS="$SCRIPT_DIR/test-performance.sh"

# Check if test files exist
check_test_files() {
    local missing_files=0
    
    if [ ! -f "$BASIC_TESTS" ]; then
        log_error "Basic tests file not found: $BASIC_TESTS"
        ((missing_files++))
    fi
    
    if [ ! -f "$POX_TESTS" ]; then
        log_error "PoX tests file not found: $POX_TESTS"
        ((missing_files++))
    fi
    
    if [ ! -f "$PERFORMANCE_TESTS" ]; then
        log_error "Performance tests file not found: $PERFORMANCE_TESTS"
        ((missing_files++))
    fi
    
    if [ $missing_files -gt 0 ]; then
        log_error "Missing test files. Please ensure all test scripts are present."
        exit 1
    fi
    
    log_success "All test files found"
}

# Pre-flight checks
pre_flight_checks() {
    log_info "Running pre-flight checks..."
    
    # Check if nodes are running
    if ! curl -s http://localhost:20443/v2/info > /dev/null; then
        log_error "BTCZS node not responding on localhost:20443"
        echo "Please start BTCZS node before running tests"
        exit 1
    fi
    
    if ! curl -s -u test:test http://localhost:1979 -d '{"jsonrpc":"1.0","method":"getblockcount","id":1}' > /dev/null; then
        log_error "BitcoinZ node not responding on localhost:1979"
        echo "Please start BitcoinZ node before running tests"
        exit 1
    fi
    
    # Check if web interface is running
    if ! curl -s http://localhost:3000 > /dev/null; then
        log_warning "Web interface not responding on localhost:3000"
        echo "Web interface tests may fail"
    fi
    
    log_success "Pre-flight checks passed"
}

# Run test suite
run_test_suite() {
    local test_file="$1"
    local test_name="$2"
    local start_time=$(date +%s)
    
    echo ""
    echo "=================================================="
    log_info "Starting: $test_name"
    echo "=================================================="
    
    if bash "$test_file"; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        log_success "$test_name completed successfully in ${duration}s"
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        log_error "$test_name failed after ${duration}s"
        return 1
    fi
}

# Main execution
main() {
    local total_start=$(date +%s)
    local failed_suites=0
    
    echo "Starting comprehensive BTCZS testing..."
    echo "Timestamp: $(date)"
    echo ""
    
    # Check prerequisites
    check_test_files
    pre_flight_checks
    
    echo ""
    echo "🚀 BEGINNING TEST EXECUTION"
    echo "==========================="
    
    # Run basic function tests
    if ! run_test_suite "$BASIC_TESTS" "Basic Function Tests"; then
        ((failed_suites++))
    fi
    
    # Run PoX function tests
    if ! run_test_suite "$POX_TESTS" "PoX Function Tests"; then
        ((failed_suites++))
    fi
    
    # Run performance tests
    if ! run_test_suite "$PERFORMANCE_TESTS" "Performance Tests"; then
        ((failed_suites++))
    fi
    
    # Final summary
    local total_end=$(date +%s)
    local total_duration=$((total_end - total_start))
    
    echo ""
    echo "=================================================="
    echo "🏁 FINAL TESTING SUMMARY"
    echo "=================================================="
    echo "Total execution time: ${total_duration}s"
    echo "Test suites run: 3"
    echo "Test suites failed: $failed_suites"
    
    if [ $failed_suites -eq 0 ]; then
        echo ""
        log_success "🎉 ALL TEST SUITES PASSED!"
        echo ""
        echo "✅ BTCZS is ready for VPS deployment"
        echo "✅ All core functions working"
        echo "✅ PoX system operational"
        echo "✅ Performance acceptable"
        echo ""
        echo "📋 Next Steps:"
        echo "1. Deploy to VPS"
        echo "2. Update network configurations"
        echo "3. Test with real wallets"
        echo "4. Begin staking operations"
        echo ""
        exit 0
    else
        echo ""
        log_error "❌ $failed_suites TEST SUITE(S) FAILED"
        echo ""
        echo "Please fix the issues before VPS deployment:"
        echo "• Check node configurations"
        echo "• Verify network connectivity"
        echo "• Review error logs"
        echo ""
        exit 1
    fi
}

# Handle script interruption
trap 'echo ""; log_warning "Testing interrupted by user"; exit 130' INT

# Run main function
main "$@"
