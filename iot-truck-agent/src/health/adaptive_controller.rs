use crate::health::types::{HealthAction, ActionType, AlertSeverity};
use crate::health::config::HealthConfig;
use std::collections::VecDeque;
use tracing::{error, info, warn};

pub struct AdaptiveController {
    config: HealthConfig,
    pending_actions: VecDeque<HealthAction>,
}

impl AdaptiveController {
    pub fn new(config: HealthConfig) -> Self {
        Self {
            config,
            pending_actions: VecDeque::new(),
        }
    }

    pub fn evaluate_system_health(
        &mut self,
        cpu_percent: f32,
        memory_percent: f32,
        disk_percent: f32,
        network_latency: f32,
    ) -> Vec<HealthAction> {
        let mut actions = Vec::new();

        // CPU-based degradation
        if cpu_percent > self.config.degradation.cpu_degrade_threshold {
            let reduction = ((cpu_percent - self.config.degradation.cpu_degrade_threshold) / 10.0).ceil() as u32;
            let fps_reduction = reduction * self.config.degradation.camera_fps_degrade_step;

            actions.push(HealthAction {
                action_id: format!("cpu-degrade-{}", chrono::Utc::now().timestamp_nanos()),
                action_type: ActionType::ThrottleCameraFps,
                target_module: "camera".to_string(),
                parameters: serde_json::json!({"reduction_fps": fps_reduction}),
                executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                success: false,
                message: format!("Reducing camera FPS by {} due to high CPU", fps_reduction),
            });

            // Disable ML models in order
            let models_to_disable = (reduction as usize).min(self.config.degradation.ml_model_disable_order.len());
            for i in 0..models_to_disable {
                if let Some(model) = self.config.degradation.ml_model_disable_order.get(i) {
                    actions.push(HealthAction {
                        action_id: format!("disable-ml-{}-{}", model, chrono::Utc::now().timestamp_nanos()),
                        action_type: ActionType::DisableMlModel,
                        target_module: "ml_edge".to_string(),
                        parameters: serde_json::json!({"model": model}),
                        executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                        success: false,
                        message: format!("Disabling ML model {} due to high CPU", model),
                    });
                }
            }
        }

        // Memory-based degradation
        if memory_percent > self.config.degradation.memory_degrade_threshold {
            let reduction = ((memory_percent - self.config.degradation.memory_degrade_threshold) / 10.0).ceil() as u32;
            let rate_reduction = reduction * self.config.degradation.sensor_rate_degrade_step;

            actions.push(HealthAction {
                action_id: format!("mem-degrade-{}", chrono::Utc::now().timestamp_nanos()),
                action_type: ActionType::ReduceSensorRate,
                target_module: "sensors".to_string(),
                parameters: serde_json::json!({"reduction_percent": rate_reduction * 10}),
                executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                success: false,
                message: format!("Reducing sensor rate by {}% due to high memory", rate_reduction * 10),
            });
        }

        // Network-based degradation
        if network_latency > self.config.degradation.network_degrade_threshold {
            actions.push(HealthAction {
                action_id: format!("net-degrade-{}", chrono::Utc::now().timestamp_nanos()),
                action_type: ActionType::ThrottleCameraFps,
                target_module: "camera".to_string(),
                parameters: serde_json::json!({"reduction_percent": 50}),
                executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                success: false,
                message: "Reducing camera FPS by 50% due to high network latency".to_string(),
            });

            actions.push(HealthAction {
                action_id: format!("net-disable-ml-{}", chrono::Utc::now().timestamp_nanos()),
                action_type: ActionType::DisableMlModel,
                target_module: "ml_edge".to_string(),
                parameters: serde_json::json!({"model": "license_plate"}),
                executed_at: chrono::Utc::now().timestamp_nanos() as u64,
                success: false,
                message: "Disabling license plate ML model due to network latency".to_string(),
            });
        }

        // Add to pending actions
        for action in &actions {
            self.pending_actions.push_back(action.clone());
        }

        actions
    }

    pub fn get_next_action(&mut self) -> Option<HealthAction> {
        self.pending_actions.pop_front()
    }

    pub fn mark_action_complete(&mut self, action_id: &str, success: bool, message: &str) {
        // In production, update the action in persistent storage
        info!(action_id, success, message, "Action completed");
    }
}