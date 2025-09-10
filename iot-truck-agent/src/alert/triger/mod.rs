use crate::alert::types::Alert;
use crate::alert::error::Result;
use crate::ml_edge::types::MLEvent;
use crate::health::types::HealthEvent;
use crate::sensors::types::SensorEvent;
use tracing::{info, warn};

pub struct AlertTriggerEngine {
    ml_triggers: crate::alert::trigger::ml::MlTriggerEngine,
    health_triggers: crate::alert::trigger::health::HealthTriggerEngine,
    sensor_triggers: crate::alert::trigger::sensor::SensorTriggerEngine,
}

impl AlertTriggerEngine {
    pub fn new() -> Self {
        Self {
            ml_triggers: crate::alert::trigger::ml::MlTriggerEngine::new(),
            health_triggers: crate::alert::trigger::health::HealthTriggerEngine::new(),
            sensor_triggers: crate::alert::trigger::sensor::SensorTriggerEngine::new(),
        }
    }

    pub fn trigger_from_ml(&self, ml_event: &MLEvent) -> Option<Alert> {
        self.ml_triggers.trigger_from_ml(ml_event)
    }

    pub fn trigger_from_health(&self, health_event: &HealthEvent) -> Vec<Alert> {
        self.health_triggers.trigger_from_health(health_event)
    }

    pub fn trigger_from_sensor(&self, sensor_event: &SensorEvent) -> Option<Alert> {
        self.sensor_triggers.trigger_from_sensor(sensor_event)
    }
}