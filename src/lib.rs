pub mod config;
pub mod arbitrage_engine;
pub mod dex_monitor;
pub mod grpc_server;
pub mod jito_client;
pub mod risk_manager;
pub mod portfolio_manager;
pub mod monitoring;
pub mod utils;
pub mod types;

pub use config::Config;
pub use arbitrage_engine::ArbitrageEngine;
pub use dex_monitor::DexMonitor;
pub use grpc_server::ArbitrageGrpcServer;
pub use jito_client::JitoClient;
pub use risk_manager::RiskManager;
pub use portfolio_manager::PortfolioManager;
pub use monitoring::MonitoringService;

// Generated gRPC code
pub mod arbitrage {
    tonic::include_proto!("arbitrage");
}

pub use arbitrage::*;
