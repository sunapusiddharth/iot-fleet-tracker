use crate::alert::types::{Alert, AlertAction};
use crate::alert::error::Result;
use crate::ml_edge::types::MLEvent;
use crate::health::types::HealthEvent;
use crate::sensors::types::SensorEvent;
use crate::stream::types::StreamEvent;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub mod types;
pub mod error;
pub mod actuator;
pub mod policy;
pub mod trigger;

// Metrics
metrics::describe_counter!("alert_triggers_total", "Total alerts triggered");
metrics::describe_counter!("alert_suppressed_total", "Alerts suppressed due to cooldown");
metrics::describe_counter!("alert_actions_total", "Total alert actions executed");
metrics::describe_counter!("alert_actions_success", "Successful alert actions");
metrics::describe_counter!("alert_actions_failed", "Failed alert actions");
metrics::describe_gauge!("alert_active_count", "Number of active alerts");

pub struct AlertManager {
    actuator_registry: crate::alert::actuator::ActuatorRegistry,
    debouncer: crate::alert::policy::debounce::AlertDebouncer,
    escalator: crate::alert::policy::escalation::AlertEscalator,
    trigger_engine: crate::alert::trigger::AlertTriggerEngine,
    tx: mpsc::Sender<StreamEvent>,
    device_id: String,
    active_alerts: std::sync::RwLock<std::collections::HashMap<String, Alert>>,
}

impl AlertManager {
    pub async fn new(config: &crate::config::Config, tx: mpsc::Sender<StreamEvent>) -> Result<Self> {
        let mut actuator_registry = crate::alert::actuator::ActuatorRegistry::new();

        // Register GPIO actuators
        if config.alerts.enable_local_alerts {
            let buzzer = crate::alert::actuator::gpio::GpioActuator::new(config.alerts.gpio_buzzer_pin, crate::alert::types::ActionType::TriggerBuzzer)?;
            actuator_registry.register_actuator("buzzer_1", Box::new(buzzer)).await;

            let led_red = crate::alert::actuator::gpio::GpioActuator::new(22, crate::alert::types::ActionType::FlashLed)?;
            actuator_registry.register_actuator("led_red", Box::new(led_red)).await;

            let led_green = crate::alert::actuator::gpio::GpioActuator::new(23, crate::alert::types::ActionType::FlashLed)?;
            actuator_registry.register_actuator("led_green", Box::new(led_green)).await;
        }

        // Register relay actuators
        let relay = crate::alert::actuator::relay::RelayActuator::new(24, true)?;
        actuator_registry.register_actuator("relay_1", Box::new(relay)).await;

        // Register CAN bus actuator if configured
        if !config.alerts.can_interface.is_empty() {
            let can_bus = crate::alert::actuator::canbus::CanBusActuator::new(&config.alerts.can_interface)?;
            actuator_registry.register_actuator("can_bus_1", Box::new(can_bus)).await;
        }

        info!("✅ Alert Manager initialized with {} actuators", 3);

        Ok(Self {
            actuator_registry,
            debouncer: crate::alert::policy::debounce::AlertDebouncer::new(),
            escalator: crate::alert::policy::escalation::AlertEscalator::new(),
            trigger_engine: crate::alert::trigger::AlertTriggerEngine::new(),
            tx,
            device_id: config.device_id.clone(),
            active_alerts: std::sync::RwLock::new(std::collections::HashMap::new()),
        })
    }

    pub async fn process_ml_event(&self, ml_event: &MLEvent) -> Result<()> {
        if let Some(alert) = self.trigger_engine.trigger_from_ml(ml_event) {
            self.process_alert(alert).await?;
        }
        Ok(())
    }

    pub async fn process_health_event(&self, health_event: &HealthEvent) -> Result<()> {
        let alerts = self.trigger_engine.trigger_from_health(health_event);
        for alert in alerts {
            self.process_alert(alert).await?;
        }
        Ok(())
    }

    pub async fn process_sensor_event(&self, sensor_event: &SensorEvent) -> Result<()> {
        if let Some(alert) = self.trigger_engine.trigger_from_sensor(sensor_event) {
            self.process_alert(alert).await?;
        }
        Ok(())
    }

    async fn process_alert(&self, mut alert: Alert) -> Result<()> {
        // Apply debouncing
        if self.debouncer.should_suppress(&alert) {
            return Ok(());
        }

        // Get escalation actions
        let actions = self.escalator.get_actions_for_alert(&alert);
        alert.actions = actions;

        // Execute actions
        for action in &alert.actions {
            match self.actuator_registry.trigger_action(&alert, action).await {
                Ok(()) => {
                    metrics::counter!("alert_actions_success").increment(1);
                }
                Err(e) => {
                    error!(error=%e, "Failed to execute alert action");
                    metrics::counter!("alert_actions_failed").increment(1);
                }
            }
        }

        // Send to streamer
        let stream_event = StreamEvent::new_alert(alert.clone(), &self.device_id);
        if self.tx.send(stream_event).is_err() {
            warn!("Alert event channel full — dropping event");
        }

        // Track active alerts
        {
            let mut active = self.active_alerts.write().unwrap();
            active.insert(alert.alert_id.clone(), alert);
            metrics::gauge!("alert_active_count").set(active.len() as f64);
        }

        metrics::counter!("alert_triggers_total", "type" => format!("{:?}", alert.alert_type)).increment(1);
        info!(alert_id=%alert.alert_id, "🚨 Alert processed successfully");

        Ok(())
    }

    pub async fn acknowledge_alert(&self, alert_id: &str) -> Result<()> {
        let mut active = self.active_alerts.write().unwrap();
        if let Some(alert) = active.get_mut(alert_id) {
            alert.status = crate::alert::types::AlertStatus::Acknowledged;
            info!(alert_id=%alert_id, "✅ Alert acknowledged");
        }
        Ok(())
    }

    pub async fn resolve_alert(&self, alert_id: &str) -> Result<()> {
        let mut active = self.active_alerts.write().unwrap();
        if active.remove(alert_id).is_some() {
            metrics::gauge!("alert_active_count").set(active.len() as f64);
            info!(alert_id=%alert_id, "✅ Alert resolved");
        }
        Ok(())
    }
}

// Add to StreamEvent
impl StreamEvent {
    pub fn new_alert(alert: crate::alert::types::Alert, device_id: &str) -> Self {
        Self {
            event_id: alert.alert_id.clone(),
            event_type: crate::stream::types::EventType::Alert,
            payload: crate::stream::types::EventPayload::Alert(alert),
            timestamp: chrono::Utc::now().timestamp_nanos() as u64,
            meta crate::stream::types::EventMetadata {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(),
                sequence_number: 0,
                retry_count: 0,
                source_module: "alert".to_string(),
            },
        }
    }
}