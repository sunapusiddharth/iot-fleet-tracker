use rumqttc::{AsyncClient, MqttOptions, QoS, Event, EventLoop};
use tokio::time::{sleep, Duration};
use tracing::{info, error};
use crate::server::ingestion::IngestionEvent;
use tokio::sync::broadcast;
use std::collections::HashMap;

pub struct MqttIngestionHandler {
    broker: String,
    username: Option<String>,
    password: Option<String>,
}

impl MqttIngestionHandler {
    pub fn new(broker: &str, username: Option<String>, password: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            broker: broker.to_string(),
            username,
            password,
        })
    }
    
    pub async fn start(&self, tx: broadcast::Sender<IngestionEvent>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting MQTT ingestion from {}", self.broker);
        
        let mut mqtt_options = MqttOptions::parse_url(&self.broker)?;
        mqtt_options.set_client_id(format!("truck-central-server-{}", uuid::Uuid::new_v4()));
        mqtt_options.set_keep_alive(Duration::from_secs(30));
        
        if let Some(username) = &self.username {
            mqtt_options.set_credentials(username, self.password.as_deref().unwrap_or(""));
        }
        
        let (client, eventloop) = AsyncClient::new(mqtt_options, 10);
        
        // Subscribe to topics
        client.subscribe("truck/+/telemetry", QoS::AtLeastOnce).await?;
        client.subscribe("truck/+/alert", QoS::AtLeastOnce).await?;
        client.subscribe("truck/+/ml", QoS::AtLeastOnce).await?;
        client.subscribe("truck/+/health", QoS::AtLeastOnce).await?;
        
        // Start event loop
        tokio::spawn(async move {
            let mut event_loop = event_loop;
            loop {
                match event_loop.poll().await {
                    Ok(Event::Incoming(rumqttc::Packet::Publish(p))) => {
                        let topic = p.topic;
                        let payload = p.payload;
                        
                        // Parse topic to get truck ID
                        let parts: Vec<&str> = topic.split('/').collect();
                        if parts.len() >= 3 {
                            let truck_id_str = parts[1];
                            let event_type = parts[2];
                            
                            match event_type {
                                "telemetry" => {
                                    if let Ok(telemetry) = serde_json::from_slice::<crate::models::telemetry::TelemetryData>(&payload) {
                                        if let Err(e) = tx.send(IngestionEvent::Telemetry(telemetry)) {
                                            error!("Failed to send telemetry event: {}", e);
                                        }
                                    }
                                }
                                "alert" => {
                                    if let Ok(alert) = serde_json::from_slice::<crate::models::alert::Alert>(&payload) {
                                        if let Err(e) = tx.send(IngestionEvent::Alert(alert)) {
                                            error!("Failed to send alert event: {}", e);
                                        }
                                    }
                                }
                                "ml" => {
                                    if let Ok(ml_event) = serde_json::from_slice::<crate::models::ml::MlEvent>(&payload) {
                                        if let Err(e) = tx.send(IngestionEvent::MlEvent(ml_event)) {
                                            error!("Failed to send ML event: {}", e);
                                        }
                                    }
                                }
                                "health" => {
                                    if let Ok(health_status) = serde_json::from_slice::<crate::models::health::HealthStatus>(&payload) {
                                        if let Err(e) = tx.send(IngestionEvent::HealthStatus(health_status)) {
                                            error!("Failed to send health status: {}", e);
                                        }
                                    }
                                }
                                _ => {
                                    error!("Unknown event type: {}", event_type);
                                }
                            }
                        }
                    }
                    Ok(Event::Incoming(rumqttc::Packet::ConnAck(_))) => {
                        info!("✅ MQTT connected");
                    }
                    Ok(Event::Incoming(rumqttc::Packet::Disconnect)) => {
                        error!("MQTT disconnected");
                    }
                    Err(e) => {
                        error!("MQTT error: {}", e);
                        sleep(Duration::from_secs(5)).await;
                    }
                    _ => {}
                }
            }
        });
        
        Ok(())
    }
}