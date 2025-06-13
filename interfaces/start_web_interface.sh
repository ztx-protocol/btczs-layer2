#!/bin/bash
# Start BTCZS Web Interface

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/btczs-web-interface"

echo "🚀 Starting BTCZS Web Interface..."
echo "📍 URL: http://localhost:3000"
echo "🔥 PoX Interface for BitcoinZ Layer 2"
echo ""

npm start
