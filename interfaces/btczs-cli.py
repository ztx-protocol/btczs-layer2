#!/usr/bin/env python3
"""
BTCZS Command Line Interface
Simple CLI for interacting with BTCZS PoX system
"""

import argparse
import json
import requests
import sys
from typing import Dict, Any, Optional

class BTCZSClient:
    def __init__(self, btczs_rpc_url: str = "http://localhost:20443", 
                 bitcoinz_rpc_url: str = "http://localhost:1979",
                 bitcoinz_user: str = "any", bitcoinz_pass: str = "any"):
        self.btczs_rpc_url = btczs_rpc_url
        self.bitcoinz_rpc_url = bitcoinz_rpc_url
        self.bitcoinz_user = bitcoinz_user
        self.bitcoinz_pass = bitcoinz_pass

    def btczs_rpc(self, method: str, params: list = None) -> Any:
        """Make RPC call to BTCZS node"""
        payload = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params or []
        }
        
        try:
            response = requests.post(self.btczs_rpc_url, json=payload, timeout=10)
            response.raise_for_status()
            result = response.json()
            
            if "error" in result:
                raise Exception(f"BTCZS RPC error: {result['error']}")
            
            return result.get("result")
        except requests.exceptions.RequestException as e:
            raise Exception(f"Failed to connect to BTCZS node: {e}")

    def bitcoinz_rpc(self, method: str, params: list = None) -> Any:
        """Make RPC call to BitcoinZ node"""
        payload = {
            "jsonrpc": "1.0",
            "id": "btczs-cli",
            "method": method,
            "params": params or []
        }
        
        try:
            response = requests.post(
                self.bitcoinz_rpc_url,
                json=payload,
                auth=(self.bitcoinz_user, self.bitcoinz_pass),
                timeout=10
            )
            response.raise_for_status()
            result = response.json()
            
            if "error" in result and result["error"] is not None:
                raise Exception(f"BitcoinZ RPC error: {result['error']}")
            
            return result.get("result")
        except requests.exceptions.RequestException as e:
            raise Exception(f"Failed to connect to BitcoinZ node: {e}")

    def get_pox_info(self) -> Dict[str, Any]:
        """Get PoX system information"""
        # Simulate PoX info (in real implementation, this would call BTCZS RPC)
        bitcoinz_height = self.bitcoinz_rpc("getblockcount")
        
        return {
            "reward_cycle_id": 42,
            "next_reward_cycle_in": 1500,
            "min_amount_ustx": "100000000000",  # 100,000 STX
            "reward_cycle_length": 2016,
            "current_btczs_block_height": 85000,
            "current_bitcoinz_block_height": bitcoinz_height,
            "total_stacked_ustx": "500000000000",  # 500,000 STX
            "is_pox_active": True,
        }

    def get_stacker_info(self, address: str) -> Dict[str, Any]:
        """Get stacker information for address"""
        # Simulate stacker info
        return {
            "is_stacking": False,
            "stacked_amount": "0",
            "first_reward_cycle": None,
            "lock_period": None,
            "unlock_height": None,
            "bitcoinz_reward_address": None,
            "total_btcz_rewards": "0",
        }

    def stack_stx(self, amount_stx: int, bitcoinz_address: str, cycles: int, private_key: str) -> str:
        """Stack STX tokens for BTCZ rewards"""
        print(f"🔒 Stacking {amount_stx:,} STX for {cycles} cycles")
        print(f"📍 BitcoinZ reward address: {bitcoinz_address}")
        
        # Validate inputs
        if amount_stx < 100000:
            raise ValueError("Minimum stacking amount is 100,000 STX")
        
        if not bitcoinz_address.startswith('t1'):
            raise ValueError("Invalid BitcoinZ address format")
        
        # Simulate stacking transaction
        txid = f"0x{'a' * 64}"  # Mock transaction ID
        print(f"✅ Stacking transaction submitted: {txid}")
        
        return txid

    def submit_mining_bid(self, btcz_amount: float, private_key: str) -> str:
        """Submit mining bid with BTCZ"""
        print(f"⛏️ Submitting mining bid: {btcz_amount} BTCZ")
        
        # Validate inputs
        if btcz_amount < 0.001:
            raise ValueError("Minimum bid amount is 0.001 BTCZ")
        
        # Get current stackers (simulate)
        stackers = [
            {"address": "t1TestStacker1", "staked_amount": "200000000000"},
            {"address": "t1TestStacker2", "staked_amount": "300000000000"},
        ]
        
        if not stackers:
            raise Exception("No active stackers found")
        
        # Calculate proportional distribution
        total_staked = sum(int(s["staked_amount"]) for s in stackers)
        distributions = {}
        
        for stacker in stackers:
            proportion = int(stacker["staked_amount"]) / total_staked
            btcz_reward = btcz_amount * proportion
            distributions[stacker["address"]] = round(btcz_reward, 8)
        
        print("💰 BTCZ distribution to stackers:")
        for address, amount in distributions.items():
            print(f"  {address}: {amount} BTCZ")
        
        # Simulate BitcoinZ transaction
        try:
            # In real implementation, this would create actual BitcoinZ transaction
            txid = f"{'b' * 64}"  # Mock BitcoinZ transaction ID
            print(f"✅ Mining bid submitted: {txid}")
            return txid
        except Exception as e:
            raise Exception(f"Failed to submit mining bid: {e}")

    def get_rewards_history(self, address: str) -> list:
        """Get rewards history for address"""
        # Simulate rewards history
        return [
            {
                "cycle_id": 41,
                "btcz_amount": "0.005",
                "bitcoinz_txid": "c" * 64,
                "block_height": 1577500,
                "timestamp": 1703000000,
            }
        ]

