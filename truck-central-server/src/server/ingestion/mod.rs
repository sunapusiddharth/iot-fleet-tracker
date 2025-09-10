use crate::server::config::ServerConfig;
use crate::models::telemetry::TelemetryData;
use crate::models::alert::Alert;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;
use tokio::sync::broadcast;
use tracing::{info, error};

pub mod mqtt;
pub mod http;
pub mod websocket;
pub mod kafka;

pub struct IngestionManager {
    config: ServerConfig,
    tx: broadcast::Sender<IngestionEvent>,
}

pub enum IngestionEvent {
    Telemetry(TelemetryData),
    Alert(Alert),
    MlEvent(MlEvent),
    HealthStatus(HealthStatus),
}

impl IngestionManager {
    pub async fn new(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, _) = broadcast::channel(1000);
        
        Ok(Self {
            config,
            tx,
        })
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting ingestion manager");
        
        // Start MQTT ingestion if configured
        if !self.config.message_queue.mqtt_broker.is_empty() {
            let mqtt_handler = mqtt::MqttIngestionHandler::new(
                &self.config.message_queue.mqtt_broker,
                self.config.message_queue.mqtt_username.clone(),
                self.config.message_queue.mqtt_password.clone(),
            )?;
            
            let tx = self.tx.clone();
            tokio::spawn(async move {
                if let Err(e) = mqtt_handler.start(tx).await {
                    error!("MQTT ingestion failed: {}", e);
                }
            });
        }
        
        // Start Kafka ingestion if configured
        if !self.config.message_queue.kafka_brokers.is_empty() {
            let kafka_handler = kafka::KafkaIngestionHandler::new(
                &self.config.message_queue.kafka_brokers,
                &self.config.message_queue.kafka_topic,
            )?;
            
            let tx = self.tx.clone();
            tokio::spawn(async move {
                if let Err(e) = kafka_handler.start(tx).await {
                    error!("Kafka ingestion failed: {}", e);
                }
            });
        }
        
        // Start HTTP ingestion
        let http_handler = http::HttpIngestionHandler::new(self.config.server.http_port)?;
        let tx = self.tx.clone();
        tokio::spawn(async move {
            if let Err(e) = http_handler.start(tx).await {
                error!("HTTP ingestion failed: {}", e);
            }
        });
        
        // Start WebSocket ingestion
        let websocket_handler = websocket::WebSocketIngestionHandler::new(self.config.server.websocket_port)?;
        let tx = self.tx.clone();
        tokio::spawn(async move {
            if let Err(e) = websocket_handler.start(tx).await {
                error!("WebSocket ingestion failed: {}", e);
            }
        });
        
        Ok(())
    }
    
    pub fn get_receiver(&self) -> broadcast::Receiver<IngestionEvent> {
        self.tx.subscribe()
    }
}