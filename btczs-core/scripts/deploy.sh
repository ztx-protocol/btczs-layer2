#!/bin/bash

# BTCZS Deployment Script
# This script automates the deployment of BTCZS nodes

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEFAULT_ENVIRONMENT="staging"
DEFAULT_CONFIG_DIR="$PROJECT_ROOT/config"
DEFAULT_DATA_DIR="/var/lib/btczs"

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
BTCZS Deployment Script

Usage: $0 [OPTIONS] COMMAND

Commands:
    deploy      Deploy BTCZS nodes
    start       Start BTCZS services
    stop        Stop BTCZS services
    restart     Restart BTCZS services
    status      Check service status
    logs        Show service logs
    update      Update BTCZS to latest version
    backup      Create system backup
    restore     Restore from backup

Options:
    -e, --environment ENV    Deployment environment (production, staging, development, local)
    -c, --config DIR         Configuration directory (default: $DEFAULT_CONFIG_DIR)
    -d, --data DIR           Data directory (default: $DEFAULT_DATA_DIR)
    -n, --nodes COUNT        Number of nodes to deploy (default: 1)
    -h, --help               Show this help message

Examples:
    $0 deploy                           # Deploy staging environment
    $0 -e production deploy             # Deploy production environment
    $0 -n 3 deploy                      # Deploy 3 nodes
    $0 start                            # Start services
    $0 logs                             # Show logs

EOF
}

# Parse command line arguments
ENVIRONMENT="$DEFAULT_ENVIRONMENT"
CONFIG_DIR="$DEFAULT_CONFIG_DIR"
DATA_DIR="$DEFAULT_DATA_DIR"
NODE_COUNT=1
COMMAND=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -e|--environment)
            ENVIRONMENT="$2"
            shift 2
            ;;
        -c|--config)
            CONFIG_DIR="$2"
            shift 2
            ;;
        -d|--data)
            DATA_DIR="$2"
            shift 2
            ;;
        -n|--nodes)
            NODE_COUNT="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        deploy|start|stop|restart|status|logs|update|backup|restore)
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

# Validate environment
validate_environment() {
    case "$ENVIRONMENT" in
        production|staging|development|local)
            log_info "Using environment: $ENVIRONMENT"
            ;;
        *)
            log_error "Invalid environment: $ENVIRONMENT"
            log_error "Valid environments: production, staging, development, local"
            exit 1
            ;;
    esac
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if running as root for production
    if [[ "$ENVIRONMENT" == "production" && $EUID -ne 0 ]]; then
        log_error "Production deployment requires root privileges"
        exit 1
    fi
    
    # Check required commands
    local required_commands=("cargo" "systemctl" "docker")
    for cmd in "${required_commands[@]}"; do
        if ! command -v "$cmd" &> /dev/null; then
            log_warning "$cmd is not installed"
        fi
    done
    
    # Check BitcoinZ node connectivity
    if ! curl -s "http://localhost:1979" &> /dev/null; then
        log_warning "BitcoinZ node not accessible at localhost:1979"
    fi
    
    log_success "Prerequisites check completed"
}

# Build BTCZS
build_btczs() {
    log_info "Building BTCZS..."
    
    cd "$PROJECT_ROOT"
    
    # Clean previous builds
    cargo clean
    
    # Build release version
    if cargo build --release; then
        log_success "BTCZS build completed"
    else
        log_error "BTCZS build failed"
        exit 1
    fi
}

