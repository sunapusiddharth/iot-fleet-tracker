use crate::health::types::NetworkHealth;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

pub struct BandwidthManager {
    network_health: Arc<RwLock<NetworkHealth>>,
    max_bandwidth_kbps: u32,
}

impl BandwidthManager {
    pub fn new(network_health: Arc<RwLock<NetworkHealth>>, max_bandwidth_kbps: u32) -> Self {
        Self {
            network_health,
            max_bandwidth_kbps,
        }
    }

    pub async fn get_max_download_bandwidth(&self) -> u32 {
        let network = self.network_health.read().await;
        
        // Adjust bandwidth based on network quality
        let mut bandwidth = self.max_bandwidth_kbps;
        
        if network.latency_ms > 200.0 {
            bandwidth = bandwidth / 2;
        }
        
        if network.packet_loss_percent > 5.0 {
            bandwidth = bandwidth / 4;
        }
        
        if network.bandwidth_kbps < bandwidth as f32 {
            bandwidth = network.bandwidth_kbps as u32;
        }
        
        bandwidth.max(50) // Minimum 50 Kbps
    }

    pub async fn should_delay_update(&self, priority: &crate::ota::types::UpdatePriority) -> bool {
        let network = self.network_health.read().await;
        
        match priority {
            crate::ota::types::UpdatePriority::Critical => false,
            crate::ota::types::UpdatePriority::High => network.latency_ms < 500.0,
            crate::ota::types::UpdatePriority::Medium => network.latency_ms < 300.0 && network.packet_loss_percent < 10.0,
            crate::ota::types::UpdatePriority::Low => network.latency_ms < 200.0 && network.packet_loss_percent < 5.0,
        }
    }
}