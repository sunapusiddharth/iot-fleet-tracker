use crate::server::config::ServerConfig;
use crate::models::telemetry::TelemetryData;
use crate::models::alert::Alert;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;
use tokio::sync::broadcast;
use tracing::{info, error};

pub mod websocket;
pub mod pubsub;

pub struct RealtimeManager {
    config: ServerConfig,
    websocket_server: websocket::WebSocketServer,
    pubsub_manager: pubsub::PubSubManager,
}

impl RealtimeManager {
    pub async fn new(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let websocket_server = websocket::WebSocketServer::new(config.server.websocket_port)?;
        let pubsub_manager = pubsub::PubSubManager::new();
        
        Ok(Self {
            config,
            websocket_server,
            pubsub_manager,
        })
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting realtime manager");
        
        // Start WebSocket server
        let websocket_server = self.websocket_server.clone();
        tokio::spawn(async move {
            if let Err(e) = websocket_server.start().await {
                error!("WebSocket server failed: {}", e);
            }
        });
        
        // Start Pub/Sub manager
        let pubsub_manager = self.pubsub_manager.clone();
        tokio::spawn(async move {
            if let Err(e) = pubsub_manager.start().await {
                error!("Pub/Sub manager failed: {}", e);
            }
        });
        
        Ok(())
    }
    
    pub async fn broadcast_telemetry(&self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        self.pubsub_manager.publish_telemetry(telemetry).await?;
        Ok(())
    }
    
    pub async fn broadcast_alert(&self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        self.pubsub_manager.publish_alert(alert).await?;
        Ok(())
    }
    
    pub async fn broadcast_ml_event(&self, ml_event: &MlEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.pubsub_manager.publish_ml_event(ml_event).await?;
        Ok(())
    }
    
    pub async fn broadcast_health_status(&self, health_status: &HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
        self.pubsub_manager.publish_health_status(health_status).await?;
        Ok(())
    }
}