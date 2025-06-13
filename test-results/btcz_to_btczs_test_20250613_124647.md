# BTCZ to BTCZS Conversion Test Report

**Test Date**: Fri Jun 13 12:46:47 IST 2025
**Test Duration**: 5 seconds
**Test Amount**: 0.1 BTCZ
**Expected BTCZS**: .01 BTCZS

## Test Configuration
- BitcoinZ RPC: http://localhost:1979
- BTCZS API: http://127.0.0.1:20445
- Burn Address: Not created
- Burn TXID: Not sent

## Test Results

- 2025-06-13 12:46:47 | BitcoinZ Connection | PASS | Node accessible at http://localhost:1979
- 2025-06-13 12:46:47 | BTCZS API | WARN | API not responding (may be normal for simulation)
- 2025-06-13 12:46:47 | Wallet Balance | PASS | Balance: 0.89695021 BTCZ (sufficient for 0.1 BTCZ test)
- 2025-06-13 12:46:47 | Burn Address Creation | FAIL | Failed to create burn address

## Summary
- **Total Tests**: 4
- **Passed**: 2
- **Success Rate**: 50.0%
- **Duration**: 5 seconds

## Files Generated
- Burn Address: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/burn_address.txt
- Burn TXID: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/burn_txid.txt
- Burn Amount: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/burn_amount.txt
- Minted BTCZS: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/minted_btczs.txt
- Final State: /Users/mac/Documents/augment-projects/layer2/btczs-core/test-results/final_state.txt