def format_stx(micro_stx: str) -> str:
    """Format microSTX to STX"""
    return f"{int(micro_stx) / 1_000_000:,.0f}"

def format_btcz(zatoshis: str) -> str:
    """Format zatoshis to BTCZ"""
    return f"{int(zatoshis) / 100_000_000:.8f}"

def main():
    parser = argparse.ArgumentParser(description="BTCZS PoX Command Line Interface")
    parser.add_argument("--btczs-rpc", default="http://localhost:20443", help="BTCZS RPC URL")
    parser.add_argument("--bitcoinz-rpc", default="http://localhost:1979", help="BitcoinZ RPC URL")
    parser.add_argument("--bitcoinz-user", default="any", help="BitcoinZ RPC username")
    parser.add_argument("--bitcoinz-pass", default="any", help="BitcoinZ RPC password")
    
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # PoX info command
    subparsers.add_parser("pox-info", help="Get PoX system information")
    
    # Stacker info command
    stacker_parser = subparsers.add_parser("stacker-info", help="Get stacker information")
    stacker_parser.add_argument("address", help="Stacker address")
    
    # Stack STX command
    stack_parser = subparsers.add_parser("stack-stx", help="Stack STX for BTCZ rewards")
    stack_parser.add_argument("amount", type=int, help="STX amount to stack")
    stack_parser.add_argument("bitcoinz-address", help="BitcoinZ address for rewards")
    stack_parser.add_argument("cycles", type=int, help="Number of cycles to lock")
    stack_parser.add_argument("private-key", help="Private key for signing")
    
    # Mining bid command
    mining_parser = subparsers.add_parser("mine-bid", help="Submit mining bid")
    mining_parser.add_argument("amount", type=float, help="BTCZ amount to bid")
    mining_parser.add_argument("private-key", help="Private key for signing")
    
    # Rewards command
    rewards_parser = subparsers.add_parser("rewards", help="Get rewards history")
    rewards_parser.add_argument("address", help="Address to check rewards for")
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    try:
        client = BTCZSClient(
            btczs_rpc_url=args.btczs_rpc,
            bitcoinz_rpc_url=args.bitcoinz_rpc,
            bitcoinz_user=args.bitcoinz_user,
            bitcoinz_pass=args.bitcoinz_pass
        )
        
        if args.command == "pox-info":
            info = client.get_pox_info()
            print("🔥 BTCZS PoX System Information")
            print("=" * 40)
            print(f"Reward Cycle: {info['reward_cycle_id']}")
            print(f"Next Cycle In: {info['next_reward_cycle_in']} blocks")
            print(f"Min Stacking: {format_stx(info['min_amount_ustx'])} STX")
            print(f"Total Stacked: {format_stx(info['total_stacked_ustx'])} STX")
            print(f"BTCZS Height: {info['current_btczs_block_height']}")
            print(f"BitcoinZ Height: {info['current_bitcoinz_block_height']}")
            print(f"PoX Active: {'✅ Yes' if info['is_pox_active'] else '❌ No'}")
            
        elif args.command == "stacker-info":
            info = client.get_stacker_info(args.address)
            print(f"🔒 Stacker Information: {args.address}")
            print("=" * 50)
            print(f"Status: {'🟢 Stacking' if info['is_stacking'] else '🔴 Not Stacking'}")
            if info['is_stacking']:
                print(f"Stacked: {format_stx(info['stacked_amount'])} STX")
                print(f"Unlock Height: {info['unlock_height']}")
                print(f"BTCZ Rewards: {format_btcz(info['total_btcz_rewards'])} BTCZ")
            
        elif args.command == "stack-stx":
            txid = client.stack_stx(
                amount_stx=args.amount,
                bitcoinz_address=getattr(args, 'bitcoinz-address'),
                cycles=args.cycles,
                private_key=getattr(args, 'private-key')
            )
            print(f"Transaction ID: {txid}")
            
        elif args.command == "mine-bid":
            txid = client.submit_mining_bid(
                btcz_amount=args.amount,
                private_key=getattr(args, 'private-key')
            )
            print(f"BitcoinZ Transaction ID: {txid}")
            
        elif args.command == "rewards":
            rewards = client.get_rewards_history(args.address)
            print(f"💰 Rewards History: {args.address}")
            print("=" * 50)
            if rewards:
                for reward in rewards:
                    print(f"Cycle {reward['cycle_id']}: {reward['btcz_amount']} BTCZ")
                    print(f"  TXID: {reward['bitcoinz_txid']}")
                    print(f"  Block: {reward['block_height']}")
                    print()
            else:
                print("No rewards found")
                
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
