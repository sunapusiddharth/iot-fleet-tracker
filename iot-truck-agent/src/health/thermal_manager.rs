use crate::health::types::{AlertInfo, AlertSeverity, HealthAction, ActionType};
use crate::health::config::HealthConfig;
use std::fs;
use tracing::{error, warn};

pub struct ThermalManager {
    config: HealthConfig,
}

impl ThermalManager {
    pub fn new(config: HealthConfig) -> Self {
        Self { config }
    }

    pub fn check_thermal(&self, temperature_c: f32) -> (Vec<AlertInfo>, Vec<HealthAction>) {
        let mut alerts = Vec::new();
        let mut actions = Vec::new();

        if temperature_c > self.config.thermal.critical_shutdown_temp_c {
            alerts.push(AlertInfo {
                alert_id: format!("thermal-shutdown-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "thermal_shutdown_imminent".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("Temperature critical: {:.1}°C — shutdown imminent", temperature_c),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "thermal_manager".to_string(),
                recommended_action: "Immediate shutdown to prevent hardware damage".to_string(),
            });

            actions.push(HealthAction {
                action_id: format!("thermal-shutdown-{}", chrono::Utc::now().timestamp_nanos()),
                action_type: ActionType::RebootSystem,
                target_module: "system".to_string(),
                parameters: serde_json::json!({"reason": "thermal_shutdown", "temperature": temperature_c}),
                executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                success: false, // Will be set by executor
                message: "Scheduled system shutdown due to thermal emergency".to_string(),
            });
        } else if temperature_c > self.config.thermal.throttle_at_temp_c {
            alerts.push(AlertInfo {
                alert_id: format!("thermal-throttle-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "thermal_throttling".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Temperature high: {:.1}°C — throttling system", temperature_c),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "thermal_manager".to_string(),
                recommended_action: "Reduce CPU load, increase cooling".to_string(),
            });

            actions.push(HealthAction {
                action_id: format!("thermal-throttle-{}", chrono::Utc::now().timestamp_nanos()),
                action_type: ActionType::ThrottleCameraFps,
                target_module: "camera".to_string(),
                parameters: serde_json::json!({"reduction_percent": 50}),
                executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                success: false,
                message: "Reducing camera FPS due to thermal conditions".to_string(),
            });

            actions.push(HealthAction {
                action_id: format!("thermal-disable-ml-{}", chrono::Utc::now().timestamp_nanos()),
                action_type: ActionType::DisableMlModel,
                target_module: "ml_edge".to_string(),
                parameters: serde_json::json!({"model": "license_plate"}),
                executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                success: false,
                message: "Disabling heavy ML model due to thermal conditions".to_string(),
            });
        }

        (alerts, actions)
    }
}