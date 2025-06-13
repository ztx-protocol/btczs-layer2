/**
 * BTCZS Stacking Interface
 * Adapted from Stacks.js for BitcoinZ integration
 */

import { IntegerType } from '@stacks/common';

// BTCZS-specific network configuration
export interface BTCZSNetworkConfig {
  /** BTCZS node RPC URL */
  btczsRpcUrl: string;
  /** BitcoinZ node RPC URL */
  bitcoinzRpcUrl: string;
  /** BitcoinZ RPC credentials */
  bitcoinzRpcUser: string;
  bitcoinzRpcPass: string;
  /** Network type */
  network: 'mainnet' | 'testnet' | 'regtest';
}

// BTCZS PoX Information
export interface BTCZSPoxInfo {
  /** Current reward cycle ID */
  reward_cycle_id: number;
  /** Blocks until next cycle */
  next_reward_cycle_in: number;
  /** Minimum STX to stack (microSTX) */
  min_amount_ustx: string;
  /** Current cycle length in blocks */
  reward_cycle_length: number;
  /** Current BTCZS block height */
  current_btczs_block_height: number;
  /** Current BitcoinZ block height */
  current_bitcoinz_block_height: number;
  /** Total STX stacked in current cycle */
  total_stacked_ustx: string;
  /** Is PoX active */
  is_pox_active: boolean;
}

// STX Stacking Options for BTCZS
export interface BTCZSStackingOptions {
  /** STX amount to stack (in microSTX) */
  amountMicroStx: IntegerType;
  /** BitcoinZ address for BTCZ rewards */
  bitcoinzRewardAddress: string;
  /** Number of cycles to lock STX */
  cycles: number;
  /** Private key for signing transaction */
  privateKey: string;
  /** Optional: Start at specific burn height */
  burnBlockHeight?: number;
}

// Mining Bid Options for BTCZS
export interface BTCZSMiningOptions {
  /** BTCZ amount to bid (in zatoshis) */
  btczBidAmount: IntegerType;
  /** Private key for signing BTCZ transaction */
  privateKey: string;
  /** Target block height to mine */
  targetBlockHeight?: number;
}

// Stacker Information
export interface BTCZSStackerInfo {
  /** Is currently stacking */
  is_stacking: boolean;
  /** STX amount stacked */
  stacked_amount?: string;
  /** First reward cycle */
  first_reward_cycle?: number;
  /** Lock period in cycles */
  lock_period?: number;
  /** Unlock height */
  unlock_height?: number;
  /** BitcoinZ reward address */
  bitcoinz_reward_address?: string;
  /** Total BTCZ rewards earned */
  total_btcz_rewards?: string;
}

// Mining Bid Information
export interface BTCZSMiningBid {
  /** Bid amount in zatoshis */
  bid_amount: string;
  /** Miner address */
  miner_address: string;
  /** Target block height */
  target_block: number;
  /** Bid transaction ID */
  bid_txid: string;
  /** Bid status */
  status: 'pending' | 'confirmed' | 'won' | 'lost';
}

// Reward Information
export interface BTCZSRewardInfo {
  /** Reward cycle ID */
  cycle_id: number;
  /** BTCZ amount received */
  btcz_amount: string;
  /** BitcoinZ transaction ID */
  bitcoinz_txid: string;
  /** Block height when received */
  block_height: number;
  /** Timestamp */
  timestamp: number;
}

/**
 * BTCZS Stacking Client
 * Main interface for interacting with BTCZS PoX system
 */
export class BTCZSStackingClient {
  private config: BTCZSNetworkConfig;
  private userAddress: string;

  constructor(config: BTCZSNetworkConfig, userAddress: string) {
    this.config = config;
    this.userAddress = userAddress;
  }

