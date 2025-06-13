#!/bin/bash

# 🥩 BTCZS Stacking Preparation Script
# Use this when you have BTCZS tokens to stack for BTCZ rewards

echo "🥩 BTCZS Stacking Preparation"
echo "============================"

# Load wallet configuration
WALLET_CONFIG="btczs-wallet/wallet.json"
BTCZS_ADDRESS=$(jq -r '.accounts.default.stacks_address' "$WALLET_CONFIG")
BITCOINZ_REWARD_ADDRESS=$(jq -r '.accounts.default.bitcoinz_reward_address' "$WALLET_CONFIG")
PRIVATE_KEY=$(jq -r '.accounts.default.private_key' "$WALLET_CONFIG")

echo "📊 Stacking Configuration:"
echo "BTCZS Address: $BTCZS_ADDRESS"
echo "BitcoinZ Reward Address: $BITCOINZ_REWARD_ADDRESS"
echo ""

echo "🎯 Stacking Requirements:"
echo "- Minimum: 100,000 BTCZS (100K BTCZS)"
echo "- Lock Period: 1-12 cycles (each cycle ~2 weeks)"
echo "- Rewards: Paid in BTCZ to your BitcoinZ address"
echo ""

echo "📝 To start stacking when you have BTCZS:"
echo "1. Check your BTCZS balance"
echo "2. Choose stacking amount (min 100K BTCZS)"
echo "3. Select number of cycles (1-12)"
echo "4. Submit stacking transaction"
echo ""

echo "⚠️  Note: You need BTCZS tokens first from mining rewards"
echo "✅ Your mining bid is active - wait for BTCZS rewards!"
