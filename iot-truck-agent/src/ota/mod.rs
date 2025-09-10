use crate::config::Config;
use crate::health::types::NetworkHealth;
use crate::ota::error::Result;
use crate::ota::types::{CommandResponse, OtaStatus, OtaUpdate, RemoteCommand};
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

pub mod bandwidth_manager;
pub mod command_executor;
pub mod config_manager;
pub mod error;
pub mod health_reporter;
pub mod rollback;
pub mod types;
pub mod updater;

// Metrics
metrics::describe_counter!("ota_updates_total", "Total OTA updates");
metrics::describe_counter!("ota_updates_success", "Successful OTA updates");
metrics::describe_counter!("ota_updates_failed", "Failed OTA updates");
metrics::describe_counter!("ota_rollbacks_total", "Total rollbacks");
metrics::describe_gauge!("ota_update_progress", "OTA update progress 0-100");
metrics::describe_counter!("remote_commands_total", "Total remote commands");
metrics::describe_counter!("remote_commands_success", "Successful remote commands");
metrics::describe_counter!("remote_commands_failed", "Failed remote commands");

pub struct OtaManager {
    config: Config,
    network_health: std::sync::Arc<tokio::sync::RwLock<NetworkHealth>>,
    command_rx: mpsc::Receiver<RemoteCommand>,
    command_tx: mpsc::Sender<CommandResponse>,
    device_id: String,
}

impl OtaManager {
    pub async fn new(
        config: Config,
        network_health: std::sync::Arc<tokio::sync::RwLock<NetworkHealth>>,
        command_rx: mpsc::Receiver<RemoteCommand>,
        command_tx: mpsc::Sender<CommandResponse>,
    ) -> Result<Self> {
        let device_id = config.device_id.clone();

        info!("✅ OTA Manager initialized");

        Ok(Self {
            config,
            network_health,
            command_rx,
            command_tx,
            device_id,
        })
    }

    pub async fn start(&self) -> Result<()> {
        info!("👂 Starting OTA command listener");

        let mut update_status = None;

        loop {
            tokio::select! {
                Some(command) = self.command_rx.recv() => {
                    metrics::counter!("remote_commands_total").increment(1);

                    let executor = crate::ota::command_executor::CommandExecutor;
                    let response = executor.execute_command(&command).await;

                    if let Err(e) = self.command_tx.send(response.clone()).await {
                        error!(error=%e, "Failed to send command response");
                    }

                    match response.status {
                        crate::ota::types::CommandStatus::Success => {
                            metrics::counter!("remote_commands_success").increment(1);
                        }
                        _ => {
                            metrics::counter!("remote_commands_failed").increment(1);
                        }
                    }
                }
                _ = sleep(Duration::from_secs(60)) => {
                    // Periodic health check
                    if let Some(status) = &update_status {
                        if status.status == crate::ota::types::UpdateStatus::Applying {
                            // Check if update is still progressing
                        }
                    }
                }
            }
        }
    }

    pub async fn apply_update(&self, update: OtaUpdate) -> Result<OtaStatus> {
        metrics::counter!("ota_updates_total").increment(1);

        let bandwidth_manager = crate::ota::bandwidth_manager::BandwidthManager::new(
            self.network_health.clone(),
            1000, // 1 Mbps max
        );

        // Check if we should delay based on priority and network
        if !bandwidth_manager
            .should_delay_update(&update.priority)
            .await
        {
            return Err(OtaError::Timeout);
        }

        let downloader = crate::ota::updater::download::BandwidthAwareDownloader::new(
            bandwidth_manager.get_max_download_bandwidth().await,
            &self.config.storage.wal_path,
        );

        let file_path = downloader.download_update(&update).await?;

        let verifier = crate::ota::updater::verify::UpdateVerifier;
        verifier.verify_update(&file_path, &update.checksum, &update.signature)?;

        let applier = crate::ota::updater::apply::UpdateApplier::new(
            &self.config.version,
            &format!("{}/backups", self.config.storage.wal_path),
        );

        let status = applier.apply_update(&update, &file_path).await;

        match &status {
            Ok(s) if s.status == crate::ota::types::UpdateStatus::Success => {
                metrics::counter!("ota_updates_success").increment(1);
            }
            _ => {
                metrics::counter!("ota_updates_failed").increment(1);

                // Auto-rollback on failure
                let rollback_manager = crate::ota::rollback::RollbackManager::new(&format!(
                    "{}/backups",
                    self.config.storage.wal_path
                ));
                if let Err(e) = rollback_manager.rollback_update(&update).await {
                    error!(error=%e, "Rollback failed");
                    metrics::counter!("ota_rollbacks_total").increment(1);
                }
            }
        }

        status
    }
}
