use crate::alert::types::{Alert, ActionType};
use crate::alert::error::Result;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct ActuatorRegistry {
    actuators: RwLock<HashMap<String, Box<dyn Actuator + Send + Sync>>>,
}

impl ActuatorRegistry {
    pub fn new() -> Self {
        Self {
            actuators: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register_actuator(&self, name: &str, actuator: Box<dyn Actuator + Send + Sync>) {
        let mut actuators = self.actuators.write().await;
        actuators.insert(name.to_string(), actuator);
        tracing::info!(name=%name, "✅ Actuator registered");
    }

    pub async fn trigger_action(&self, alert: &Alert, action: &crate::alert::types::AlertAction) -> Result<()> {
        let actuators = self.actuators.read().await;
        let actuator = actuators.get(&action.target)
            .ok_or_else(|| AlertError::ActuatorNotFound(action.target.clone()))?;

        actuator.trigger(alert, action).await
    }

    pub async fn get_actuator(&self, name: &str) -> Option<Box<dyn Actuator + Send + Sync>> {
        let actuators = self.actuators.read().await;
        actuators.get(name).map(|a| a.box_clone())
    }
}

#[async_trait::async_trait]
pub trait Actuator: Send + Sync {
    async fn trigger(&self, alert: &Alert, action: &crate::alert::types::AlertAction) -> Result<()>;
    fn get_type(&self) -> ActionType;
    fn box_clone(&self) -> Box<dyn Actuator + Send + Sync>;
}