# Solana Arbitrage Bot

A high-performance, real-time arbitrage bot for Solana that identifies and executes profitable price differences across multiple DEXs using  Jupiter Aggregator v6, gRPC, Jito bundles, and advanced MEV strategies.

<div align="center">

### 📞 Contact & Support

[![Telegram](https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)](https://t.me/roswelly)

**💬 Get in touch for support, questions, or collaboration**

</div>

## 🚀 Features

- **Multi-DEX Arbitrage**: Supports Raydium, Orca, Serum, Aldrin, Saber, and Mercurial
- **Jupiter Aggregator Integration**: Optimal routing and price discovery across 20+ DEXs
- **Real-time Price Monitoring**: gRPC streaming for instant price updates
- **Jito Bundle Integration**: Atomic execution with priority fees
- **Flash Loan Support**: Capital-efficient arbitrage opportunities
- **Risk Management**: Built-in stop-loss, take-profit, and position sizing
- **Performance Monitoring**: Prometheus metrics and real-time analytics
- **Configurable Strategies**: Customizable arbitrage parameters
- **Hybrid Execution**: Choose between Jupiter routing or direct DEX execution

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Price Feed    │    │  Arbitrage      │    │   Jito Bundle   │
│   (gRPC)        │───▶│   Engine        │───▶│   Submission    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   DEX APIs      │    │   Risk          │    │   Monitoring    │
│   (Multi-DEX)   │    │   Management    │    │   Service       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │
         ▼                       ▼
┌─────────────────┐    ┌─────────────────┐
│   Jupiter       │    │   Enhanced      │
│   Aggregator    │    │   Opportunities │
└─────────────────┘    └─────────────────┘
```

## 📦 Installation

### Prerequisites

- Rust 1.70+
- Solana CLI tools
- Jito bundle access
- RPC endpoints for multiple DEXs
- Jupiter API access (optional, can use public API)

### Build

```bash
cd solana-jupitere-arbitrage-bot
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
tip_account = "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5"
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

## 🚀 Usage

### Basic Usage

```bash
./target/release/solana-arbitrage-bot --config config.toml
```

### Advanced Usage

```bash
# Start with Jupiter integration
./target/release/solana-arbitrage-bot start --config config.toml --jito --grpc

# Test Jupiter integration
./target/release/solana-arbitrage-bot test-jupiter \
  --input-mint So11111111111111111111111111111111111111112 \
  --output-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 1000000

# Test Metis integration (v6 feature)
./target/release/solana-arbitrage-bot test-metis \
  --input-mint So11111111111111111111111111111111111111112 \
  --output-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 1000000

# Test Ultra API integration (v6 feature)
./target/release/solana-arbitrage-bot test-ultra \
  --input-mint So11111111111111111111111111111111111111112 \
  --output-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 1000000

# Check Jupiter API health
./target/release/solana-arbitrage-bot health

# Get Jupiter API information
./target/release/solana-arbitrage-bot info

# Scan for opportunities with enhanced Jupiter support
./target/release/solana-arbitrage-bot scan --min-profit 0.5 --max-amount 1000.0
```

## 📊 Monitoring

The bot provides comprehensive monitoring through:

- **Prometheus Metrics**: Real-time performance data
- **Structured Logging**: Detailed execution logs
- **Performance Analytics**: Profit/loss tracking
- **Health Checks**: System status monitoring

Access metrics at: `http://localhost:9090/metrics`

## 🌟 Jupiter v6 Integration Benefits

### Enhanced Price Discovery
- **20+ DEXs**: Access to Raydium, Orca, Serum, Aldrin, Saber, Mercurial, and more
- **Optimal Routing**: Automatic pathfinding across multiple DEXs for best prices
- **Real-time Quotes**: Instant price updates with minimal latency
- **Liquidity Aggregation**: Combined liquidity from all supported DEXs
- **Metis Optimization**: Advanced routing algorithms with convergence optimization
- **Cross-App State Sharing**: Synchronized state across multiple applications

### Improved Execution
- **Atomic Swaps**: Single transaction execution across multiple DEXs
- **Slippage Protection**: Built-in slippage controls and price impact monitoring
- **Gas Optimization**: Efficient transaction routing to minimize costs
- **MEV Protection**: Integration with Jito bundles for MEV-resistant execution
- **Ultra API**: Premium features with advanced routing and MEV protection
- **Dynamic Slippage**: Adaptive slippage based on market conditions

### API Gateway Options
- **Public API**: Free tier with basic features
- **Pro API**: Enhanced features for professional users
- **Lite API**: Lightweight version for simple integrations
- **Self-Hosted**: Full control with your own infrastructure
- **Ultra API**: Premium features with maximum performance

### Advanced Features
- **Integrator Fees**: Earn fees on successful trades
- **Yellowstone gRPC**: High-performance data streaming
- **Health Monitoring**: Real-time API status and performance metrics
- **Enhanced Error Reporting**: Detailed error information with debugging context
- **Rate Limit Management**: Intelligent rate limiting and retry logic

### Configuration Options
- **DEX Selection**: Choose preferred and excluded DEXs
- **Slippage Control**: Configurable slippage tolerance (default 0.5%)
- **Price Impact Limits**: Maximum price impact thresholds
- **Execution Methods**: Choose between Jupiter routing or direct DEX execution
- **API Type Selection**: Public, Pro, Lite, Self-Hosted, or Ultra
- **Feature Toggles**: Enable/disable Metis, Ultra, and health checks

## 🔧 API Reference

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

## 🛡️ Security

- **Private Key Protection**: Secure key handling and storage
- **Transaction Validation**: Comprehensive transaction verification
- **Rate Limiting**: Protection against API abuse
- **Error Handling**: Robust error recovery mechanisms

## 📈 Performance

- **Sub-second Execution**: Optimized for speed
- **Low Latency**: Direct RPC connections
- **High Throughput**: Concurrent opportunity processing
- **Resource Efficient**: Minimal CPU and memory usage

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For issues and questions:
- Open an issue on GitHub
- Check the documentation
- Review the examples

