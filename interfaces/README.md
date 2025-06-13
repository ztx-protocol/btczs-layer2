# BTCZS PoX Interfaces

User interfaces for the BTCZS Proof of Transfer system, adapted from Stacks for BitcoinZ integration.

## 🌐 Web Interface

Modern React-based web interface for easy PoX participation.

### Features:
- 🔒 **STX Stacking**: Lock STX tokens to earn BTCZ rewards
- ⛏️ **BTCZ Mining**: Bid BTCZ to mine BTCZS blocks and earn STX
- 💰 **Rewards Tracking**: Monitor your BTCZ earnings
- 📊 **PoX Dashboard**: Real-time system status

### Usage:
```bash
# Start web interface
./start_web_interface.sh

# Open browser to http://localhost:3000
```

## 💻 Command Line Interface

Python-based CLI for advanced users and automation.

### Features:
- Get PoX system information
- Check stacker status
- Submit stacking transactions
- Submit mining bids
- View rewards history

### Usage:
```bash
# Get PoX info
python3 btczs-cli.py pox-info

# Check stacker status
python3 btczs-cli.py stacker-info SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7

# Stack STX (example)
python3 btczs-cli.py stack-stx 100000 t1YourBitcoinZAddress 6 your-private-key

# Submit mining bid (example)
python3 btczs-cli.py mine-bid 0.01 your-btcz-private-key

# Check rewards
python3 btczs-cli.py rewards SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7
```

## 🔧 Configuration

Both interfaces connect to:
- **BTCZS Node**: http://localhost:20443 (default)
- **BitcoinZ Node**: http://localhost:1979 (default)

Update configuration in:
- Web: `src/App.tsx` - `BTCZS_CONFIG`
- CLI: Command line arguments `--btczs-rpc` and `--bitcoinz-rpc`

## 🎯 How BTCZS PoX Works

### For Stackers:
1. Lock STX tokens for 1-12 cycles
2. Provide BitcoinZ address for rewards
3. Receive BTCZ from miners proportionally
4. Earn actual BitcoinZ (not BTCZS)

### For Miners:
1. Bid BTCZ for the right to mine blocks
2. BTCZ goes to STX stackers as rewards
3. If you win, get 12,500 BTCZS + fees
4. If you lose, still pay BTCZ (cost of bidding)

### Economic Model:
- **Miners**: Pay BTCZ → Earn STX (bet on STX appreciation)
- **Stackers**: Lock STX → Earn BTCZ (guaranteed Bitcoin-family rewards)
- **Network**: Gets security from both groups

## 🚀 Getting Started

1. **Setup**: Run `./setup_interfaces.sh`
2. **Test CLI**: Run `./test_cli_interface.sh`
3. **Start Web**: Run `./start_web_interface.sh`
4. **Participate**: Stack STX or bid BTCZ!

## 🔗 Integration

These interfaces work with the BTCZS PoX system that we tested successfully:
- ✅ BTCZS node running (PID 45037)
- ✅ BitcoinZ connected (block 1,577,773)
- ✅ PoX mechanism functional
- ✅ Real transaction capability

Ready for live testing with BitcoinZ community! 🎯
