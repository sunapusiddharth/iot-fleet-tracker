use crate::config::Config;
use crate::ota::types::RemoteCommand;
use crate::ota::error::Result;
use tokio::fs;
use tracing::{info, warn};

pub struct RemoteConfigManager;

impl RemoteConfigManager {
    pub async fn update_config(&self, command: &RemoteCommand) -> Result<serde_json::Value> {
        info!(command_id=%command.command_id, "⚙️  Updating configuration");

        if let Some(params) = command.parameters.get("config") {
            let new_config: Config = serde_json::from_value(params.clone())?;
            
            // Validate config
            new_config.validate()?;
            
            // Backup current config
            let config_path = "config/agent.toml";
            let backup_path = format!("{}_backup", config_path);
            fs::copy(config_path, &backup_path).await?;
            
            // Write new config
            let toml = toml::to_string(&new_config)?;
            fs::write(config_path, toml).await?;
            
            info!("✅ Configuration updated");
            
            // In production, signal other modules to reload config
            Ok(serde_json::json!({"status": "success", "message": "Configuration updated"}))
        } else {
            Err(OtaError::CommandFailed("No config provided".to_string()))
        }
    }
}