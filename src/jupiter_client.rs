use crate::types::{JupiterQuote, JupiterSwap, SwapRequest, SwapResponse};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone)]
pub struct JupiterClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JupiterQuoteRequest {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: u64,
    pub slippage_bps: u16,
    pub swap_mode: Option<String>,
    pub dexes: Option<Vec<String>>,
    pub exclude_dexes: Option<Vec<String>>,
    pub platform_fee_bps: Option<u16>,
    pub max_accounts: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JupiterQuoteResponse {
    pub input_mint: String,
    pub in_amount: String,
    pub output_mint: String,
    pub out_amount: String,
    pub other_amount_threshold: String,
    pub swap_mode: String,
    pub slippage_bps: u16,
    pub platform_fee: Option<PlatformFee>,
    pub price_impact_pct: String,
    pub route_plan: Vec<RoutePlan>,
    pub context_slot: u64,
    pub time_taken: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformFee {
    pub amount: String,
    pub fee_bps: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: u8,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct JupiterSwapRequest {
    pub quote_response: JupiterQuoteResponse,
    pub user_public_key: String,
    pub dynamic_compute_unit_limit: Option<bool>,
    pub prioritization_fee_lamports: Option<u64>,
    pub as_legacy_transaction: Option<bool>,
    pub use_shared_accounts: Option<bool>,
    pub fee_account: Option<String>,
    pub tracking_account: Option<String>,
    pub compute_unit_price_micro_lamports: Option<u64>,
    pub as_versioned_transaction: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JupiterSwapResponse {
    pub swap_transaction: String,
    pub last_valid_block_height: u64,
    pub prioritization_fee_lamports: u64,
    pub compute_unit_limit: u32,
    pub prioritization_fee_lamports_per_cu: u64,
}

impl JupiterClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(key) = &api_key {
            headers.insert(
                "Authorization",
                format!("Bearer {}", key).parse().unwrap(),
            );
        }
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            api_key,
        }
    }

    pub async fn get_quote(&self, request: JupiterQuoteRequest) -> Result<JupiterQuote> {
        debug!("🔍 Getting Jupiter quote for {} -> {}", request.input_mint, request.output_mint);
        
        let url = format!("{}/quote", self.base_url);
        let response = self.client
            .get(&url)
            .query(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("❌ Jupiter quote request failed: {}", error_text);
            return Err(anyhow::anyhow!("Jupiter quote request failed: {}", error_text));
        }

        let quote_response: JupiterQuoteResponse = response.json().await?;
        
        let quote = JupiterQuote {
            input_mint: quote_response.input_mint,
            in_amount: quote_response.in_amount.parse()?,
            output_mint: quote_response.output_mint,
            out_amount: quote_response.out_amount.parse()?,
            price_impact_pct: quote_response.price_impact_pct.parse()?,
            route_plan: quote_response.route_plan,
            context_slot: quote_response.context_slot,
            time_taken: quote_response.time_taken,
            slippage_bps: quote_response.slippage_bps,
        };

        debug!("✅ Jupiter quote received: {} -> {} ({} tokens)", 
               quote.input_mint, quote.output_mint, quote.out_amount);
        
        Ok(quote)
    }

    pub async fn get_swap_transaction(&self, request: JupiterSwapRequest) -> Result<JupiterSwap> {
        debug!("🔄 Getting Jupiter swap transaction");
        
        let url = format!("{}/swap", self.base_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("❌ Jupiter swap request failed: {}", error_text);
            return Err(anyhow::anyhow!("Jupiter swap request failed: {}", error_text));
        }

        let swap_response: JupiterSwapResponse = response.json().await?;
        
        let swap = JupiterSwap {
            swap_transaction: swap_response.swap_transaction,
            last_valid_block_height: swap_response.last_valid_block_height,
            prioritization_fee_lamports: swap_response.prioritization_fee_lamports,
            compute_unit_limit: swap_response.compute_unit_limit,
        };

        debug!("✅ Jupiter swap transaction received");
        Ok(swap)
    }

    pub async fn get_tokens(&self) -> Result<HashMap<String, TokenInfo>> {
        debug!("🪙 Fetching Jupiter token list");
        
        let url = format!("{}/tokens", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("❌ Jupiter tokens request failed: {}", error_text);
            return Err(anyhow::anyhow!("Jupiter tokens request failed: {}", error_text));
        }

        let tokens: HashMap<String, TokenInfo> = response.json().await?;
        debug!("✅ Fetched {} tokens from Jupiter", tokens.len());
        Ok(tokens)
    }

    pub async fn get_price(&self, ids: &[String]) -> Result<HashMap<String, f64>> {
        debug!("💰 Getting Jupiter prices for {} tokens", ids.len());
        
        let url = format!("{}/price", self.base_url);
        let response = self.client
            .get(&url)
            .query(&[("ids", ids.join(","))])
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("❌ Jupiter price request failed: {}", error_text);
            return Err(anyhow::anyhow!("Jupiter price request failed: {}", error_text));
        }

        let prices: HashMap<String, PriceData> = response.json().await?;
        let price_map: HashMap<String, f64> = prices
            .into_iter()
            .map(|(k, v)| (k, v.price))
            .collect();

        debug!("✅ Fetched prices for {} tokens", price_map.len());
        Ok(price_map)
    }