  /**
   * Get current PoX information
   */
  async getPoxInfo(): Promise<BTCZSPoxInfo> {
    const response = await fetch(`${this.config.btczsRpcUrl}/v2/pox`);
    const data = await response.json();
    
    return {
      reward_cycle_id: data.reward_cycle_id,
      next_reward_cycle_in: data.next_reward_cycle_in,
      min_amount_ustx: data.min_amount_ustx,
      reward_cycle_length: data.reward_cycle_length,
      current_btczs_block_height: data.current_btczs_block_height,
      current_bitcoinz_block_height: data.current_bitcoinz_block_height,
      total_stacked_ustx: data.total_stacked_ustx,
      is_pox_active: data.is_pox_active,
    };
  }

  /**
   * Get stacker information for current user
   */
  async getStackerInfo(): Promise<BTCZSStackerInfo> {
    const response = await fetch(
      `${this.config.btczsRpcUrl}/v2/accounts/${this.userAddress}/stacking`
    );
    const data = await response.json();
    
    return {
      is_stacking: data.is_stacking,
      stacked_amount: data.stacked_amount,
      first_reward_cycle: data.first_reward_cycle,
      lock_period: data.lock_period,
      unlock_height: data.unlock_height,
      bitcoinz_reward_address: data.bitcoinz_reward_address,
      total_btcz_rewards: data.total_btcz_rewards,
    };
  }

  /**
   * Stack STX tokens for BTCZ rewards
   */
  async stackStx(options: BTCZSStackingOptions): Promise<string> {
    // Validate BitcoinZ address format
    if (!this.isValidBitcoinZAddress(options.bitcoinzRewardAddress)) {
      throw new Error('Invalid BitcoinZ address format');
    }

    // Validate minimum stacking amount
    const poxInfo = await this.getPoxInfo();
    if (BigInt(options.amountMicroStx.toString()) < BigInt(poxInfo.min_amount_ustx)) {
      throw new Error(`Minimum stacking amount is ${poxInfo.min_amount_ustx} microSTX`);
    }

    // Create stacking transaction
    const stackingTx = {
      contract_address: 'SP000000000000000000002Q6VF78',
      contract_name: 'pox-4',
      function_name: 'stack-stx',
      function_args: [
        { type: 'uint', value: options.amountMicroStx.toString() },
        { type: 'tuple', value: this.bitcoinzAddressToTuple(options.bitcoinzRewardAddress) },
        { type: 'uint', value: options.burnBlockHeight?.toString() || '0' },
        { type: 'uint', value: options.cycles.toString() },
      ],
      private_key: options.privateKey,
    };

    // Submit transaction to BTCZS network
    const response = await fetch(`${this.config.btczsRpcUrl}/v2/transactions`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(stackingTx),
    });

