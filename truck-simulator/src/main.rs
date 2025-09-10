fn main() {
    println!("Hello, world!");
}
use tokio;
use tracing::{info, Level};
use truck_simulator::telemetry;
use truck_simulator::config::SimulatorConfig;
use truck_simulator::simulator::Simulator;

mod simulator;
mod config;
mod telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging and metrics
    telemetry::init_tracing();
    telemetry::init_metrics("0.0.0.0:9091".parse().unwrap())?;

    info!("🚛 Starting Truck Simulator Server");

    // Load configuration
    let config_path = "config/simulator.toml";
    let config = SimulatorConfig::load_from_file(config_path)
        .map_err(|e| {
            tracing::error!(error = %e, "❌ Failed to load config — CRASHING");
            e
        })?;

    // Create simulator
    let mut simulator = Simulator::new(config).await?;

    // Start simulator
    simulator.start().await?;

    Ok(())
}