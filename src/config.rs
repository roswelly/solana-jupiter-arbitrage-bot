use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rpc_endpoints: RpcConfig,
    pub dex_endpoints: DexConfig,
    pub wallet: WalletConfig,
    pub jito: JitoConfig,
    pub risk_settings: RiskSettings,
    pub monitoring: MonitoringConfig,
    pub trading: TradingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcConfig {
    pub primary: String,
    pub secondary: Vec<String>,
    pub timeout_ms: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexConfig {
    pub raydium: DexEndpoint,
    pub orca: DexEndpoint,
    pub serum: DexEndpoint,
    pub aldrin: DexEndpoint,
    pub saber: DexEndpoint,
    pub mercurial: DexEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexEndpoint {
    pub name: String,
    pub rpc_url: String,
    pub api_url: Option<String>,
    pub enabled: bool,
    pub priority: u8,
    pub fee_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    pub private_key: String,
    pub public_key: String,
    pub max_sol_balance: f64,
    pub min_sol_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitoConfig {
    pub enabled: bool,
    pub tip_account: String,
    pub bundle_endpoint: String,
    pub max_tip_lamports: u64,
    pub min_tip_lamports: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSettings {
    pub max_position_size: f64,
    pub max_daily_loss: f64,
    pub max_slippage: f64,
    pub min_profit_threshold: f64,
    pub max_trades_per_hour: u32,
    pub enable_stop_loss: bool,
    pub stop_loss_percentage: f64,
    pub max_gas_price: u64,
    pub min_liquidity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub prometheus_port: u16,
    pub log_level: String,
    pub enable_metrics: bool,
    pub metrics_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingConfig {
    pub scan_interval_ms: u64,
    pub execution_timeout_ms: u64,
    pub max_concurrent_trades: u32,
    pub enable_auto_trading: bool,
    pub min_opportunity_duration_ms: u64,
    pub price_update_threshold: f64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn default() -> Self {
        Self {
            rpc_endpoints: RpcConfig {
                primary: "https://api.mainnet-beta.solana.com".to_string(),
                secondary: vec![
                    "https://solana-api.projectserum.com".to_string(),
                    "https://rpc.ankr.com/solana".to_string(),
                ],
                timeout_ms: 5000,
                retry_attempts: 3,
            },
            dex_endpoints: DexConfig {
                raydium: DexEndpoint {
                    name: "Raydium".to_string(),
                    rpc_url: "https://api.raydium.io/v2/sdk/liquidity/mainnet.json".to_string(),
                    api_url: Some("https://api.raydium.io".to_string()),
                    enabled: true,
                    priority: 1,
                    fee_percentage: 0.25,
                },
                orca: DexEndpoint {
                    name: "Orca".to_string(),
                    rpc_url: "https://api.mainnet.orca.so/v1/whirlpool/list".to_string(),
                    api_url: Some("https://api.mainnet.orca.so".to_string()),
                    enabled: true,
                    priority: 2,
                    fee_percentage: 0.3,
                },
                serum: DexEndpoint {
                    name: "Serum".to_string(),
                    rpc_url: "https://serum-api.bonfida.com/pools".to_string(),
                    api_url: Some("https://serum-api.bonfida.com".to_string()),
                    enabled: true,
                    priority: 3,
                    fee_percentage: 0.22,
                },
                aldrin: DexEndpoint {
                    name: "Aldrin".to_string(),
                    rpc_url: "https://api.aldrin.com/pools".to_string(),
                    api_url: Some("https://api.aldrin.com".to_string()),
                    enabled: false,
                    priority: 4,
                    fee_percentage: 0.3,
                },
                saber: DexEndpoint {
                    name: "Saber".to_string(),
                    rpc_url: "https://api.saber.so/pools".to_string(),
                    api_url: Some("https://api.saber.so".to_string()),
                    enabled: false,
                    priority: 5,
                    fee_percentage: 0.04,
                },
                mercurial: DexEndpoint {
                    name: "Mercurial".to_string(),
                    rpc_url: "https://api.mercurial.finance/pools".to_string(),
                    api_url: Some("https://api.mercurial.finance".to_string()),
                    enabled: false,
                    priority: 6,
                    fee_percentage: 0.01,
                },
            },
            wallet: WalletConfig {
                private_key: "".to_string(),
                public_key: "".to_string(),
                max_sol_balance: 10.0,
                min_sol_balance: 0.1,
            },
            jito: JitoConfig {
                enabled: true,
                tip_account: "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5".to_string(),
                bundle_endpoint: "https://mainnet.block-engine.jito.wtf".to_string(),
                max_tip_lamports: 1_000_000, // 0.001 SOL
                min_tip_lamports: 100_000,   // 0.0001 SOL
            },
            risk_settings: RiskSettings {
                max_position_size: 1000.0,
                max_daily_loss: 100.0,
                max_slippage: 1.0,
                min_profit_threshold: 0.5,
                max_trades_per_hour: 10,
                enable_stop_loss: true,
                stop_loss_percentage: 5.0,
                max_gas_price: 1_000_000,
                min_liquidity: 10_000.0,
            },
            monitoring: MonitoringConfig {
                prometheus_port: 9090,
                log_level: "info".to_string(),
                enable_metrics: true,
                metrics_interval_ms: 1000,
            },
            trading: TradingConfig {
                scan_interval_ms: 1000,
                execution_timeout_ms: 30000,
                max_concurrent_trades: 3,
                enable_auto_trading: false,
                min_opportunity_duration_ms: 500,
                price_update_threshold: 0.1,
            },
        }
    }
}
