use clap::{Parser, Subcommand};
use solana_arbitrage_bot::{
    config::Config,
    arbitrage_engine::ArbitrageEngine,
    dex_monitor::DexMonitor,
    grpc_server::ArbitrageGrpcServer,
    jito_client::JitoClient,
    jupiter_client::JupiterClient,
    risk_manager::RiskManager,
    portfolio_manager::PortfolioManager,
    monitoring::MonitoringService,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

#[derive(Parser)]
#[command(name = "solana-arbitrage-bot")]
#[command(about = "Advanced Solana Arbitrage Bot with gRPC and Jito integration")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the arbitrage bot
    Start {
        /// Enable gRPC server
        #[arg(long)]
        grpc: bool,
        
        /// gRPC server port
        #[arg(long, default_value = "50051")]
        grpc_port: u16,
        
        /// Enable Jito bundle submission
        #[arg(long)]
        jito: bool,
    },
    /// Run a single arbitrage scan
    Scan {
        /// Minimum profit percentage
        #[arg(long, default_value = "0.5")]
        min_profit: f64,
        
        /// Maximum amount to trade
        #[arg(long, default_value = "1000.0")]
        max_amount: f64,
    },
    /// Get current portfolio
    Portfolio,
    /// Update risk settings
    Risk {
        /// Maximum position size
        #[arg(long)]
        max_position: Option<f64>,
        
        /// Maximum daily loss
        #[arg(long)]
        max_daily_loss: Option<f64>,
        
        /// Maximum slippage
        #[arg(long)]
        max_slippage: Option<f64>,
    },
    /// Test Jupiter integration
    TestJupiter {
        /// Input token mint
        #[arg(long)]
        input_mint: String,
        
        /// Output token mint
        #[arg(long)]
        output_mint: String,
        
        /// Amount to swap
        #[arg(long, default_value = "1000000")]
        amount: u64,
    },
    /// Test Metis integration
    TestMetis {
        /// Input token mint
        #[arg(long)]
        input_mint: String,
        
        /// Output token mint
        #[arg(long)]
        output_mint: String,
        
        /// Amount to swap
        #[arg(long, default_value = "1000000")]
        amount: u64,
    },
    /// Test Ultra API integration
    TestUltra {
        /// Input token mint
        #[arg(long)]
        input_mint: String,
        
        /// Output token mint
        #[arg(long)]
        output_mint: String,
        
        /// Amount to swap
        #[arg(long, default_value = "1000000")]
        amount: u64,
    },
    /// Check Jupiter API health
    Health,
    /// Get Jupiter API information
    Info,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.debug { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("solana_arbitrage_bot={}", log_level))
        .init();
    
    info!("🚀 Starting Solana Arbitrage Bot v{}", env!("CARGO_PKG_VERSION"));
    
    // Load configuration
    let config = Config::load(&cli.config)?;
    info!("📋 Configuration loaded from {}", cli.config);
    
    // Initialize services
    let monitoring = Arc::new(MonitoringService::new());
    let risk_manager = Arc::new(RwLock::new(RiskManager::new(config.risk_settings.clone())));
    let portfolio_manager = Arc::new(PortfolioManager::new(config.clone()));
    let jito_client = if cli.command.is_jito_enabled() {
        Some(Arc::new(JitoClient::new(config.jito.clone())))
    } else {
        None
    };

    let jupiter_client = if config.jupiter.enabled {
        Some(Arc::new(JupiterClient::new(
            config.jupiter.api_url.clone(),
            config.jupiter.api_key.clone(),
        )))
    } else {
        None
    };
    
    let dex_monitor = Arc::new(DexMonitor::new(config.dex_endpoints.clone()));
    let arbitrage_engine = Arc::new(ArbitrageEngine::new(
        config.clone(),
        dex_monitor.clone(),
        risk_manager.clone(),
        portfolio_manager.clone(),
        jito_client.clone(),
        jupiter_client.clone(),
        monitoring.clone(),
    ));
    
    match cli.command {
        Commands::Start { grpc, grpc_port, jito } => {
            info!("🎯 Starting arbitrage bot with gRPC: {}, Jito: {}", grpc, jito);
            
            // Start monitoring
            monitoring.start().await?;
            
            // Start DEX monitoring
            dex_monitor.start().await?;
            
            // Start arbitrage engine
            arbitrage_engine.start().await?;
            
            if grpc {
                let grpc_server = ArbitrageGrpcServer::new(
                    arbitrage_engine.clone(),
                    portfolio_manager.clone(),
                    risk_manager.clone(),
                    monitoring.clone(),
                );
                
                info!("🌐 Starting gRPC server on port {}", grpc_port);
                grpc_server.start(grpc_port).await?;
            } else {
                // Keep the main thread alive
                tokio::signal::ctrl_c().await?;
                info!("🛑 Shutting down arbitrage bot");
            }
        }
        Commands::Scan { min_profit, max_amount } => {
            info!("🔍 Scanning for arbitrage opportunities...");
            let opportunities = arbitrage_engine.scan_opportunities(min_profit, max_amount).await?;
            
            if opportunities.is_empty() {
                info!("❌ No profitable opportunities found");
            } else {
                info!("✅ Found {} opportunities:", opportunities.len());
                for (i, opp) in opportunities.iter().enumerate() {
                    info!("  {}. {}: {:.2}% profit, ${:.2} estimated", 
                          i + 1, opp.token_pair, opp.profit_percentage, opp.estimated_profit);
                }
            }
        }
        Commands::Portfolio => {
            let portfolio = portfolio_manager.get_portfolio().await?;
            info!("💰 Portfolio Value: ${:.2}", portfolio.total_value_usd);
            for balance in portfolio.balances {
                info!("  {}: {:.4} (${:.2})", balance.symbol, balance.amount, balance.value_usd);
            }
        }
        Commands::Risk { max_position, max_daily_loss, max_slippage } => {
            let mut risk = risk_manager.write().await;
            if let Some(pos) = max_position {
                risk.update_max_position_size(pos);
                info!("📊 Updated max position size to ${:.2}", pos);
            }
            if let Some(loss) = max_daily_loss {
                risk.update_max_daily_loss(loss);
                info!("📊 Updated max daily loss to ${:.2}", loss);
            }
            if let Some(slip) = max_slippage {
                risk.update_max_slippage(slip);
                info!("📊 Updated max slippage to {:.2}%", slip);
            }
        }
        Commands::TestJupiter { input_mint, output_mint, amount } => {
            if let Some(jupiter_client) = jupiter_client {
                info!("🧪 Testing Jupiter integration: {} -> {} (amount: {})", 
                      input_mint, output_mint, amount);
                
                use crate::jupiter_client::JupiterQuoteRequest;
                let request = JupiterQuoteRequest {
                    input_mint: input_mint.clone(),
                    output_mint: output_mint.clone(),
                    amount,
                    slippage_bps: 50, // 0.5%
                    swap_mode: Some("ExactIn".to_string()),
                    dexes: None,
                    exclude_dexes: None,
                    platform_fee_bps: None,
                    max_accounts: Some(64),
                };

                match jupiter_client.get_quote(request).await {
                    Ok(quote) => {
                        info!("✅ Jupiter quote received:");
                        info!("  Input: {} {} tokens", quote.in_amount, input_mint);
                        info!("  Output: {} {} tokens", quote.out_amount, output_mint);
                        info!("  Price impact: {:.2}%", quote.price_impact_pct);
                        info!("  Time taken: {:.2}ms", quote.time_taken);
                        info!("  Route: {} steps", quote.route_plan.len());
                    }
                    Err(e) => {
                        error!("❌ Jupiter quote failed: {}", e);
                    }
                }
            } else {
                error!("❌ Jupiter client not available. Enable Jupiter in config.");
            }
        }
        Commands::TestMetis { input_mint, output_mint, amount } => {
            if let Some(jupiter_client) = jupiter_client {
                info!("🔮 Testing Metis integration: {} -> {} (amount: {})", 
                      input_mint, output_mint, amount);
                
                use crate::types::{MetisQuoteRequest, MetisOptimization};
                let request = MetisQuoteRequest {
                    input_mint: input_mint.clone(),
                    output_mint: output_mint.clone(),
                    amount,
                    slippage_bps: 50, // 0.5%
                    swap_mode: Some("ExactIn".to_string()),
                    dexes: None,
                    exclude_dexes: None,
                    platform_fee_bps: None,
                    max_accounts: Some(64),
                    metis_optimization: Some(MetisOptimization {
                        enabled: true,
                        optimization_level: 3,
                        max_iterations: 100,
                        convergence_threshold: 0.001,
                    }),
                    cross_app_state: None,
                };

                match jupiter_client.get_metis_quote(request).await {
                    Ok(quote) => {
                        info!("✅ Metis quote received:");
                        info!("  Input: {} {} tokens", quote.in_amount, input_mint);
                        info!("  Output: {} {} tokens", quote.out_amount, output_mint);
                        info!("  Price impact: {:.2}%", quote.price_impact_pct);
                        info!("  Time taken: {:.2}ms", quote.time_taken);
                        info!("  Route: {} steps", quote.route_plan.len());
                        if let Some(opt) = &quote.metis_optimization {
                            info!("  Metis optimization: level {}, {} iterations", 
                                  opt.optimization_level, opt.max_iterations);
                        }
                    }
                    Err(e) => {
                        error!("❌ Metis quote failed: {}", e);
                    }
                }
            } else {
                error!("❌ Jupiter client not available. Enable Jupiter in config.");
            }
        }
        Commands::TestUltra { input_mint, output_mint, amount } => {
            if let Some(jupiter_client) = jupiter_client {
                info!("⚡ Testing Ultra API integration: {} -> {} (amount: {})", 
                      input_mint, output_mint, amount);
                
                use crate::types::{UltraQuoteRequest, UltraFeatures, SlippageProtection};
                let request = UltraQuoteRequest {
                    input_mint: input_mint.clone(),
                    output_mint: output_mint.clone(),
                    amount,
                    slippage_bps: 50, // 0.5%
                    swap_mode: Some("ExactIn".to_string()),
                    dexes: None,
                    exclude_dexes: None,
                    platform_fee_bps: None,
                    max_accounts: Some(64),
                    ultra_features: Some(UltraFeatures {
                        enabled: true,
                        advanced_routing: true,
                        mev_protection: true,
                        gas_optimization: true,
                        price_impact_optimization: true,
                    }),
                    slippage_protection: Some(SlippageProtection {
                        enabled: true,
                        max_slippage_bps: 50,
                        price_impact_threshold: 2.0,
                        dynamic_slippage: true,
                    }),
                };

                match jupiter_client.get_ultra_quote(request).await {
                    Ok(quote) => {
                        info!("✅ Ultra quote received:");
                        info!("  Input: {} {} tokens", quote.in_amount, input_mint);
                        info!("  Output: {} {} tokens", quote.out_amount, output_mint);
                        info!("  Price impact: {:.2}%", quote.price_impact_pct);
                        info!("  Time taken: {:.2}ms", quote.time_taken);
                        info!("  Route: {} steps", quote.route_plan.len());
                        if let Some(features) = &quote.ultra_features {
                            info!("  Ultra features: MEV protection: {}, Gas optimization: {}", 
                                  features.mev_protection, features.gas_optimization);
                        }
                    }
                    Err(e) => {
                        error!("❌ Ultra quote failed: {}", e);
                    }
                }
            } else {
                error!("❌ Jupiter client not available. Enable Jupiter in config.");
            }
        }
        Commands::Health => {
            if let Some(jupiter_client) = jupiter_client {
                info!("🏥 Checking Jupiter API health status");
                
                match jupiter_client.get_health_status().await {
                    Ok(health) => {
                        info!("✅ Jupiter API Health Status:");
                        info!("  Status: {:?}", health.status);
                        info!("  Version: {}", health.version);
                        info!("  Uptime: {} seconds", health.uptime);
                        if let Some(error) = &health.last_error {
                            info!("  Last error: {}", error);
                        }
                        if let Some(rate_limit) = &health.rate_limit_status {
                            info!("  Rate limit: {}/{} remaining", rate_limit.remaining, rate_limit.limit);
                        }
                    }
                    Err(e) => {
                        error!("❌ Health check failed: {}", e);
                    }
                }
            } else {
                error!("❌ Jupiter client not available. Enable Jupiter in config.");
            }
        }
        Commands::Info => {
            if let Some(jupiter_client) = jupiter_client {
                info!("ℹ️ Getting Jupiter API information");
                
                match jupiter_client.get_api_info().await {
                    Ok(info) => {
                        info!("✅ Jupiter API Information:");
                        info!("  Version: {}", info.version);
                        info!("  API Type: {}", info.api_type);
                        info!("  Supported features: {:?}", info.supported_features);
                        info!("  Rate limits: {}/min, {}/hour, {}/day", 
                              info.rate_limits.requests_per_minute,
                              info.rate_limits.requests_per_hour,
                              info.rate_limits.requests_per_day);
                        info!("  Available endpoints: {}", info.endpoints.len());
                        for endpoint in &info.endpoints {
                            info!("    {} {} - {} req/min", endpoint.method, endpoint.path, endpoint.rate_limit);
                        }
                    }
                    Err(e) => {
                        error!("❌ API info request failed: {}", e);
                    }
                }
            } else {
                error!("❌ Jupiter client not available. Enable Jupiter in config.");
            }
        }
    }
    
    Ok(())
}

trait CommandExt {
    fn is_jito_enabled(&self) -> bool;
}

impl CommandExt for Commands {
    fn is_jito_enabled(&self) -> bool {
        match self {
            Commands::Start { jito, .. } => *jito,
            _ => false,
        }
    }
}
