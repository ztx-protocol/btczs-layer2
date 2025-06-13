// Copyright (c) 2009-2010 Satoshi Nakamoto
// Copyright (c) 2009-2014 The Bitcoin Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING or https://www.opensource.org/licenses/mit-license.php .

#include "validationinterface.h"

#include "chainparams.h"
#include "init.h"
#include "main.h"
#include "txmempool.h"
#include "ui_interface.h"

#include <boost/thread.hpp>

#include <chrono>
#include <thread>

using namespace boost::placeholders;

static CMainSignals g_signals;

CMainSignals& GetMainSignals()
{
    return g_signals;
}

void RegisterValidationInterface(CValidationInterface* pwalletIn) {
    g_signals.UpdatedBlockTip.connect(boost::bind(&CValidationInterface::UpdatedBlockTip, pwalletIn, _1));
    g_signals.SyncTransaction.connect(boost::bind(&CValidationInterface::SyncTransaction, pwalletIn, _1, _2));
    g_signals.EraseTransaction.connect(boost::bind(&CValidationInterface::EraseFromWallet, pwalletIn, _1));
    g_signals.UpdatedTransaction.connect(boost::bind(&CValidationInterface::UpdatedTransaction, pwalletIn, _1));
    g_signals.ChainTip.connect(boost::bind(&CValidationInterface::ChainTip, pwalletIn, _1, _2, _3));
    g_signals.Inventory.connect(boost::bind(&CValidationInterface::Inventory, pwalletIn, _1));
    g_signals.Broadcast.connect(boost::bind(&CValidationInterface::ResendWalletTransactions, pwalletIn, _1));
    g_signals.BlockChecked.connect(boost::bind(&CValidationInterface::BlockChecked, pwalletIn, _1, _2));
    g_signals.ScriptForMining.connect(boost::bind(&CValidationInterface::GetScriptForMining, pwalletIn, _1));
    g_signals.BlockFound.connect(boost::bind(&CValidationInterface::ResetRequestCount, pwalletIn, _1));
}

void UnregisterValidationInterface(CValidationInterface* pwalletIn) {
    g_signals.BlockFound.disconnect(boost::bind(&CValidationInterface::ResetRequestCount, pwalletIn, _1));
    g_signals.ScriptForMining.disconnect(boost::bind(&CValidationInterface::GetScriptForMining, pwalletIn, _1));
    g_signals.BlockChecked.disconnect(boost::bind(&CValidationInterface::BlockChecked, pwalletIn, _1, _2));
    g_signals.Broadcast.disconnect(boost::bind(&CValidationInterface::ResendWalletTransactions, pwalletIn, _1));
    g_signals.Inventory.disconnect(boost::bind(&CValidationInterface::Inventory, pwalletIn, _1));
    g_signals.ChainTip.disconnect(boost::bind(&CValidationInterface::ChainTip, pwalletIn, _1, _2, _3));
    g_signals.UpdatedTransaction.disconnect(boost::bind(&CValidationInterface::UpdatedTransaction, pwalletIn, _1));
    g_signals.EraseTransaction.disconnect(boost::bind(&CValidationInterface::EraseFromWallet, pwalletIn, _1));
    g_signals.SyncTransaction.disconnect(boost::bind(&CValidationInterface::SyncTransaction, pwalletIn, _1, _2));
    g_signals.UpdatedBlockTip.disconnect(boost::bind(&CValidationInterface::UpdatedBlockTip, pwalletIn, _1));
}

void UnregisterAllValidationInterfaces() {
    g_signals.BlockFound.disconnect_all_slots();
    g_signals.ScriptForMining.disconnect_all_slots();
    g_signals.BlockChecked.disconnect_all_slots();
    g_signals.Broadcast.disconnect_all_slots();
    g_signals.Inventory.disconnect_all_slots();
    g_signals.ChainTip.disconnect_all_slots();
    g_signals.UpdatedTransaction.disconnect_all_slots();
    g_signals.EraseTransaction.disconnect_all_slots();
    g_signals.SyncTransaction.disconnect_all_slots();
    g_signals.UpdatedBlockTip.disconnect_all_slots();
}

void SyncWithWallets(const CTransaction &tx, const CBlock *pblock) {
    g_signals.SyncTransaction(tx, pblock);
}

