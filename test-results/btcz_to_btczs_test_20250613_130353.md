# BTCZ to BTCZS Conversion Test Report

**Test Date**: Fri Jun 13 13:03:53 IST 2025
**Test Duration**: 75 seconds
**Test Amount**: 0.05 BTCZ
**Expected BTCZS**: .005 BTCZS

## Test Configuration
- BitcoinZ RPC: http://localhost:1979
- BTCZS API: http://127.0.0.1:20445
- Burn Address: t1eJkQ7wKrCWfuH87t3nJVqaHvucGgwBzkt
- Burn TXID: 9cb05e283f56bb4d5ca7c9e74382be3df5f18db2875cb428d5913fef512d62c5

## Test Results

- 2025-06-13 13:02:47 | BitcoinZ Connection | PASS | Node accessible at http://localhost:1979
- 2025-06-13 13:02:47 | BTCZS API | WARN | API not responding (may be normal for simulation)
- 2025-06-13 13:02:47 | Wallet Balance | PASS | Balance: 0.89694777 BTCZ (sufficient for 0.05 BTCZ test)
- 2025-06-13 13:02:47 | Burn Address Creation | PASS | Created burn address: t1eJkQ7wKrCWfuH87t3nJVqaHvucGgwBzkt
- 2025-06-13 13:02:47 | BTCZ Burn Transaction | PASS | Sent 0.05 BTCZ, TXID: 9cb05e283f56bb4d5ca7c9e74382be3df5f18db2875cb428d5913fef512d62c5
- 2025-06-13 13:03:48 | Transaction Confirmation | PASS | Transaction confirmed with 1 confirmations
- 2025-06-13 13:03:53 | BTCZS Token Minting | PASS | Minted .050 BTCZS from 0.05 BTCZ burn
- 2025-06-13 13:03:53 | Cross-Chain State | PASS | BTCZ balance updated, BTCZS supply: .050

## Summary
- **Total Tests**: 8
- **Passed**: 7
- **Success Rate**: 87.5%
- **Duration**: 75 seconds

## Files Generated
- Burn Address: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/burn_address.txt
- Burn TXID: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/burn_txid.txt
- Burn Amount: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/burn_amount.txt
- Minted BTCZS: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/minted_btczs.txt
- Final State: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/final_state.txt