    pub async fn execute_swap(&self, swap_request: SwapRequest) -> Result<SwapResponse> {
        info!("🚀 Executing Jupiter swap: {} -> {}", 
              swap_request.input_mint, swap_request.output_mint);

        // Get quote first
        let quote_request = JupiterQuoteRequest {
            input_mint: swap_request.input_mint.clone(),
            output_mint: swap_request.output_mint.clone(),
            amount: swap_request.amount,
            slippage_bps: (swap_request.slippage * 100.0) as u16,
            swap_mode: Some("ExactIn".to_string()),
            dexes: swap_request.allowed_dexes,
            exclude_dexes: swap_request.excluded_dexes,
            platform_fee_bps: None,
            max_accounts: Some(64),
        };

        let quote = self.get_quote(quote_request).await?;

        // Create swap transaction
        let swap_request_jupiter = JupiterSwapRequest {
            quote_response: JupiterQuoteResponse {
                input_mint: quote.input_mint.clone(),
                in_amount: quote.in_amount.to_string(),
                output_mint: quote.output_mint.clone(),
                out_amount: quote.out_amount.to_string(),
                other_amount_threshold: "0".to_string(),
                swap_mode: "ExactIn".to_string(),
                slippage_bps: quote.slippage_bps,
                platform_fee: None,
                price_impact_pct: quote.price_impact_pct.to_string(),
                route_plan: quote.route_plan.clone(),
                context_slot: quote.context_slot,
                time_taken: quote.time_taken,
            },
            user_public_key: swap_request.user_public_key,
            dynamic_compute_unit_limit: Some(true),
            prioritization_fee_lamports: Some(swap_request.priority_fee),
            as_legacy_transaction: Some(false),
            use_shared_accounts: Some(true),
            fee_account: None,
            tracking_account: None,
            compute_unit_price_micro_lamports: None,
            as_versioned_transaction: Some(true),
        };

        let swap = self.get_swap_transaction(swap_request_jupiter).await?;

        Ok(SwapResponse {
            transaction: swap.swap_transaction,
            success: true,
            error_message: String::new(),
            actual_profit: 0.0, // Will be calculated after execution
            gas_used: swap.prioritization_fee_lamports as f64 / 1_000_000_000.0, // Convert lamports to SOL
            execution_time: 0,
            bundle_id: String::new(),
            quote: Some(quote),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub chain_id: u16,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub logo_uri: Option<String>,
    pub tags: Vec<String>,
    pub extensions: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceData {
    pub id: String,
    pub mint_symbol: String,
    pub vs_token: String,
    pub vs_token_symbol: String,
    pub price: f64,
}
