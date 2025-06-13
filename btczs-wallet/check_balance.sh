#!/bin/bash

# Load wallet configuration
WALLET_CONFIG="$(dirname "$0")/wallet.json"
STACKS_ADDRESS=$(jq -r '.accounts.default.stacks_address' "$WALLET_CONFIG")
BTCZS_RPC_URL=$(jq -r '.settings.btczs_rpc_url' "$WALLET_CONFIG")

echo "🔍 Checking BTCZS Balance..."
echo "Address: $STACKS_ADDRESS"

# Check BTCZS balance
curl -s -X GET "$BTCZS_RPC_URL/v2/accounts/$STACKS_ADDRESS" | jq '.'
