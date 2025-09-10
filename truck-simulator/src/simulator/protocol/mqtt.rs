use crate::simulator::types::TruckState;
use tokio::sync::broadcast;
use tracing::{info, error};
use rumqttd::Broker;
use rumqttd::Config;
use std::net::SocketAddr;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MqttHandler {
    port: u16,
}

impl MqttHandler {
    pub async fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            port,
        })
    }
    
    pub async fn start(&self, mut rx: broadcast::Receiver<TruckState>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting MQTT server on port {}", self.port);
        
        // Create rumqttd config
        let mut config = Config::default();
        config.v4.listen = vec![SocketAddr::from(([0, 0, 0, 0], self.port))];
        
        // Start broker
        let broker = Broker::new(config);
        
        // Start broker in separate task
        tokio::spawn(async move {
            if let Err(e) = broker.start().await {
                error!("MQTT broker failed: {}", e);
            }
        });
        
        // Start publishing task
        while let Ok(state) = rx.recv().await {
            // Publish to MQTT topics
            let client_id = format!("simulator-{}", state.truck_id);
            let topic = format!("truck/{}/telemetry", state.truck_id);
            
            // Convert state to JSON
            let payload = serde_json::to_vec(&state)?;
            
            // In production, use rumqttc to publish
            // For now, just log
            info!(truck_id=%state.truck_id, "📤 Published to MQTT topic: {}", topic);
            
            // Add delay to simulate network
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        
        Ok(())
    }
}