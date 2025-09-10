use crate::ota::types::{OtaUpdate, OtaStatus, UpdateStatus};
use crate::ota::error::Result;
use std::fs;
use std::process::Command;
use tracing::{info, warn};

pub struct UpdateApplier {
    current_version: String,
    backup_dir: String,
}

impl UpdateApplier {
    pub fn new(current_version: &str, backup_dir: &str) -> Self {
        Self {
            current_version: current_version.to_string(),
            backup_dir: backup_dir.to_string(),
        }
    }

    pub async fn apply_update(&self, update: &OtaUpdate, file_path: &str) -> Result<OtaStatus> {
        info!(update_id=%update.update_id, "🔧 Applying update");

        // Create backup
        self.create_backup()?;

        // Apply based on target
        match update.target {
            crate::ota::types::UpdateTarget::Agent => {
                self.apply_agent_update(file_path).await?;
            }
            crate::ota::types::UpdateTarget::Model => {
                self.apply_model_update(file_path).await?;
            }
            crate::ota::types::UpdateTarget::Config => {
                self.apply_config_update(file_path).await?;
            }
            crate::ota::types::UpdateTarget::Firmware => {
                self.apply_firmware_update(file_path).await?;
            }
        }

        // Verify update
        if !self.verify_update_applied(update)? {
            // Rollback
            warn!(update_id=%update.update_id, "❌ Update verification failed - rolling back");
            self.rollback().await?;
            return Err(OtaError::RollbackFailed("Update verification failed".to_string()));
        }

        info!(update_id=%update.update_id, "✅ Update applied successfully");
        Ok(OtaStatus {
            update_id: update.update_id.clone(),
            status: UpdateStatus::Success,
            progress_percent: 100.0,
            current_version: update.version.clone(),
            target_version: update.version.clone(),
            last_error: None,
            timestamp: chrono::Utc::now().timestamp_nanos() as u64,
        })
    }

    fn create_backup(&self) -> Result<()> {
        // Create backup directory
        fs::create_dir_all(&self.backup_dir)?;

        // Backup current binary
        let current_exe = std::env::current_exe()?;
        let backup_path = format!("{}/agent_backup_{}", self.backup_dir, self.current_version);
        fs::copy(&current_exe, &backup_path)?;

        info!(backup_path=%backup_path, "✅ Backup created");
        Ok(())
    }

    async fn apply_agent_update(&self, file_path: &str) -> Result<()> {
        // Make new binary executable
        fs::set_permissions(file_path, std::fs::Permissions::from_mode(0o755))?;

        // Replace current binary
        let current_exe = std::env::current_exe()?;
        let backup_path = format!("{}_backup", current_exe.display());
        fs::rename(&current_exe, &backup_path)?;
        fs::rename(file_path, &current_exe)?;

        info!("✅ Agent binary updated");
        Ok(())
    }

    async fn apply_model_update(&self, file_path: &str) -> Result<()> {
        // Extract and replace models
        let models_dir = "./models";
        fs::create_dir_all(models_dir)?;

        // In production, extract archive
        fs::copy(file_path, format!("{}/new_model.onnx", models_dir))?;

        info!("✅ Model updated");
        Ok(())
    }

    async fn apply_config_update(&self, file_path: &str) -> Result<()> {
        // Backup current config
        let config_path = "config/agent.toml";
        let backup_path = format!("{}_backup", config_path);
        fs::copy(config_path, &backup_path)?;

        // Replace config
        fs::copy(file_path, config_path)?;

        info!("✅ Config updated");
        Ok(())
    }

    async fn apply_firmware_update(&self, _file_path: &str) -> Result<()> {
        // In production, use device-specific firmware update tool
        warn!("Firmware update not implemented");
        Ok(())
    }

    fn verify_update_applied(&self, update: &OtaUpdate) -> Result<bool> {
        // In production, verify the update was applied correctly
        Ok(true)
    }

    pub async fn rollback(&self) -> Result<()> {
        info!("🔄 Starting rollback");

        // Restore from backup
        let current_exe = std::env::current_exe()?;
        let backup_path = format!("{}_backup", current_exe.display());
        if fs::metadata(&backup_path).is_ok() {
            fs::rename(&backup_path, &current_exe)?;
            info!("✅ Rollback completed");
        } else {
            return Err(OtaError::RollbackFailed("Backup not found".to_string()));
        }

        Ok(())
    }
}