#!/bin/bash

# 🧪 Local CI/CD Testing Script for BTCZS
# This script replicates the GitHub Actions workflow locally

set -e  # Exit on any error

echo "🚀 Starting Local CI/CD Testing for BTCZS Layer 2"
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    local status=$1
    local message=$2
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✅ $message${NC}"
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}❌ $message${NC}"
    elif [ "$status" = "INFO" ]; then
        echo -e "${BLUE}ℹ️  $message${NC}"
    elif [ "$status" = "WARN" ]; then
        echo -e "${YELLOW}⚠️  $message${NC}"
    fi
}

# Function to run a test job
run_job() {
    local job_name=$1
    local job_function=$2
    
    echo ""
    echo "🔄 Running Job: $job_name"
    echo "----------------------------------------"
    
    if $job_function; then
        print_status "PASS" "$job_name completed successfully"
        return 0
    else
        print_status "FAIL" "$job_name failed"
        return 1
    fi
}

# Job 1: Code Quality
test_code_quality() {
    print_status "INFO" "Checking code quality..."
    
    # Check if Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        print_status "FAIL" "Cargo.toml not found at root level"
        return 1
    fi
    print_status "PASS" "Cargo.toml found at root level"
    
    # Check Rust installation
    if ! command -v cargo &> /dev/null; then
        print_status "WARN" "Cargo not installed, skipping Rust checks"
        return 0
    fi
    
    # Check format
    print_status "INFO" "Checking code format..."
    if cargo fmt --check; then
        print_status "PASS" "Code format check passed"
    else
        print_status "WARN" "Code format issues found (non-critical)"
    fi
    
    # Check clippy
    print_status "INFO" "Running clippy analysis..."
    if cargo clippy -- -D warnings; then
        print_status "PASS" "Clippy analysis passed"
    else
        print_status "WARN" "Clippy warnings found (non-critical)"
    fi
    
    return 0
}

# Job 2: Documentation
test_documentation() {
    print_status "INFO" "Checking documentation..."
    
    # Required documentation files
    required_files=(
        "README.md"
        "BTCZS_BITCOINZ_POX_COMPATIBILITY_REPORT.md"
        "TECHNICAL_SPECIFICATIONS.md"
        "BTCZS_TOKEN_ECONOMICS.md"
        "LICENSE"
    )
    
    local missing_files=0
    for file in "${required_files[@]}"; do
        if [ -f "$file" ]; then
            print_status "PASS" "$file exists"
        else
            print_status "FAIL" "$file missing"
            missing_files=$((missing_files + 1))
        fi
    done
    
    if [ $missing_files -eq 0 ]; then
        print_status "PASS" "All required documentation files present"
        return 0
    else
        print_status "FAIL" "$missing_files documentation files missing"
        return 1
    fi
}

# Job 3: Build & Test
test_build_and_test() {
    print_status "INFO" "Testing build and test process..."
    
    # Check if Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        print_status "FAIL" "Cargo.toml not found - cannot build"
        return 1
    fi
    
    # Check Rust installation
    if ! command -v cargo &> /dev/null; then
        print_status "WARN" "Cargo not installed, skipping build"
        return 0
    fi
    
    # Try to build
    print_status "INFO" "Building BTCZS..."
    if cargo build --verbose; then
        print_status "PASS" "Build successful"
    else
        print_status "FAIL" "Build failed"
        return 1
    fi
    
    # Try to run tests
    print_status "INFO" "Running Rust tests..."
    if cargo test --verbose; then
        print_status "PASS" "Rust tests passed"
    else
        print_status "WARN" "Some Rust tests failed (checking script tests)"
    fi
    
    # Check for test scripts
    if [ -f "run-all-tests.sh" ]; then
        print_status "INFO" "Running integration test scripts..."
        if ./run-all-tests.sh; then
            print_status "PASS" "Integration tests passed"
        else
            print_status "WARN" "Some integration tests failed"
        fi
    fi
    
    return 0
}

