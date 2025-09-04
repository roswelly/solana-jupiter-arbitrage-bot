use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Existing types from the original codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub dex_name: String,
    pub token_pair: String,
    pub base_token: String,
    pub quote_token: String,
    pub price: f64,
    pub volume_24h: f64,
    pub liquidity: f64,
    pub timestamp: i64,
    pub pool_address: String,
    pub price_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub id: String,
    pub token_pair: String,
    pub buy_dex: String,
    pub sell_dex: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub profit_percentage: f64,
    pub estimated_profit: f64,
    pub max_amount: f64,
    pub gas_cost: f64,
    pub timestamp: i64,
    pub buy_pool: String,
    pub sell_pool: String,
    pub slippage: f64,
    pub is_profitable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRequest {
    pub opportunity_id: String,
    pub amount: f64,
    pub private_key: String,
    pub max_slippage: f64,
    pub priority_fee: i32,
    pub use_jito: bool,
    pub jito_tip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResponse {
    pub transaction_id: String,
    pub success: bool,
    pub error_message: String,
    pub actual_profit: f64,
    pub gas_used: f64,
    pub execution_time: i64,
    pub bundle_id: String,
}

// New Jupiter-specific types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterQuote {
    pub input_mint: String,
    pub in_amount: u64,
    pub output_mint: String,
    pub out_amount: u64,
    pub price_impact_pct: f64,
    pub route_plan: Vec<RoutePlan>,
    pub context_slot: u64,
    pub time_taken: f64,
    pub slippage_bps: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapInfo {
    pub amm_key: String,
    pub label: String,
    pub input_mint: String,
    pub in_amount: String,
    pub output_mint: String,
    pub out_amount: String,
    pub fee_amount: String,
    pub fee_mint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterSwap {
    pub swap_transaction: String,
    pub last_valid_block_height: u64,
    pub prioritization_fee_lamports: u64,
    pub compute_unit_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRequest {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: u64,
    pub user_public_key: String,
    pub slippage: f64,
    pub priority_fee: u64,
    pub allowed_dexes: Option<Vec<String>>,
    pub excluded_dexes: Option<Vec<String>>,
    pub use_jupiter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResponse {
    pub transaction: String,
    pub success: bool,
    pub error_message: String,
    pub actual_profit: f64,
    pub gas_used: f64,
    pub execution_time: i64,
    pub bundle_id: String,
    pub quote: Option<JupiterQuote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterPriceData {
    pub id: String,
    pub mint_symbol: String,
    pub vs_token: String,
    pub vs_token_symbol: String,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterTokenInfo {
    pub address: String,
    pub chain_id: u16,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub logo_uri: Option<String>,
    pub tags: Vec<String>,
    pub extensions: Option<serde_json::Value>,
}

// Enhanced arbitrage opportunity with Jupiter integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedArbitrageOpportunity {
    pub id: String,
    pub token_pair: String,
    pub input_mint: String,
    pub output_mint: String,
    pub jupiter_quote: Option<JupiterQuote>,
    pub direct_dex_prices: Vec<DexPrice>,
    pub best_jupiter_price: f64,
    pub best_direct_price: f64,
    pub profit_percentage: f64,
    pub estimated_profit: f64,
    pub max_amount: f64,
    pub gas_cost: f64,
    pub timestamp: i64,
    pub slippage: f64,
    pub is_profitable: bool,
    pub execution_method: ExecutionMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexPrice {
    pub dex_name: String,
    pub price: f64,
    pub liquidity: f64,
    pub pool_address: String,
    pub price_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionMethod {
    Jupiter,
    DirectDex,
    Hybrid,
}

// Portfolio types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub wallet_address: String,
    pub balances: Vec<TokenBalance>,
    pub total_value_usd: f64,
    pub available_balance: f64,
    pub last_updated: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub token_mint: String,
    pub symbol: String,
    pub amount: f64,
    pub value_usd: f64,
    pub price: f64,
}

// Risk management types
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
    pub use_jupiter_for_execution: bool,
    pub jupiter_slippage_bps: u16,
    pub max_price_impact_pct: f64,
}

// Monitoring and statistics types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingStats {
    pub total_profit: f64,
    pub total_trades: u32,
    pub successful_trades: u32,
    pub win_rate: f64,
    pub avg_profit_per_trade: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub jupiter_trades: u32,
    pub direct_dex_trades: u32,
    pub hybrid_trades: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub execution_time_avg: f64,
    pub price_impact_avg: f64,
    pub slippage_avg: f64,
    pub gas_efficiency: f64,
    pub jupiter_success_rate: f64,
    pub direct_dex_success_rate: f64,
}

// Configuration types for Jupiter integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterConfig {
    pub enabled: bool,
    pub api_url: String,
    pub api_key: Option<String>,
    pub timeout_ms: u64,
    pub retry_attempts: u32,
    pub default_slippage_bps: u16,
    pub max_price_impact_pct: f64,
    pub preferred_dexes: Vec<String>,
    pub excluded_dexes: Vec<String>,
    pub use_shared_accounts: bool,
    pub dynamic_compute_unit_limit: bool,
    pub prioritization_fee_lamports: u64,
}

// Error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArbitrageError {
    JupiterApiError(String),
    DexApiError(String),
    InsufficientLiquidity,
    PriceImpactTooHigh,
    SlippageExceeded,
    GasPriceTooHigh,
    RiskCheckFailed,
    TransactionFailed(String),
    NetworkError(String),
}

impl std::fmt::Display for ArbitrageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArbitrageError::JupiterApiError(msg) => write!(f, "Jupiter API Error: {}", msg),
            ArbitrageError::DexApiError(msg) => write!(f, "DEX API Error: {}", msg),
            ArbitrageError::InsufficientLiquidity => write!(f, "Insufficient liquidity"),
            ArbitrageError::PriceImpactTooHigh => write!(f, "Price impact too high"),
            ArbitrageError::SlippageExceeded => write!(f, "Slippage exceeded"),
            ArbitrageError::GasPriceTooHigh => write!(f, "Gas price too high"),
            ArbitrageError::RiskCheckFailed => write!(f, "Risk check failed"),
            ArbitrageError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            ArbitrageError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for ArbitrageError {}