struct CachedBlockData {
    CBlockIndex *pindex;
    std::pair<SproutMerkleTree, SaplingMerkleTree> oldTrees;
    std::list<CTransaction> txConflicted;

    CachedBlockData(
        CBlockIndex *pindex,
        std::pair<SproutMerkleTree, SaplingMerkleTree> oldTrees,
        std::list<CTransaction> txConflicted):
        pindex(pindex), oldTrees(oldTrees), txConflicted(txConflicted) {}
};

void ThreadNotifyWallets(CBlockIndex *pindexLastTip)
{
    // If pindexLastTip == nullptr, the wallet is at genesis.
    // However, the genesis block is not loaded synchronously.
    // We need to wait for ThreadImport to finish.
    while (pindexLastTip == nullptr) {
        {
            LOCK(cs_main);
            pindexLastTip = chainActive.Genesis();
        }
        MilliSleep(50);
    }

    while (true) {
        // Run the notifier on an integer second in the steady clock.
        auto now = std::chrono::steady_clock::now().time_since_epoch();
        auto nextFire = std::chrono::duration_cast<std::chrono::seconds>(
            now + std::chrono::seconds(1));
        std::this_thread::sleep_until(
            std::chrono::time_point<std::chrono::steady_clock>(nextFire));

        boost::this_thread::interruption_point();

        auto chainParams = Params();

        //
        // Collect all the state we require
        //

        // The common ancestor between the last chain tip we notified and the
        // current chain tip.
        const CBlockIndex *pindexFork;
        // The stack of blocks we will notify as having been connected.
        // Pushed in reverse, popped in order.
        std::vector<CachedBlockData> blockStack;
        // Sequence number indicating that we have notified wallets of transactions up to
        // the ConnectBlock() call that generated this sequence number.
        std::optional<uint64_t> chainNotifiedSequence;
        // Transactions that have been recently added to the mempool.
        std::pair<std::vector<CTransaction>, uint64_t> recentlyAdded;

        {
            LOCK(cs_main);

            // Figure out the path from the last block we notified to the
            // current chain tip.
            CBlockIndex *pindex = chainActive.Tip();
            pindexFork = chainActive.FindFork(pindexLastTip);

            // Iterate backwards over the connected blocks until we have at
            // most WALLET_NOTIFY_MAX_BLOCKS to process.
            while (pindex && pindex->nHeight > pindexFork->nHeight + WALLET_NOTIFY_MAX_BLOCKS) {
                pindex = pindex->pprev;
            }

            // Iterate backwards over the connected blocks we need to notify.
            bool originalTipAtFork = pindex && pindex == pindexFork;
            while (pindex && pindex != pindexFork) {
                // Get the Sprout commitment tree as of the start of this block.
                SproutMerkleTree oldSproutTree;
                assert(pcoinsTip->GetSproutAnchorAt(pindex->hashSproutAnchor, oldSproutTree));

                // Get the Sapling commitment tree as of the start of this block.
                // We can get this from the `hashFinalSaplingRoot` of the last block
                // However, this is only reliable if the last block was on or after
                // the Sapling activation height. Otherwise, the last anchor was the
                // empty root.
                SaplingMerkleTree oldSaplingTree;
                if (chainParams.GetConsensus().NetworkUpgradeActive(
                    pindex->pprev->nHeight, Consensus::UPGRADE_SAPLING)) {
                    assert(pcoinsTip->GetSaplingAnchorAt(
                        pindex->pprev->hashFinalSaplingRoot, oldSaplingTree));
                } else {
                    assert(pcoinsTip->GetSaplingAnchorAt(SaplingMerkleTree::empty_root(), oldSaplingTree));
                }

                // Fetch recently-conflicted transactions. These will include any
                // block that has been connected since the last cycle, but we only
                // notify for the conflicts created by the current active chain.
                auto recentlyConflicted = TakeRecentlyConflicted(pindex);

                blockStack.emplace_back(
                    pindex,
                    std::make_pair(oldSproutTree, oldSaplingTree),
                    recentlyConflicted.first);

                chainNotifiedSequence = recentlyConflicted.second;

                pindex = pindex->pprev;
            }

            // This conditional can be true in the case that in the interval
            // since the last second-boundary, two reorgs occurred: one that
            // shifted over to a different chain history, and then a second
            // that returned the chain to the original pre-reorg tip.  This
            // should never occur unless a caller has manually used
            // `invalidateblock` to force the second reorg or we have a long
            // persistent set of dueling chains. In such a case, wallets may
            // not be fully notified of conflicted transactions, but they will
            // still have a correct view of the current main chain, and they
            // will still be notified properly of the current state of
            // transactions in the mempool.
            if (originalTipAtFork) {
                chainNotifiedSequence = GetChainConnectedSequence();
            }
            if (chainNotifiedSequence.has_value()) {
                recentlyAdded = mempool.DrainRecentlyAdded();
            }
        }

        //
        // Execute wallet logic based on the collected state. We MUST NOT take
        // the cs_main or mempool.cs locks again until after the next sleep;
        // doing so introduces a locking side-channel between this code and the
        // network message processing thread.
        //

        // Notify block disconnects
        while (pindexLastTip && pindexLastTip != pindexFork) {
            // Read block from disk.
            CBlock block;
            if (!ReadBlockFromDisk(block, pindexLastTip, chainParams.GetConsensus())) {
                LogPrintf(
                        "*** %s: Failed to read block %s while notifying wallets of block disconnects",
                        __func__, pindexLastTip->GetBlockHash().GetHex());
                uiInterface.ThreadSafeMessageBox(
                    _("Error: A fatal internal error occurred, see debug.log for details"),
                    "", CClientUIInterface::MSG_ERROR);
                StartShutdown();
                return;
            }

            // Let wallets know transactions went from 1-confirmed to
            // 0-confirmed or conflicted:
            for (const CTransaction &tx : block.vtx) {
                SyncWithWallets(tx, NULL);
            }
            // Update cached incremental witnesses
            // This will take the cs_main lock in order to obtain the CBlockLocator
            // used by `SetBestChain`, but as that write only occurs once every
            // WRITE_WITNESS_INTERVAL * 1000000 microseconds this should not be
            // exploitable as a timing channel.
            GetMainSignals().ChainTip(pindexLastTip, &block, std::nullopt);

            // On to the next block!
            pindexLastTip = pindexLastTip->pprev;
        }

        // Notify block connections
        while (!blockStack.empty()) {
            auto blockData = blockStack.back();
            blockStack.pop_back();

            // Read block from disk.
            CBlock block;
            if (!ReadBlockFromDisk(block, blockData.pindex, chainParams.GetConsensus())) {
                LogPrintf(
                        "*** %s: Failed to read block %s while notifying wallets of block connects",
                        __func__, blockData.pindex->GetBlockHash().GetHex());
                uiInterface.ThreadSafeMessageBox(
                    _("Error: A fatal internal error occurred, see debug.log for details"),
                    "", CClientUIInterface::MSG_ERROR);
                StartShutdown();
                return;
            }

            // Tell wallet about transactions that went from mempool
            // to conflicted:
            for (const CTransaction &tx : blockData.txConflicted) {
                SyncWithWallets(tx, NULL);
            }
            // ... and about transactions that got confirmed:
            for (const CTransaction &tx : block.vtx) {
                SyncWithWallets(tx, &block);
            }
            // Update cached incremental witnesses
            // This will take the cs_main lock in order to obtain the CBlockLocator
            // used by `SetBestChain`, but as that write only occurs once every
            // WRITE_WITNESS_INTERVAL * 1000000 microseconds this should not be
            // exploitable as a timing channel.
            GetMainSignals().ChainTip(blockData.pindex, &block, blockData.oldTrees);

            // This block is done!
            pindexLastTip = blockData.pindex;
        }

        // Notify transactions in the mempool
        for (auto tx : recentlyAdded.first) {
            try {
                SyncWithWallets(tx, NULL);
            } catch (const boost::thread_interrupted&) {
                throw;
            } catch (const std::exception& e) {
                PrintExceptionContinue(&e, "ThreadNotifyWallets()");
            } catch (...) {
                PrintExceptionContinue(NULL, "ThreadNotifyWallets()");
            }
        }

        // Update the notified sequence numbers. We only need this in regtest mode,
        // and should not lock on cs or cs_main here otherwise.
        if (chainParams.NetworkIDString() == "regtest") {
            if (chainNotifiedSequence.has_value()) {
                SetChainNotifiedSequence(chainNotifiedSequence.value());
            }
            if (recentlyAdded.second > 0) {
                mempool.SetNotifiedSequence(recentlyAdded.second);
            }
        }
    }
}