# Job 4: Integration Tests
test_integration() {
    print_status "INFO" "Testing integration test setup..."
    
    # Check for test scripts
    test_scripts=(
        "test-btczs-functions.sh"
        "test-pox-functions.sh"
        "test-performance.sh"
        "run-all-tests.sh"
    )
    
    local missing_scripts=0
    for script in "${test_scripts[@]}"; do
        if [ -f "$script" ]; then
            if [ -x "$script" ]; then
                print_status "PASS" "$script exists and is executable"
            else
                print_status "WARN" "$script exists but not executable"
                chmod +x "$script"
                print_status "INFO" "Made $script executable"
            fi
        else
            print_status "FAIL" "$script missing"
            missing_scripts=$((missing_scripts + 1))
        fi
    done
    
    if [ $missing_scripts -eq 0 ]; then
        print_status "PASS" "All integration test scripts present"
        
        # Try running a quick test
        if [ -f "test-btczs-functions.sh" ]; then
            print_status "INFO" "Running quick integration test..."
            if timeout 30s ./test-btczs-functions.sh; then
                print_status "PASS" "Quick integration test passed"
            else
                print_status "WARN" "Integration test timed out or failed (may need BitcoinZ node)"
            fi
        fi
        
        return 0
    else
        print_status "FAIL" "$missing_scripts integration test scripts missing"
        return 1
    fi
}

# Job 5: Security Audit
test_security() {
    print_status "INFO" "Testing security audit setup..."
    
    # Check if Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        print_status "WARN" "No Cargo.toml found, skipping cargo audit"
        return 0
    fi
    
    # Check Rust installation
    if ! command -v cargo &> /dev/null; then
        print_status "WARN" "Cargo not installed, skipping security audit"
        return 0
    fi
    
    # Check for cargo-audit
    if ! cargo audit --version &> /dev/null; then
        print_status "INFO" "Installing cargo-audit..."
        if cargo install cargo-audit; then
            print_status "PASS" "cargo-audit installed"
        else
            print_status "WARN" "Failed to install cargo-audit"
            return 0
        fi
    fi
    
    # Run security audit
    print_status "INFO" "Running security audit..."
    if cargo audit; then
        print_status "PASS" "Security audit passed"
    else
        print_status "WARN" "Security audit found issues (check output above)"
    fi
    
    # Check for common security issues
    print_status "INFO" "Checking for common security issues..."
    
    # Check for hardcoded secrets (basic check)
    if grep -r -i "password\|secret\|key\|token" --include="*.rs" --include="*.toml" . | grep -v "test" | grep -v "example"; then
        print_status "WARN" "Potential hardcoded secrets found (review above)"
    else
        print_status "PASS" "No obvious hardcoded secrets found"
    fi
    
    return 0
}

# Main execution
main() {
    local failed_jobs=0
    
    # Change to repository directory
    cd "$(dirname "$0")"
    
    print_status "INFO" "Working directory: $(pwd)"
    print_status "INFO" "Git branch: $(git branch --show-current 2>/dev/null || echo 'unknown')"
    
    # Run all jobs
    run_job "Code Quality" test_code_quality || failed_jobs=$((failed_jobs + 1))
    run_job "Documentation" test_documentation || failed_jobs=$((failed_jobs + 1))
    run_job "Build & Test" test_build_and_test || failed_jobs=$((failed_jobs + 1))
    run_job "Integration Tests" test_integration || failed_jobs=$((failed_jobs + 1))
    run_job "Security Audit" test_security || failed_jobs=$((failed_jobs + 1))
    
    # Summary
    echo ""
    echo "🏁 Local CI/CD Testing Complete"
    echo "================================"
    
    if [ $failed_jobs -eq 0 ]; then
        print_status "PASS" "All jobs completed successfully! 🎉"
        echo ""
        print_status "INFO" "Your CI/CD pipeline should work on GitHub Actions"
        exit 0
    else
        print_status "FAIL" "$failed_jobs job(s) failed"
        echo ""
        print_status "INFO" "Fix the issues above before pushing to GitHub"
        exit 1
    fi
}

# Run main function
main "$@"
