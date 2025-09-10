use rdkafka::consumer::{StreamConsumer, Consumer, CommitMode};
use rdkafka::config::ClientConfig;
use rdkafka::message::Message;
use tracing::{info, error};
use crate::server::ingestion::IngestionEvent;
use tokio::sync::broadcast;
use std::time::Duration;

pub struct KafkaIngestionHandler {
    brokers: Vec<String>,
    topic: String,
}

impl KafkaIngestionHandler {
    pub fn new(brokers: &[String], topic: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            brokers: brokers.to_vec(),
            topic: topic.to_string(),
        })
    }
    
    pub async fn start(&self, tx: broadcast::Sender<IngestionEvent>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting Kafka ingestion from topic {}", self.topic);
        
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "truck-central-server")
            .set("bootstrap.servers", &self.brokers.join(","))
            .set("enable.auto.commit", "false")
            .set("session.timeout.ms", "6000")
            .set("auto.offset.reset", "earliest")
            .create()?;
        
        consumer.subscribe(&[&self.topic])?;
        
        tokio::spawn(async move {
            loop {
                match consumer.recv().await {
                    Ok(message) => {
                        if let Some(payload) = message.payload() {
                            // Determine event type from topic or headers
                            let topic = message.topic();
                            
                            match topic {
                                topic if topic.ends_with("telemetry") => {
                                    if let Ok(telemetry) = serde_json::from_slice::<crate::models::telemetry::TelemetryData>(payload) {
                                        if let Err(e) = tx.send(IngestionEvent::Telemetry(telemetry)) {
                                            error!("Failed to send telemetry event: {}", e);
                                        }
                                    }
                                }
                                topic if topic.ends_with("alert") => {
                                    if let Ok(alert) = serde_json::from_slice::<crate::models::alert::Alert>(payload) {
                                        if let Err(e) = tx.send(IngestionEvent::Alert(alert)) {
                                            error!("Failed to send alert event: {}", e);
                                        }
                                    }
                                }
                                topic if topic.ends_with("ml") => {
                                    if let Ok(ml_event) = serde_json::from_slice::<crate::models::ml::MlEvent>(payload) {
                                        if let Err(e) = tx.send(IngestionEvent::MlEvent(ml_event)) {
                                            error!("Failed to send ML event: {}", e);
                                        }
                                    }
                                }
                                topic if topic.ends_with("health") => {
                                    if let Ok(health_status) = serde_json::from_slice::<crate::models::health::HealthStatus>(payload) {
                                        if let Err(e) = tx.send(IngestionEvent::HealthStatus(health_status)) {
                                            error!("Failed to send health status: {}", e);
                                        }
                                    }
                                }
                                _ => {
                                    error!("Unknown topic: {}", topic);
                                }
                            }
                            
                            // Manually commit offset
                            if let Err(e) = consumer.commit_message(&message, CommitMode::Async) {
                                error!("Failed to commit offset: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Kafka error: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });
        
        Ok(())
    }
}