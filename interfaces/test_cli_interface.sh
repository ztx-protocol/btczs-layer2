#!/bin/bash
# Test BTCZS CLI Interface

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLI="$SCRIPT_DIR/btczs-cli.py"

echo "🧪 Testing BTCZS CLI Interface..."
echo ""

echo "📊 Getting PoX information..."
python3 "$CLI" pox-info
echo ""

echo "🔒 Getting stacker info for test address..."
python3 "$CLI" stacker-info SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7
echo ""

echo "✅ CLI interface test complete!"
