use crate::ml_edge::types::{ModelConfig, SensorContext};
use crate::health::types::ResourceUsage; // From Module 7
use std::time::Instant;
use tracing::{warn};

pub struct MLScheduler {
    last_inference: std::time::Instant,
    model_configs: std::collections::HashMap<String, ModelConfig>,
}

impl MLScheduler {
    pub fn new() -> Self {
        Self {
            last_inference: Instant::now(),
            model_configs: std::collections::HashMap::new(),
        }
    }

    pub fn add_model_config(&mut self, config: ModelConfig) {
        self.model_configs.insert(config.name.clone(), config);
    }

    pub fn should_run_model(&self, model_name: &str, resources: &ResourceUsage, context: Option<&SensorContext>) -> bool {
        let config = match self.model_configs.get(model_name) {
            Some(c) => c,
            None => return false,
        };

        // Check CPU usage
        if resources.cpu_percent > 85.0 {
            warn!(model=%model_name, cpu_percent=resources.cpu_percent, "🛑 Skipping ML inference due to high CPU");
            return false;
        }

        // Check temperature
        if resources.temperature_c > 75.0 {
            warn!(model=%model_name, temp=resources.temperature_c, "🛑 Skipping ML inference due to high temperature");
            return false;
        }

        // Check memory
        if resources.memory_percent > 90.0 {
            warn!(model=%model_name, memory_percent=resources.memory_percent, "🛑 Skipping ML inference due to high memory usage");
            return false;
        }

        // Check if we're in calibration mode for this route/time
        if let Some(ctx) = context {
            if self.is_calibration_mode(&ctx.route_id, &ctx.time_of_day) {
                // Run at reduced frequency for calibration
                let elapsed = self.last_inference.elapsed().as_secs();
                if elapsed < 10 { // Only run every 10 seconds during calibration
                    return false;
                }
            }
        }

        // Check model-specific FPS limit
        let elapsed = self.last_inference.elapsed().as_millis();
        let min_interval = 1000 / config.max_fps as u128;
        if elapsed < min_interval {
            return false;
        }

        true
    }

    pub fn record_inference(&mut self) {
        self.last_inference = Instant::now();
    }

    fn is_calibration_mode(&self, route_id: &str, time_of_day: &str) -> bool {
        // In production, check against calibration schedule
        false
    }
}