    const result = await response.json();
    return result.txid;
  }

  /**
   * Submit mining bid with BTCZ
   */
  async submitMiningBid(options: BTCZSMiningOptions): Promise<string> {
    // Create BTCZ transaction to stackers
    const stackerAddresses = await this.getStackerAddresses();
    
    if (stackerAddresses.length === 0) {
      throw new Error('No active stackers found');
    }

    // Calculate proportional distribution
    const distributions = await this.calculateBTCZDistribution(
      options.btczBidAmount,
      stackerAddresses
    );

    // Create BitcoinZ transaction
    const btczTx = {
      method: 'sendmany',
      params: [
        '', // from account
        distributions, // {address: amount} mapping
        1, // min confirmations
        'BTCZS Mining Bid', // comment
      ],
    };

    // Submit to BitcoinZ network
    const response = await this.bitcoinzRpc(btczTx.method, btczTx.params);
    
    // Register bid with BTCZS network
    await this.registerMiningBid({
      btcz_txid: response,
      bid_amount: options.btczBidAmount.toString(),
      miner_address: this.userAddress,
      target_block: options.targetBlockHeight || 0,
    });

    return response;
  }

  /**
   * Get stacking rewards history
   */
  async getRewardsHistory(): Promise<BTCZSRewardInfo[]> {
    const response = await fetch(
      `${this.config.btczsRpcUrl}/v2/accounts/${this.userAddress}/rewards`
    );
    const data = await response.json();
    
    return data.rewards.map((reward: any) => ({
      cycle_id: reward.cycle_id,
      btcz_amount: reward.btcz_amount,
      bitcoinz_txid: reward.bitcoinz_txid,
      block_height: reward.block_height,
      timestamp: reward.timestamp,
    }));
  }

  /**
   * Get current mining bids
   */
  async getCurrentMiningBids(): Promise<BTCZSMiningBid[]> {
    const response = await fetch(`${this.config.btczsRpcUrl}/v2/mining/bids`);
    const data = await response.json();
    
    return data.bids;
  }

  /**
   * Check if can stack STX
   */
  async canStackStx(amountMicroStx: IntegerType): Promise<{ eligible: boolean; reason?: string }> {
    const poxInfo = await this.getPoxInfo();
    const accountInfo = await this.getAccountInfo();

    if (BigInt(amountMicroStx.toString()) < BigInt(poxInfo.min_amount_ustx)) {
      return {
        eligible: false,
        reason: `Minimum stacking amount is ${poxInfo.min_amount_ustx} microSTX`,
      };
    }

    if (BigInt(accountInfo.balance) < BigInt(amountMicroStx.toString())) {
      return {
        eligible: false,
        reason: 'Insufficient STX balance',
      };
    }

    return { eligible: true };
  }

  // Private helper methods
  private async getAccountInfo(): Promise<{ balance: string; locked: string }> {
    const response = await fetch(`${this.config.btczsRpcUrl}/v2/accounts/${this.userAddress}`);
    return response.json();
  }

  private async getStackerAddresses(): Promise<string[]> {
    const response = await fetch(`${this.config.btczsRpcUrl}/v2/pox/stackers`);
    const data = await response.json();
    return data.stackers.map((s: any) => s.bitcoinz_address);
  }

  private async calculateBTCZDistribution(
    totalAmount: IntegerType,
    stackerAddresses: string[]
  ): Promise<Record<string, number>> {
    // Get stacking amounts for each address
    const stackingInfo = await Promise.all(
      stackerAddresses.map(async (addr) => {
        const response = await fetch(`${this.config.btczsRpcUrl}/v2/accounts/${addr}/stacking`);
        const data = await response.json();
        return { address: addr, amount: BigInt(data.stacked_amount || '0') };
      })
    );

    const totalStacked = stackingInfo.reduce((sum, info) => sum + info.amount, BigInt(0));
    const distributions: Record<string, number> = {};

    for (const info of stackingInfo) {
      if (info.amount > 0) {
        const proportion = Number(info.amount) / Number(totalStacked);
        const btczAmount = Math.floor(Number(totalAmount) * proportion);
        distributions[info.address] = btczAmount / 100000000; // Convert to BTCZ
      }
    }

    return distributions;
  }

  private async registerMiningBid(bid: {
    btcz_txid: string;
    bid_amount: string;
    miner_address: string;
    target_block: number;
  }): Promise<void> {
    await fetch(`${this.config.btczsRpcUrl}/v2/mining/register-bid`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(bid),
    });
  }

  private async bitcoinzRpc(method: string, params: any[]): Promise<any> {
    const response = await fetch(this.config.bitcoinzRpcUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Basic ${btoa(`${this.config.bitcoinzRpcUser}:${this.config.bitcoinzRpcPass}`)}`,
      },
      body: JSON.stringify({
        jsonrpc: '1.0',
        id: 'btczs',
        method,
        params,
      }),
    });

    const data = await response.json();
    if (data.error) {
      throw new Error(`BitcoinZ RPC error: ${data.error.message}`);
    }
    return data.result;
  }

  private isValidBitcoinZAddress(address: string): boolean {
    // BitcoinZ mainnet addresses start with 't1'
    return address.startsWith('t1') && address.length >= 34;
  }

  private bitcoinzAddressToTuple(address: string): any {
    // Convert BitcoinZ address to PoX tuple format
    // This is a simplified version - real implementation would decode the address
    return {
      version: { type: 'buffer', value: '0x00' },
      hashbytes: { type: 'buffer', value: `0x${address.slice(2)}` },
    };
  }
}
