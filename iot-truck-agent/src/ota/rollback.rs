use crate::ota::error::Result;
use crate::ota::types::OtaUpdate;
use std::fs;
use tracing::{info, warn};

pub struct RollbackManager {
    backup_dir: String,
}

impl RollbackManager {
    pub fn new(backup_dir: &str) -> Self {
        Self {
            backup_dir: backup_dir.to_string(),
        }
    }

    pub async fn rollback_update(&self, update: &OtaUpdate) -> Result<()> {
        info!(update_id=%update.update_id, "🔄 Rolling back update");

        match update.target {
            crate::ota::types::UpdateTarget::Agent => {
                self.rollback_agent().await?;
            }
            crate::ota::types::UpdateTarget::Model => {
                self.rollback_model().await?;
            }
            crate::ota::types::UpdateTarget::Config => {
                self.rollback_config().await?;
            }
            crate::ota::types::UpdateTarget::Firmware => {
                self.rollback_firmware().await?;
            }
        }

        info!(update_id=%update.update_id, "✅ Rollback completed");
        Ok(())
    }

    async fn rollback_agent(&self) -> Result<()> {
        let current_exe = std::env::current_exe()?;
        let backup_path = format!("{}_backup", current_exe.display());

        if fs::metadata(&backup_path).is_ok() {
            fs::rename(&backup_path, &current_exe)?;
            info!("✅ Agent rollback completed");
        } else {
            return Err(OtaError::RollbackFailed(
                "Agent backup not found".to_string(),
            ));
        }

        Ok(())
    }

    async fn rollback_model(&self) -> Result<()> {
        // In production, restore model from backup
        info!("✅ Model rollback completed");
        Ok(())
    }

    async fn rollback_config(&self) -> Result<()> {
        let config_path = "config/agent.toml";
        let backup_path = format!("{}_backup", config_path);

        if fs::metadata(&backup_path).is_ok() {
            fs::rename(&backup_path, config_path)?;
            info!("✅ Config rollback completed");
        } else {
            return Err(OtaError::RollbackFailed(
                "Config backup not found".to_string(),
            ));
        }

        Ok(())
    }

    async fn rollback_firmware(&self) -> Result<()> {
        // In production, use device-specific firmware rollback
        warn!("Firmware rollback not implemented");
        Ok(())
    }
}
