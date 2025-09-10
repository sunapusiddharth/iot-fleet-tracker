use tokio;
use tracing::{info, Level};
use truck_central_server::telemetry;
use truck_central_server::server::config::ServerConfig;
use truck_central_server::server::CentralServer;

mod server;
mod models;
mod services;
mod utils;
mod telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging and metrics
    telemetry::init_tracing();
    telemetry::init_metrics("0.0.0.0:9092".parse().unwrap())?;

    info!("🚛 Starting Truck Central Server");

    // Load configuration
    let config_path = "config/server.toml";
    let config = ServerConfig::load_from_file(config_path)
        .map_err(|e| {
            tracing::error!(error = %e, "❌ Failed to load config — CRASHING");
            e
        })?;

    // Create central server
    let server = CentralServer::new(config).await?;

    // Start server
    server.start().await?;

    Ok(())
}