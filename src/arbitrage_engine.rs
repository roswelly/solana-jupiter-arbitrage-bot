use crate::{
    config::Config,
    dex_monitor::DexMonitor,
    risk_manager::RiskManager,
    portfolio_manager::PortfolioManager,
    jito_client::JitoClient,
    jupiter_client::JupiterClient,
    monitoring::MonitoringService,
    types::{
        ArbitrageOpportunity, PriceData, TradeRequest, TradeResponse,
        EnhancedArbitrageOpportunity, JupiterQuote, SwapRequest, SwapResponse,
        ExecutionMethod, DexPrice, ArbitrageError
    },
};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use chrono::Utc;

pub struct ArbitrageEngine {
    config: Config,
    dex_monitor: Arc<DexMonitor>,
    risk_manager: Arc<RwLock<RiskManager>>,
    portfolio_manager: Arc<PortfolioManager>,
    jito_client: Option<Arc<JitoClient>>,
    jupiter_client: Option<Arc<JupiterClient>>,
    monitoring: Arc<MonitoringService>,
    is_running: Arc<RwLock<bool>>,
}

impl ArbitrageEngine {
    pub fn new(
        config: Config,
        dex_monitor: Arc<DexMonitor>,
        risk_manager: Arc<RwLock<RiskManager>>,
        portfolio_manager: Arc<PortfolioManager>,
        jito_client: Option<Arc<JitoClient>>,
        jupiter_client: Option<Arc<JupiterClient>>,
        monitoring: Arc<MonitoringService>,
    ) -> Self {
        Self {
            config,
            dex_monitor,
            risk_manager,
            portfolio_manager,
            jito_client,
            jupiter_client,
            monitoring,
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let mut running = self.is_running.write().await;
        *running = true;
        drop(running);

        info!("🚀 Starting arbitrage engine");
        
        // Start the main arbitrage loop
        let engine_clone = self.clone_for_task();
        tokio::spawn(async move {
            if let Err(e) = engine_clone.arbitrage_loop().await {
                error!("❌ Arbitrage loop error: {}", e);
            }
        });

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut running = self.is_running.write().await;
        *running = false;
        info!("🛑 Stopping arbitrage engine");
        Ok(())
    }

    pub async fn scan_enhanced_opportunities(
        &self,
        min_profit_percentage: f64,
        max_amount: f64,
    ) -> Result<Vec<EnhancedArbitrageOpportunity>> {
        debug!("🔍 Scanning for enhanced arbitrage opportunities with Jupiter");
        
        let mut opportunities = Vec::new();
        
        // Get direct DEX prices
        let dex_prices = self.dex_monitor.get_all_prices().await?;
        
        // Group prices by token pair
        let mut price_groups: std::collections::HashMap<String, Vec<PriceData>> = 
            std::collections::HashMap::new();
        
        for price in dex_prices {
            price_groups.entry(price.token_pair.clone()).or_default().push(price);
        }

        // Process each token pair
        for (token_pair, prices) in price_groups {
            if prices.len() < 2 {
                continue;
            }

            // Extract token mints (simplified - in real implementation, you'd have a mapping)
            let (input_mint, output_mint) = self.extract_token_mints(&token_pair)?;
            
            // Get Jupiter quote if enabled
            let jupiter_quote = if self.config.jupiter.enabled && self.jupiter_client.is_some() {
                match self.get_jupiter_quote(&input_mint, &output_mint, max_amount as u64).await {
                    Ok(quote) => Some(quote),
                    Err(e) => {
                        warn!("⚠️ Failed to get Jupiter quote for {}: {}", token_pair, e);
                        None
                    }
                }
            } else {
                None
            };

            // Convert DEX prices to DexPrice format
            let direct_dex_prices: Vec<DexPrice> = prices.iter().map(|p| DexPrice {
                dex_name: p.dex_name.clone(),
                price: p.price,
                liquidity: p.liquidity,
                pool_address: p.pool_address.clone(),
                price_impact: p.price_impact,
            }).collect();

            // Find best prices
            let best_jupiter_price = jupiter_quote.as_ref()
                .map(|q| (q.out_amount as f64) / (q.in_amount as f64))
                .unwrap_or(0.0);
            
            let best_direct_price = direct_dex_prices.iter()
                .map(|p| p.price)
                .fold(0.0, f64::max);

            // Calculate profit opportunities
            if best_jupiter_price > 0.0 && best_direct_price > 0.0 {
                let profit_percentage = ((best_jupiter_price - best_direct_price) / best_direct_price) * 100.0;
                
                if profit_percentage >= min_profit_percentage {
                    let estimated_profit = (best_jupiter_price - best_direct_price) * max_amount;
                    let gas_cost = self.estimate_gas_cost().await?;
                    
                    if estimated_profit > gas_cost {
                        let execution_method = if jupiter_quote.is_some() {
                            ExecutionMethod::Jupiter
                        } else {
                            ExecutionMethod::DirectDex
                        };

                        let opportunity = EnhancedArbitrageOpportunity {
                            id: Uuid::new_v4().to_string(),
                            token_pair: token_pair.clone(),
                            input_mint,
                            output_mint,
                            jupiter_quote,
                            direct_dex_prices,
                            best_jupiter_price,
                            best_direct_price,
                            profit_percentage,
                            estimated_profit: estimated_profit - gas_cost,
                            max_amount,
                            gas_cost,
                            timestamp: Utc::now().timestamp_millis(),
                            slippage: self.config.jupiter.default_slippage_bps as f64 / 100.0,
                            is_profitable: true,
                            execution_method,
                        };

                        opportunities.push(opportunity);
                    }
                }
            }
        }

        // Sort by profit percentage
        opportunities.sort_by(|a, b| b.profit_percentage.partial_cmp(&a.profit_percentage).unwrap());

        info!("✅ Found {} enhanced arbitrage opportunities", opportunities.len());
        Ok(opportunities)
    }

    pub async fn scan_opportunities(
        &self,
        min_profit_percentage: f64,
        max_amount: f64,
    ) -> Result<Vec<ArbitrageOpportunity>> {
        debug!("🔍 Scanning for arbitrage opportunities");
        
        let prices = self.dex_monitor.get_all_prices().await?;
        let mut opportunities = Vec::new();

        // Group prices by token pair
        let mut price_groups: std::collections::HashMap<String, Vec<PriceData>> = 
            std::collections::HashMap::new();
        
        for price in prices {
            price_groups.entry(price.token_pair.clone()).or_default().push(price);
        }

        // Find arbitrage opportunities
        for (token_pair, prices) in price_groups {
            if prices.len() < 2 {
                continue;
            }

            // Sort by price to find best buy/sell opportunities
            let mut sorted_prices = prices.clone();
            sorted_prices.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

            let lowest_price = &sorted_prices[0];
            let highest_price = &sorted_prices[sorted_prices.len() - 1];

            let profit_percentage = ((highest_price.price - lowest_price.price) / lowest_price.price) * 100.0;
            
            if profit_percentage >= min_profit_percentage {
                let estimated_profit = (highest_price.price - lowest_price.price) * max_amount;
                let gas_cost = self.estimate_gas_cost().await?;
                
                if estimated_profit > gas_cost {
                    let opportunity = ArbitrageOpportunity {
                        id: Uuid::new_v4().to_string(),
                        token_pair: token_pair.clone(),
                        buy_dex: lowest_price.dex_name.clone(),
                        sell_dex: highest_price.dex_name.clone(),
                        buy_price: lowest_price.price,
                        sell_price: highest_price.price,
                        profit_percentage,
                        estimated_profit: estimated_profit - gas_cost,
                        max_amount,
                        gas_cost,
                        timestamp: Utc::now().timestamp_millis(),
                        buy_pool: lowest_price.pool_address.clone(),
                        sell_pool: highest_price.pool_address.clone(),
                        slippage: 0.5, // Default slippage
                        is_profitable: true,
                    };

                    opportunities.push(opportunity);
                }
            }
        }

        // Sort by profit percentage
        opportunities.sort_by(|a, b| b.profit_percentage.partial_cmp(&a.profit_percentage).unwrap());

        info!("✅ Found {} arbitrage opportunities", opportunities.len());
        Ok(opportunities)
    }

    pub async fn execute_trade(&self, request: TradeRequest) -> Result<TradeResponse> {
        info!("💼 Executing trade for opportunity: {}", request.opportunity_id);
        
        // Risk check
        let risk_manager = self.risk_manager.read().await;
        if !risk_manager.can_execute_trade(&request).await? {
            return Ok(TradeResponse {
                transaction_id: "".to_string(),
                success: false,
                error_message: "Risk check failed".to_string(),
                actual_profit: 0.0,
                gas_used: 0.0,
                execution_time: 0,
                bundle_id: "".to_string(),
            });
        }
        drop(risk_manager);

        let start_time = std::time::Instant::now();

        // Get opportunity details (in real implementation, this would be from a database)
        let opportunity = self.get_opportunity_by_id(&request.opportunity_id).await?;
        
        // Build and execute transaction
        let transaction_result = if request.use_jito && self.jito_client.is_some() {
            self.execute_jito_trade(&request, &opportunity).await?
        } else {
            self.execute_regular_trade(&request, &opportunity).await?
        };

        let execution_time = start_time.elapsed().as_millis() as i64;

        // Update monitoring metrics
        self.monitoring.record_trade_execution(
            transaction_result.success,
            transaction_result.actual_profit,
            execution_time,
        ).await;

        Ok(transaction_result)
    }

    async fn arbitrage_loop(&self) -> Result<()> {
        let mut interval = tokio::time::interval(
            std::time::Duration::from_millis(self.config.trading.scan_interval_ms)
        );

        loop {
            interval.tick().await;
            
            let running = *self.is_running.read().await;
            if !running {
                break;
            }

            // Scan for opportunities
            let opportunities = self.scan_opportunities(
                self.config.risk_settings.min_profit_threshold,
                self.config.risk_settings.max_position_size,
            ).await?;

            // Execute profitable trades if auto-trading is enabled
            if self.config.trading.enable_auto_trading {
                for opportunity in opportunities {
                    if opportunity.is_profitable {
                        let trade_request = TradeRequest {
                            opportunity_id: opportunity.id.clone(),
                            amount: opportunity.max_amount,
                            private_key: self.config.wallet.private_key.clone(),
                            max_slippage: self.config.risk_settings.max_slippage,
                            priority_fee: 1000, // Default priority fee
                            use_jito: self.jito_client.is_some(),
                            jito_tip: "100000".to_string(), // 0.0001 SOL
                        };

                        match self.execute_trade(trade_request).await {
                            Ok(response) => {
                                if response.success {
                                    info!("✅ Trade executed successfully: {}", response.transaction_id);
                                } else {
                                    warn!("❌ Trade failed: {}", response.error_message);
                                }
                            }
                            Err(e) => {
                                error!("❌ Trade execution error: {}", e);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn estimate_gas_cost(&self) -> Result<f64> {
        // Estimate gas cost based on current network conditions
        // This is a simplified estimation
        Ok(0.005) // $0.005 estimated gas cost
    }

    async fn get_opportunity_by_id(&self, id: &str) -> Result<ArbitrageOpportunity> {
        // In a real implementation, this would fetch from a database
        // For now, return a mock opportunity
        Ok(ArbitrageOpportunity {
            id: id.to_string(),
            token_pair: "SOL/USDC".to_string(),
            buy_dex: "Raydium".to_string(),
            sell_dex: "Orca".to_string(),
            buy_price: 100.0,
            sell_price: 101.0,
            profit_percentage: 1.0,
            estimated_profit: 10.0,
            max_amount: 1000.0,
            gas_cost: 0.005,
            timestamp: Utc::now().timestamp_millis(),
            buy_pool: "pool_address_1".to_string(),
            sell_pool: "pool_address_2".to_string(),
            slippage: 0.5,
            is_profitable: true,
        })
    }

    async fn execute_jito_trade(
        &self,
        request: &TradeRequest,
        opportunity: &ArbitrageOpportunity,
    ) -> Result<TradeResponse> {
        if let Some(jito_client) = &self.jito_client {
            // Build Jito bundle transaction
            let bundle_id = jito_client.submit_bundle(&request, opportunity).await?;
            
            Ok(TradeResponse {
                transaction_id: format!("jito_{}", bundle_id),
                success: true,
                error_message: "".to_string(),
                actual_profit: opportunity.estimated_profit,
                gas_used: opportunity.gas_cost,
                execution_time: 0,
                bundle_id,
            })
        } else {
            Err(anyhow::anyhow!("Jito client not available"))
        }
    }

    async fn execute_regular_trade(
        &self,
        request: &TradeRequest,
        opportunity: &ArbitrageOpportunity,
    ) -> Result<TradeResponse> {
        // Build and execute regular Solana transaction
        // This is a simplified implementation
        let transaction_id = format!("tx_{}", Uuid::new_v4());
        
        Ok(TradeResponse {
            transaction_id,
            success: true,
            error_message: "".to_string(),
            actual_profit: opportunity.estimated_profit,
            gas_used: opportunity.gas_cost,
            execution_time: 0,
            bundle_id: "".to_string(),
        })
    }

    async fn get_jupiter_quote(
        &self,
        input_mint: &str,
        output_mint: &str,
        amount: u64,
    ) -> Result<JupiterQuote> {
        if let Some(jupiter_client) = &self.jupiter_client {
            use crate::jupiter_client::JupiterQuoteRequest;
            
            let request = JupiterQuoteRequest {
                input_mint: input_mint.to_string(),
                output_mint: output_mint.to_string(),
                amount,
                slippage_bps: self.config.jupiter.default_slippage_bps,
                swap_mode: Some("ExactIn".to_string()),
                dexes: Some(self.config.jupiter.preferred_dexes.clone()),
                exclude_dexes: Some(self.config.jupiter.excluded_dexes.clone()),
                platform_fee_bps: None,
                max_accounts: Some(64),
            };

            jupiter_client.get_quote(request).await
        } else {
            Err(anyhow::anyhow!("Jupiter client not available"))
        }
    }

    fn extract_token_mints(&self, token_pair: &str) -> Result<(String, String)> {
        // Simplified token mint extraction
        // In a real implementation, you'd have a mapping from token pairs to mint addresses
        let parts: Vec<&str> = token_pair.split('/').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid token pair format: {}", token_pair));
        }

        // This is a simplified mapping - in reality, you'd have a proper token registry
        let input_mint = match parts[0] {
            "SOL" => "So11111111111111111111111111111111111111112".to_string(),
            "USDC" => "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
            "USDT" => "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB".to_string(),
            _ => return Err(anyhow::anyhow!("Unknown token: {}", parts[0])),
        };

        let output_mint = match parts[1] {
            "SOL" => "So11111111111111111111111111111111111111112".to_string(),
            "USDC" => "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
            "USDT" => "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB".to_string(),
            _ => return Err(anyhow::anyhow!("Unknown token: {}", parts[1])),
        };

        Ok((input_mint, output_mint))
    }

    async fn execute_jupiter_swap(
        &self,
        opportunity: &EnhancedArbitrageOpportunity,
        amount: u64,
    ) -> Result<SwapResponse> {
        if let Some(jupiter_client) = &self.jupiter_client {
            let swap_request = SwapRequest {
                input_mint: opportunity.input_mint.clone(),
                output_mint: opportunity.output_mint.clone(),
                amount,
                user_public_key: self.config.wallet.public_key.clone(),
                slippage: self.config.jupiter.default_slippage_bps as f64 / 100.0,
                priority_fee: self.config.jupiter.prioritization_fee_lamports,
                allowed_dexes: Some(self.config.jupiter.preferred_dexes.clone()),
                excluded_dexes: Some(self.config.jupiter.excluded_dexes.clone()),
                use_jupiter: true,
            };

            jupiter_client.execute_swap(swap_request).await
        } else {
            Err(anyhow::anyhow!("Jupiter client not available"))
        }
    }

    fn clone_for_task(&self) -> Self {
        Self {
            config: self.config.clone(),
            dex_monitor: self.dex_monitor.clone(),
            risk_manager: self.risk_manager.clone(),
            portfolio_manager: self.portfolio_manager.clone(),
            jito_client: self.jito_client.clone(),
            jupiter_client: self.jupiter_client.clone(),
            monitoring: self.monitoring.clone(),
            is_running: self.is_running.clone(),
        }
    }
}
