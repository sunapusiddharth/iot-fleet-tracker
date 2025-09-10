use tokio::sync::broadcast;
use tracing::{info, error};
use crate::models::telemetry::TelemetryData;
use crate::models::alert::Alert;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;

#[derive(Clone)]
pub struct PubSubManager {
    telemetry_tx: broadcast::Sender<TelemetryData>,
    alert_tx: broadcast::Sender<Alert>,
    ml_event_tx: broadcast::Sender<MlEvent>,
    health_status_tx: broadcast::Sender<HealthStatus>,
}

impl PubSubManager {
    pub fn new() -> Self {
        let (telemetry_tx, _) = broadcast::channel(1000);
        let (alert_tx, _) = broadcast::channel(1000);
        let (ml_event_tx, _) = broadcast::channel(1000);
        let (health_status_tx, _) = broadcast::channel(1000);
        
        Self {
            telemetry_tx,
            alert_tx,
            ml_event_tx,
            health_status_tx,
        }
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting Pub/Sub manager");
        
        // In production, you might want to add persistence or other features
        // For now, just return Ok
        Ok(())
    }
    
    pub async fn publish_telemetry(&self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = self.telemetry_tx.send(telemetry.clone()) {
            error!("Failed to publish telemetry: {}", e);
        }
        Ok(())
    }
    
    pub async fn publish_alert(&self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = self.alert_tx.send(alert.clone()) {
            error!("Failed to publish alert: {}", e);
        }
        Ok(())
    }
    
    pub async fn publish_ml_event(&self, ml_event: &MlEvent) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = self.ml_event_tx.send(ml_event.clone()) {
            error!("Failed to publish ML event: {}", e);
        }
        Ok(())
    }
    
    pub async fn publish_health_status(&self, health_status: &HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = self.health_status_tx.send(health_status.clone()) {
            error!("Failed to publish health status: {}", e);
        }
        Ok(())
    }
    
    pub fn subscribe_telemetry(&self) -> broadcast::Receiver<TelemetryData> {
        self.telemetry_tx.subscribe()
    }
    
    pub fn subscribe_alert(&self) -> broadcast::Receiver<Alert> {
        self.alert_tx.subscribe()
    }
    
    pub fn subscribe_ml_event(&self) -> broadcast::Receiver<MlEvent> {
        self.ml_event_tx.subscribe()
    }
    
    pub fn subscribe_health_status(&self) -> broadcast::Receiver<HealthStatus> {
        self.health_status_tx.subscribe()
    }
}