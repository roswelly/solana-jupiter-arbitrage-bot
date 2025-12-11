# Solana Jupiter Arbitrage Bot

A high-performance, real-time arbitrage bot for Solana that identifies and executes profitable price differences across multiple DEXs using gRPC, Jito bundles, and advanced MEV strategies.

<div align="center">

### Call Me

[![Telegram](https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)](https://t.me/roswellecho)
[![Twitter](https://img.shields.io/badge/Twitter-14171A?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/roswellyecho)


</div>

## Features

- **Multi-DEX Arbitrage**: Supports Raydium, Orca, Serum, Aldrin, Saber, and Mercurial
- **Jupiter Aggregator Integration**: Optimal routing and price discovery across 20+ DEXs
- **Real-time Price Monitoring**: gRPC streaming for instant price updates
- **Jito Bundle Integration**: Atomic execution with priority fees
- **Flash Loan Support**: Capital-efficient arbitrage opportunities
- **Risk Management**: Built-in stop-loss, take-profit, and position sizing
- **Performance Monitoring**: Prometheus metrics and real-time analytics
- **Configurable Strategies**: Customizable arbitrage parameters
- **Hybrid Execution**: Choose between Jupiter routing or direct DEX execution

## 🏛️ Supported DEXs & Program IDs

This bot supports integration with major Solana DEXs. Here are the program IDs and addresses for each supported exchange:

### Primary DEXs

| DEX | Program ID | Description | Status |
|-----|------------|-------------|---------|
| **Raydium AMM V4** | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` | Automated Market Maker | ✅ Active |
| **Raydium AMM V3** | `5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h` | Legacy AMM | ✅ Active |
| **Orca Whirlpool** | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` | Concentrated Liquidity | ✅ Active |
| **Orca AMM V1** | `9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP` | Legacy AMM | ✅ Active |
| **Serum DEX V3** | `9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin` | Order Book DEX | ✅ Active |

### Secondary DEXs

| DEX | Program ID | Description | Status |
|-----|------------|-------------|---------|
| **Aldrin** | `AMM55ShdkoGRB5jVYPjWJkkeY4QwKDoBQfN9GpU8pY5` | AMM with concentrated liquidity | ✅ Active |
| **Saber** | `SSwpkEEqUyuG4Qb3n5B39u2KrY3mzd9qat9noGn5E88` | Stable swap AMM | ✅ Active |
| **Mercurial** | `MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky` | Stable swap AMM | ✅ Active |
| **Atrix** | `ATR1xDEX1D1D1D1D1D1D1D1D1D1D1D1D1D1D1D1D1D1` | AMM with concentrated liquidity | ✅ Active |
| **Crema Finance** | `6MLxLqiXaaSUpkgMnWDTuejNZEz3kE7k2woyHGVFw319` | Concentrated liquidity AMM | ✅ Active |
| **Lifinity** | `LiFiD1D1D1D1D1D1D1D1D1D1D1D1D1D1D1D1D1D1D1D1` | Concentrated liquidity AMM | ✅ Active |
| **Meteora** | `Eo7WjKq67rjJQS5xS6p3BywB4Y9EhmDbRcTnY6T6fP` | Dynamic AMM | ✅ Active |
| **OpenBook** | `srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX` | Order book DEX (Serum fork) | ✅ Active |

### Jupiter Aggregator

| Service | Program ID | Description |
|---------|------------|-------------|
| **Jupiter V6** | `JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB` | Main aggregator program |
| **Jupiter V4** | `JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB` | Legacy aggregator |

### Additional Program IDs

| Program | Address | Description |
|---------|---------|-------------|
| **Token Program** | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | SPL Token program |
| **Associated Token Program** | `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | Associated token accounts |
| **System Program** | `11111111111111111111111111111111` | Solana system program |
| **Rent Program** | `SysvarRent111111111111111111111111111111111` | Rent sysvar |
| **Clock Program** | `SysvarC1ock11111111111111111111111111111111` | Clock sysvar |

### Pool Addresses (Examples)

| Token Pair | Raydium Pool | Orca Pool | Serum Market |
|------------|--------------|-----------|--------------|
| **SOL/USDC** | `58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2` | `HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ` | `9wFFyRfZBsuAha4YcuxcXLKwMxJR43S7fPfQLusDBzvT` |
| **SOL/USDT** | `7qbRF6YsyGuLUVs6Y1q64bdVrfe4ZcUUz1JRdoVNUJnm` | `Dqk7mHQBx2ZWExmyrR2S8X6UG75CrbbpK2FSBZsNVswF` | `HWHvQhFmJBShNUZZxXb7LbQ3h6uH5bVf7Wq3o9y8Y6nK` |
| **USDC/USDT** | `77quYg4MGneUdjgXCunt9GgM1utm2ZJtkx7n7nrkpuNA` | `2QdhepnKRTLjjSqPL1PtKNwqrUkoLee5Gqs8bvZhRdMv` | `77quYg4MGneUdjgXCunt9GgM1utm2ZJtkx7n7nrkpuNA` |

### RPC Endpoints

| Provider | Mainnet RPC | Description |
|----------|-------------|-------------|
| **Solana Labs** | `https://api.mainnet-beta.solana.com` | Official Solana RPC |
| **Project Serum** | `https://solana-api.projectserum.com` | Serum RPC endpoint |
| **Ankr** | `https://rpc.ankr.com/solana` | Ankr RPC service |
| **QuickNode** | `https://solana-mainnet.quiknode.pro/` | QuickNode RPC |
| **Alchemy** | `https://solana-mainnet.g.alchemy.com/v2/` | Alchemy RPC |

### API Endpoints

| DEX | API Endpoint | Documentation |
|-----|--------------|---------------|
| **Raydium** | `https://api.raydium.io/v2/sdk/liquidity/mainnet.json` | [Raydium Docs](https://docs.raydium.io/) |
| **Orca** | `https://api.mainnet.orca.so/v1/whirlpool/list` | [Orca Docs](https://docs.orca.so/) |
| **Serum** | `https://serum-api.bonfida.com/pools` | [Serum Docs](https://docs.projectserum.com/) |
| **Jupiter** | `https://quote-api.jup.ag/v6` | [Jupiter Docs](https://docs.jup.ag/) |
| **Aldrin** | `https://api.aldrin.com/pools` | [Aldrin Docs](https://docs.aldrin.com/) |
| **Saber** | `https://api.saber.so/pools` | [Saber Docs](https://docs.saber.so/) |
| **Mercurial** | `https://api.mercurial.finance/pools` | [Mercurial Docs](https://docs.mercurial.finance/) |

> **Note**: Program IDs and addresses are subject to change. Always verify the latest addresses from official sources before using in production.

## Installation

### Prerequisites

- Rust 1.70+
- Solana CLI tools
- Jito bundle access
- RPC endpoints for multiple DEXs
- Jupiter API access (optional, can use public API)

### Build

```bash
cd solana-jupiter-arbitrage-bot
cargo build --release
```

## ⚙️ Configuration

Create a `config.toml` file:

```toml
[rpc_endpoints]
primary = "https://api.mainnet-beta.solana.com"
secondary = ["https://solana-api.projectserum.com", "https://rpc.ankr.com/solana"]
timeout_ms = 5000
retry_attempts = 3

[wallet]
private_key = ""  # Your wallet private key (Base58 format)
public_key = ""   # Your wallet public key
max_sol_balance = 10.0
min_sol_balance = 0.1

[jito]
enabled = true
tip_account = ""
bundle_endpoint = "https://mainnet.block-engine.jito.wtf"
max_tip_lamports = 1000000  # 0.001 SOL
min_tip_lamports = 100000   # 0.0001 SOL

[jupiter]
enabled = true
api_url = "https://quote-api.jup.ag/v6"
api_key = ""  # Optional: Your Jupiter API key
timeout_ms = 10000
retry_attempts = 3
default_slippage_bps = 50  # 0.5%
max_price_impact_pct = 5.0
preferred_dexes = ["Raydium", "Orca", "Serum"]
excluded_dexes = ["Aldrin", "Saber", "Mercurial"]
use_shared_accounts = true
dynamic_compute_unit_limit = true
prioritization_fee_lamports = 100000  # 0.0001 SOL

[risk_settings]
max_position_size = 1000.0
max_daily_loss = 100.0
max_slippage = 1.0
min_profit_threshold = 0.5
max_trades_per_hour = 10
enable_stop_loss = true
stop_loss_percentage = 5.0
max_gas_price = 1000000
min_liquidity = 10000.0
use_jupiter_for_execution = true
jupiter_slippage_bps = 50
max_price_impact_pct = 5.0

[monitoring]
prometheus_port = 9090
log_level = "info"
enable_metrics = true
metrics_interval_ms = 1000

[trading]
scan_interval_ms = 1000
execution_timeout_ms = 30000
max_concurrent_trades = 3
enable_auto_trading = false
min_opportunity_duration_ms = 500
price_update_threshold = 0.1
```

## Usage

### Basic Usage

```bash
./target/release/solana-jupiter-arbitrage-bot --config config.toml
```

### Advanced Usage

```bash
# Start with Jupiter integration
./target/release/solana-jupiter-arbitrage-bot start --config config.toml --jito --grpc

# Test Jupiter integration
./target/release/solana-jupiter-arbitrage-bot test-jupiter \
  --input-mint So11111111111111111111111111111111111111112 \
  --output-mint  \
  --amount 1000000

# Scan for opportunities with enhanced Jupiter support
./target/release/solana-arbitrage-bot scan --min-profit 0.5 --max-amount 1000.0
```

## 📊 Monitoring

The bot provides comprehensive monitoring through:

- **Prometheus Metrics**: Real-time performance data
- **Structured Logging**: Detailed execution logs
- **Performance Analytics**: Profit/loss tracking
- **Health Checks**: System status monitoring

Access metrics at: `http://localhost:9000/metrics`

## 🌟 Jupiter Integration Benefits

### Enhanced Price Discovery
- **20+ DEXs**: Access to Raydium, Orca, Serum, Aldrin, Saber, Mercurial, and more
- **Optimal Routing**: Automatic pathfinding across multiple DEXs for best prices
- **Real-time Quotes**: Instant price updates with minimal latency
- **Liquidity Aggregation**: Combined liquidity from all supported DEXs

### Improved Execution
- **Atomic Swaps**: Single transaction execution across multiple DEXs
- **Slippage Protection**: Built-in slippage controls and price impact monitoring
- **Gas Optimization**: Efficient transaction routing to minimize costs
- **MEV Protection**: Integration with Jito bundles for MEV-resistant execution

### Configuration Options
- **DEX Selection**: Choose preferred and excluded DEXs
- **Slippage Control**: Configurable slippage tolerance (default 0.5%)
- **Price Impact Limits**: Maximum price impact thresholds
- **Execution Methods**: Choose between Jupiter routing or direct DEX execution

## API Reference

### Core Functions

```rust
// Initialize arbitrage engine with Jupiter
let engine = ArbitrageEngine::new(
    config,
    dex_monitor,
    risk_manager,
    portfolio_manager,
    jito_client,
    jupiter_client,  // New Jupiter client
    monitoring,
).await?;

// Scan for enhanced opportunities
let opportunities = engine.scan_enhanced_opportunities(0.5, 1000.0).await?;

// Execute Jupiter swap
let result = engine.execute_jupiter_swap(&opportunity, amount).await?;
```

### Configuration Options

- `min_profit_threshold`: Minimum profit percentage to execute
- `max_slippage`: Maximum acceptable slippage
- `enabled_dexs`: List of DEXs to monitor
- `risk_management`: Risk control parameters

## Security

- **Private Key Protection**: Secure key handling and storage
- **Transaction Validation**: Comprehensive transaction verification
- **Rate Limiting**: Protection against API abuse
- **Error Handling**: Robust error recovery mechanisms

## Performance

- **Sub-second Execution**: Optimized for speed
- **Low Latency**: Direct RPC connections
- **High Throughput**: Concurrent opportunity processing
- **Resource Efficient**: Minimal CPU and memory usage

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues and questions:
- Open an issue on GitHub
- Check the documentation
- Review the examples

