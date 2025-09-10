use crate::health::types::{AlertInfo, AlertSeverity, HealthAction, ActionType};
use crate::health::config::HealthConfig;
use tracing::{error, warn};

pub struct DiskPressureManager {
    config: HealthConfig,
}

impl DiskPressureManager {
    pub fn new(config: HealthConfig) -> Self {
        Self { config }
    }

    pub fn check_disk_pressure(&self, disk_percent: f32) -> (Vec<AlertInfo>, Vec<HealthAction>) {
        let mut alerts = Vec::new();
        let mut actions = Vec::new();

        if disk_percent > self.config.disk.pressure_threshold_percent {
            alerts.push(AlertInfo {
                alert_id: format!("disk-pressure-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "disk_pressure".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Disk pressure high: {:.1}% — taking action", disk_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "disk_pressure_manager".to_string(),
                recommended_action: "Rotate WAL, drop camera frames, reduce logging".to_string(),
            });

            if disk_percent > self.config.disk.wal_rotate_early_at_percent {
                actions.push(HealthAction {
                    action_id: format!("wal-rotate-{}", chrono::Utc::now().timestamp_nanos()),
                    action_type: ActionType::RotateWalEarly,
                    target_module: "wal".to_string(),
                    parameters: serde_json::json!({"reason": "disk_pressure"}),
                    executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                    success: false,
                    message: "Forcing WAL checkpoint due to disk pressure".to_string(),
                });
            }

            if disk_percent > self.config.disk.camera_frame_drop_at_percent {
                actions.push(HealthAction {
                    action_id: format!("drop-frames-{}", chrono::Utc::now().timestamp_nanos()),
                    action_type: ActionType::DropCameraFrames,
                    target_module: "camera".to_string(),
                    parameters: serde_json::json!({"drop_percent": 50}),
                    executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                    success: false,
                    message: "Dropping 50% of camera frames due to disk pressure".to_string(),
                });
            }

            actions.push(HealthAction {
                action_id: format!("reduce-logs-{}", chrono::Utc::now().timestamp_nanos()),
                action_type: ActionType::ReduceLogLevel,
                target_module: "system".to_string(),
                parameters: serde_json::json!({"new_level": "warn"}),
                executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                success: false,
                message: "Reducing log level to WARN due to disk pressure".to_string(),
            });
        }

        (alerts, actions)
    }
}