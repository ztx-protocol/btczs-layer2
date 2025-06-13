import React, { useState, useEffect } from 'react';

// BTCZS Configuration
const BTCZS_CONFIG = {
  btczsRpcUrl: 'http://localhost:20443',
  bitcoinzRpcUrl: 'http://localhost:1979',
  bitcoinzRpcUser: 'any',
  bitcoinzRpcPass: 'any',
  network: 'mainnet' as const,
};

interface PoxInfo {
  reward_cycle_id: number;
  next_reward_cycle_in: number;
  min_amount_ustx: string;
  reward_cycle_length: number;
  current_btczs_block_height: number;
  current_bitcoinz_block_height: number;
  total_stacked_ustx: string;
  is_pox_active: boolean;
}

interface StackerInfo {
  is_stacking: boolean;
  stacked_amount?: string;
  first_reward_cycle?: number;
  lock_period?: number;
  unlock_height?: number;
  bitcoinz_reward_address?: string;
  total_btcz_rewards?: string;
}

interface MiningBid {
  bid_amount: string;
  miner_address: string;
  target_block: number;
  bid_txid: string;
  status: 'pending' | 'confirmed' | 'won' | 'lost';
}

function App() {
  const [activeTab, setActiveTab] = useState<'stacking' | 'mining' | 'rewards'>('stacking');
  const [userAddress, setUserAddress] = useState('');
  const [poxInfo, setPoxInfo] = useState<PoxInfo | null>(null);
  const [stackerInfo, setStackerInfo] = useState<StackerInfo | null>(null);
  const [miningBids, setMiningBids] = useState<MiningBid[]>([]);
  const [loading, setLoading] = useState(false);

  // Stacking form state
  const [stackAmount, setStackAmount] = useState('');
  const [bitcoinzAddress, setBitcoinzAddress] = useState('');
  const [lockCycles, setLockCycles] = useState(1);
  const [privateKey, setPrivateKey] = useState('');

  // Mining form state
  const [bidAmount, setBidAmount] = useState('');
  const [miningPrivateKey, setMiningPrivateKey] = useState('');

  useEffect(() => {
    if (userAddress) {
      loadPoxInfo();
      loadStackerInfo();
      loadMiningBids();
    }
  }, [userAddress]);

  const loadPoxInfo = async () => {
    try {
      setLoading(true);
      // Simulate PoX info (in real implementation, this would call BTCZS RPC)
      const mockPoxInfo: PoxInfo = {
        reward_cycle_id: 42,
        next_reward_cycle_in: 1500,
        min_amount_ustx: '100000000000', // 100,000 STX
        reward_cycle_length: 2016,
        current_btczs_block_height: 85000,
        current_bitcoinz_block_height: 1577773,
        total_stacked_ustx: '500000000000', // 500,000 STX
        is_pox_active: true,
      };
      setPoxInfo(mockPoxInfo);
    } catch (error) {
      console.error('Failed to load PoX info:', error);
    } finally {
      setLoading(false);
    }
  };

  const loadStackerInfo = async () => {
    try {
      // Simulate stacker info
      const mockStackerInfo: StackerInfo = {
        is_stacking: false,
        total_btcz_rewards: '0',
      };
      setStackerInfo(mockStackerInfo);
    } catch (error) {
      console.error('Failed to load stacker info:', error);
    }
  };

  const loadMiningBids = async () => {
    try {
      // Simulate mining bids
      const mockBids: MiningBid[] = [
        {
          bid_amount: '1000000', // 0.01 BTCZ
          miner_address: 'SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7',
          target_block: 85001,
          bid_txid: '0x1234...abcd',
          status: 'pending',
        },
      ];
      setMiningBids(mockBids);
    } catch (error) {
      console.error('Failed to load mining bids:', error);
    }
  };

  const handleStackStx = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!stackAmount || !bitcoinzAddress || !privateKey) {
      alert('Please fill in all fields');
      return;
    }

    try {
      setLoading(true);
      
      // Validate minimum amount
      const amountMicroStx = BigInt(stackAmount) * BigInt(1000000); // Convert to microSTX
      if (poxInfo && amountMicroStx < BigInt(poxInfo.min_amount_ustx)) {
        alert(`Minimum stacking amount is ${Number(poxInfo.min_amount_ustx) / 1000000} STX`);
        return;
      }

      // Simulate stacking transaction
      console.log('Stacking STX:', {
        amount: stackAmount,
        bitcoinzAddress,
        cycles: lockCycles,
      });

      alert(`Successfully submitted stacking transaction!\nAmount: ${stackAmount} STX\nBitcoinZ Address: ${bitcoinzAddress}\nLock Cycles: ${lockCycles}`);
      
      // Reset form
      setStackAmount('');
      setBitcoinzAddress('');
      setLockCycles(1);
      setPrivateKey('');
      
      // Reload stacker info
      await loadStackerInfo();
    } catch (error) {
      console.error('Stacking failed:', error);
      alert('Stacking failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleMiningBid = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!bidAmount || !miningPrivateKey) {
      alert('Please fill in all fields');
      return;
    }

    try {
      setLoading(true);
      
      // Convert to zatoshis
      const bidZatoshis = Math.floor(parseFloat(bidAmount) * 100000000);
      
      // Simulate mining bid
      console.log('Submitting mining bid:', {
        bidAmount: bidAmount + ' BTCZ',
        bidZatoshis,
      });

      alert(`Successfully submitted mining bid!\nAmount: ${bidAmount} BTCZ\nTarget: Next block`);
      
      // Reset form
      setBidAmount('');
      setMiningPrivateKey('');
      
      // Reload mining bids
      await loadMiningBids();
    } catch (error) {
      console.error('Mining bid failed:', error);
      alert('Mining bid failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const formatSTX = (microStx: string) => {
    return (Number(microStx) / 1000000).toLocaleString();
  };

  const formatBTCZ = (zatoshis: string) => {
    return (Number(zatoshis) / 100000000).toFixed(8);
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>🔥 BTCZS PoX Interface</h1>
        <p>BitcoinZ Layer 2 - Proof of Transfer</p>
      </header>

      <div className="container">
        {/* User Address Input */}
        <div className="address-section">
          <label>
            Your BTCZS Address:
            <input
              type="text"
              value={userAddress}
              onChange={(e) => setUserAddress(e.target.value)}
              placeholder="SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7"
              className="address-input"
            />
          </label>
        </div>

        {userAddress && (
          <>
            {/* PoX Info Display */}
            {poxInfo && (
            <div className="pox-info">
              <h3>📊 PoX System Status</h3>
              <div className="info-grid">
                <div>Reward Cycle: {poxInfo.reward_cycle_id}</div>
                <div>Next Cycle In: {poxInfo.next_reward_cycle_in} blocks</div>
                <div>Min Stacking: {formatSTX(poxInfo.min_amount_ustx)} STX</div>
                <div>Total Stacked: {formatSTX(poxInfo.total_stacked_ustx)} STX</div>
                <div>BTCZS Height: {poxInfo.current_btczs_block_height}</div>
                <div>BitcoinZ Height: {poxInfo.current_bitcoinz_block_height}</div>
              </div>
            </div>
          )}

          {/* Tab Navigation */}
          <div className="tabs">
            <button
              className={activeTab === 'stacking' ? 'tab active' : 'tab'}
              onClick={() => setActiveTab('stacking')}
            >
              🔒 STX Stacking
            </button>
            <button
              className={activeTab === 'mining' ? 'tab active' : 'tab'}
              onClick={() => setActiveTab('mining')}
            >
              ⛏️ BTCZ Mining
            </button>
            <button
              className={activeTab === 'rewards' ? 'tab active' : 'tab'}
              onClick={() => setActiveTab('rewards')}
            >
              💰 Rewards
            </button>
          </div>

          {/* Tab Content */}
          <div className="tab-content">
            {activeTab === 'stacking' && userAddress && (
                <div className="stacking-section">
                  <h3>🔒 Stack STX for BTCZ Rewards</h3>
                  
                  {stackerInfo && (
                    <div className="stacker-status">
                      <p><strong>Status:</strong> {stackerInfo.is_stacking ? '🟢 Stacking' : '🔴 Not Stacking'}</p>
                      {stackerInfo.is_stacking && (
                        <>
                          <p><strong>Stacked:</strong> {formatSTX(stackerInfo.stacked_amount || '0')} STX</p>
                          <p><strong>Unlock Height:</strong> {stackerInfo.unlock_height}</p>
                          <p><strong>BTCZ Rewards:</strong> {formatBTCZ(stackerInfo.total_btcz_rewards || '0')} BTCZ</p>
                        </>
                      )}
                    </div>
                  )}

                  <form onSubmit={handleStackStx} className="stacking-form">
                    <label>
                      STX Amount:
                      <input
                        type="number"
                        value={stackAmount}
                        onChange={(e) => setStackAmount(e.target.value)}
                        placeholder="100000"
                        min="100000"
                        step="1000"
                      />
                      <small>Minimum: 100,000 STX</small>
                    </label>

                    <label>
                      BitcoinZ Reward Address:
                      <input
                        type="text"
                        value={bitcoinzAddress}
                        onChange={(e) => setBitcoinzAddress(e.target.value)}
                        placeholder="t1YourBitcoinZAddress..."
                      />
                      <small>Where you'll receive BTCZ rewards</small>
                    </label>

                    <label>
                      Lock Cycles:
                      <select value={lockCycles} onChange={(e) => setLockCycles(Number(e.target.value))}>
                        {[1, 2, 3, 6, 12].map(cycles => (
                          <option key={cycles} value={cycles}>{cycles} cycle{cycles > 1 ? 's' : ''}</option>
                        ))}
                      </select>
                      <small>How long to lock your STX</small>
                    </label>

                    <label>
                      Private Key:
                      <input
                        type="password"
                        value={privateKey}
                        onChange={(e) => setPrivateKey(e.target.value)}
                        placeholder="Your STX private key"
                      />
                      <small>⚠️ Never share your private key</small>
                    </label>

                    <button type="submit" disabled={loading} className="submit-btn">
                      {loading ? 'Processing...' : 'Stack STX'}
                    </button>
                  </form>
                </div>
              )}

              {activeTab === 'mining' && userAddress && (
                <div className="mining-section">
                  <h3>⛏️ Bid BTCZ to Mine BTCZS</h3>
                  
                  <div className="mining-info">
                    <p><strong>How it works:</strong></p>
                    <ul>
                      <li>Bid BTCZ for the right to mine the next BTCZS block</li>
                      <li>Your BTCZ goes to STX stackers as rewards</li>
                      <li>If you win, you get 12,500 BTCZS + transaction fees</li>
                      <li>If you lose, you still pay the BTCZ (cost of bidding)</li>
                    </ul>
                  </div>

                  <form onSubmit={handleMiningBid} className="mining-form">
                    <label>
                      BTCZ Bid Amount:
                      <input
                        type="number"
                        value={bidAmount}
                        onChange={(e) => setBidAmount(e.target.value)}
                        placeholder="0.01"
                        min="0.001"
                        step="0.001"
                      />
                      <small>Minimum: 0.001 BTCZ</small>
                    </label>

                    <label>
                      Private Key:
                      <input
                        type="password"
                        value={miningPrivateKey}
                        onChange={(e) => setMiningPrivateKey(e.target.value)}
                        placeholder="Your BTCZ private key"
                      />
                      <small>⚠️ For signing BTCZ transaction</small>
                    </label>

                    <button type="submit" disabled={loading} className="submit-btn">
                      {loading ? 'Processing...' : 'Submit Mining Bid'}
                    </button>
                  </form>

                  {/* Current Mining Bids */}
                  <div className="current-bids">
                    <h4>Current Mining Bids</h4>
                    {miningBids.length > 0 ? (
                      <div className="bids-list">
                        {miningBids.map((bid, index) => (
                          <div key={index} className="bid-item">
                            <div>Bid: {formatBTCZ(bid.bid_amount)} BTCZ</div>
                            <div>Block: {bid.target_block}</div>
                            <div>Status: {bid.status}</div>
                          </div>
                        ))}
                      </div>
                    ) : (
                      <p>No current mining bids</p>
                    )}
                  </div>
                </div>
              )}

              {activeTab === 'rewards' && userAddress && (
                <div className="rewards-section">
                  <h3>💰 Your Rewards</h3>
                  
                  <div className="rewards-summary">
                    <div className="reward-card">
                      <h4>Total BTCZ Earned</h4>
                      <div className="reward-amount">
                        {stackerInfo ? formatBTCZ(stackerInfo.total_btcz_rewards || '0') : '0.00000000'} BTCZ
                      </div>
                    </div>
                  </div>

                  <div className="rewards-history">
                    <h4>Rewards History</h4>
                    <p>No rewards yet. Start stacking STX to earn BTCZ!</p>
                  </div>
                </div>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default App;