# Deploy BTCZS nodes
deploy_btczs() {
    log_info "Deploying BTCZS nodes..."
    
    # Create directories
    sudo mkdir -p "$DATA_DIR"
    sudo mkdir -p "/etc/btczs"
    sudo mkdir -p "/var/log/btczs"
    
    # Copy configuration files
    if [[ -d "$CONFIG_DIR/$ENVIRONMENT" ]]; then
        sudo cp -r "$CONFIG_DIR/$ENVIRONMENT"/* "/etc/btczs/"
        log_success "Configuration files copied"
    else
        log_warning "No configuration found for $ENVIRONMENT"
    fi
    
    # Copy binary
    sudo cp "$PROJECT_ROOT/target/release/btczs-node" "/usr/local/bin/"
    sudo chmod +x "/usr/local/bin/btczs-node"
    log_success "BTCZS binary installed"
    
    # Create systemd service
    create_systemd_service
    
    # Deploy multiple nodes if requested
    for ((i=1; i<=NODE_COUNT; i++)); do
        deploy_node "$i"
    done
    
    log_success "BTCZS deployment completed"
}

# Create systemd service
create_systemd_service() {
    log_info "Creating systemd service..."
    
    cat << EOF | sudo tee /etc/systemd/system/btczs-node@.service > /dev/null
[Unit]
Description=BTCZS Node %i
After=network.target
Wants=network.target

[Service]
Type=simple
User=btczs
Group=btczs
ExecStart=/usr/local/bin/btczs-node --config=/etc/btczs/node-%i.toml
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=btczs-node-%i

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$DATA_DIR /var/log/btczs

[Install]
WantedBy=multi-user.target
EOF
    
    sudo systemctl daemon-reload
    log_success "Systemd service created"
}

# Deploy individual node
deploy_node() {
    local node_id="$1"
    log_info "Deploying node $node_id..."
    
    # Create node-specific data directory
    local node_data_dir="$DATA_DIR/node-$node_id"
    sudo mkdir -p "$node_data_dir"
    
    # Create node-specific configuration
    create_node_config "$node_id"
    
    # Enable and start service
    sudo systemctl enable "btczs-node@$node_id"
    sudo systemctl start "btczs-node@$node_id"
    
    log_success "Node $node_id deployed"
}

# Create node configuration
create_node_config() {
    local node_id="$1"
    local config_file="/etc/btczs/node-$node_id.toml"
    
    log_info "Creating configuration for node $node_id..."
    
    # Calculate ports (base port + node_id)
    local rpc_port=$((20443 + node_id - 1))
    local p2p_port=$((20444 + node_id - 1))
    
    cat << EOF | sudo tee "$config_file" > /dev/null
# BTCZS Node $node_id Configuration

[network]
environment = "$ENVIRONMENT"
rpc_port = $rpc_port
p2p_port = $p2p_port

[bitcoinz]
rpc_url = "http://localhost:1979"
rpc_username = "btczs"
rpc_password = "btczs"

[storage]
data_dir = "$DATA_DIR/node-$node_id"

[logging]
level = "info"
file = "/var/log/btczs/node-$node_id.log"
EOF
    
    log_success "Configuration created for node $node_id"
}

# Start services
start_services() {
    log_info "Starting BTCZS services..."
    
    for ((i=1; i<=NODE_COUNT; i++)); do
        sudo systemctl start "btczs-node@$i"
    done
    
    log_success "Services started"
}

# Stop services
stop_services() {
    log_info "Stopping BTCZS services..."
    
    for ((i=1; i<=NODE_COUNT; i++)); do
        sudo systemctl stop "btczs-node@$i"
    done
    
    log_success "Services stopped"
}

# Restart services
restart_services() {
    log_info "Restarting BTCZS services..."
    
    for ((i=1; i<=NODE_COUNT; i++)); do
        sudo systemctl restart "btczs-node@$i"
    done
    
    log_success "Services restarted"
}

# Check service status
check_status() {
    log_info "Checking service status..."
    
    for ((i=1; i<=NODE_COUNT; i++)); do
        echo "Node $i:"
        sudo systemctl status "btczs-node@$i" --no-pager -l
        echo
    done
}

# Show logs
show_logs() {
    log_info "Showing service logs..."
    
    if [[ $NODE_COUNT -eq 1 ]]; then
        sudo journalctl -u "btczs-node@1" -f
    else
        log_info "Multiple nodes detected. Showing logs for all nodes:"
        for ((i=1; i<=NODE_COUNT; i++)); do
            echo "=== Node $i Logs ==="
            sudo journalctl -u "btczs-node@$i" --no-pager -n 20
            echo
        done
    fi
}

# Update BTCZS
update_btczs() {
    log_info "Updating BTCZS..."
    
    # Stop services
    stop_services
    
    # Build new version
    build_btczs
    
    # Install new binary
    sudo cp "$PROJECT_ROOT/target/release/btczs-node" "/usr/local/bin/"
    
    # Start services
    start_services
    
    log_success "BTCZS updated"
}

# Create backup
create_backup() {
    log_info "Creating backup..."
    
    local backup_dir="/var/backups/btczs"
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local backup_file="$backup_dir/btczs_backup_$timestamp.tar.gz"
    
    sudo mkdir -p "$backup_dir"
    
    # Create backup
    sudo tar -czf "$backup_file" \
        -C / \
        --exclude="$DATA_DIR/*/chainstate" \
        "etc/btczs" \
        "var/lib/btczs" \
        "var/log/btczs"
    
    log_success "Backup created: $backup_file"
}

# Restore from backup
restore_backup() {
    log_info "Restoring from backup..."
    
    # This is a placeholder - implement based on specific backup format
    log_warning "Restore functionality not yet implemented"
    log_info "Manual restore process:"
    log_info "1. Stop BTCZS services"
    log_info "2. Extract backup to root filesystem"
    log_info "3. Restart BTCZS services"
}

# Main execution
main() {
    if [[ -z "$COMMAND" ]]; then
        log_error "No command specified"
        show_help
        exit 1
    fi
    
    validate_environment
    check_prerequisites
    
    case "$COMMAND" in
        deploy)
            build_btczs
            deploy_btczs
            ;;
        start)
            start_services
            ;;
        stop)
            stop_services
            ;;
        restart)
            restart_services
            ;;
        status)
            check_status
            ;;
        logs)
            show_logs
            ;;
        update)
            update_btczs
            ;;
        backup)
            create_backup
            ;;
        restore)
            restore_backup
            ;;
        *)
            log_error "Unknown command: $COMMAND"
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
