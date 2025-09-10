use crate::health::types::{NetworkHealth, AlertInfo, AlertSeverity};
use crate::health::config::HealthConfig;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use rumqttc::AsyncClient;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use tracing::{error, warn};

pub struct NetworkMonitor {
    config: HealthConfig,
    mqtt_client: Option<Arc<AsyncClient>>,
    http_client: HttpClient,
    last_latency: f32,
    last_packets_lost: u32,
}

impl NetworkMonitor {
    pub fn new(config: HealthConfig, mqtt_client: Option<Arc<AsyncClient>>) -> Self {
        Self {
            config,
            mqtt_client,
            http_client: HttpClient::new(),
            last_latency: 0.0,
            last_packets_lost: 0,
        }
    }

    pub async fn collect(&mut self) -> Result<(NetworkHealth, Vec<AlertInfo>), Box<dyn std::error::Error>> {
        let ping_latency = self.ping_host().await;
        let mqtt_connected = self.check_mqtt().await;
        let http_connected = self.check_http().await;
        let bandwidth = self.estimate_bandwidth().await;

        let network = NetworkHealth {
            mqtt_connected,
            http_connected,
            latency_ms: ping_latency,
            last_heartbeat_ack: chrono::Utc::now().timestamp_nanos() as u64, // Placeholder
            packets_lost: self.last_packets_lost,
            bandwidth_kbps: bandwidth,
        };

        let mut alerts = Vec::new();

        if ping_latency > self.config.network.max_latency_ms {
            alerts.push(AlertInfo {
                alert_id: format!("net-latency-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "network_latency_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("Network latency critical: {:.1}ms", ping_latency),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "network_monitor".to_string(),
                recommended_action: "Switch to lower bandwidth mode, reduce frame rate".to_string(),
            });
        } else if ping_latency > self.config.network.max_latency_ms * 0.7 {
            alerts.push(AlertInfo {
                alert_id: format!("net-latency-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "network_latency_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Network latency warning: {:.1}ms", ping_latency),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "network_monitor".to_string(),
                recommended_action: "Monitor network quality".to_string(),
            });
        }

        if !mqtt_connected {
            alerts.push(AlertInfo {
                alert_id: format!("net-mqtt-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "mqtt_disconnected".to_string(),
                severity: AlertSeverity::Critical,
                message: "MQTT broker disconnected".to_string(),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "network_monitor".to_string(),
                recommended_action: "Switch to HTTP fallback, buffer data locally".to_string(),
            });
        }

        if !http_connected {
            alerts.push(AlertInfo {
                alert_id: format!("net-http-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "http_disconnected".to_string(),
                severity: AlertSeverity::Critical,
                message: "HTTP server unreachable".to_string(),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "network_monitor".to_string(),
                recommended_action: "Buffer data locally, retry later".to_string(),
            });
        }

        Ok((network, alerts))
    }

    async fn ping_host(&self) -> f32 {
        let start = std::time::Instant::now();
        match timeout(Duration::from_secs(5), TcpStream::connect(&self.config.network.ping_host)).await {
            Ok(Ok(_)) => {
                let latency = start.elapsed().as_secs_f32() * 1000.0;
                self.last_latency = latency;
                latency
            }
            Ok(Err(_)) => {
                self.last_packets_lost += 1;
                9999.0 // High latency to trigger alert
            }
            Err(_) => {
                self.last_packets_lost += 1;
                9999.0
            }
        }
    }

    async fn check_mqtt(&self) -> bool {
        if let Some(client) = &self.mqtt_client {
            // In production, track connection state via event loop
            true // Placeholder
        } else {
            false
        }
    }

    async fn check_http(&self) -> bool {
        match self.http_client
            .get(&format!("https://{}/health", self.config.network.ping_host))
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    async fn estimate_bandwidth(&self) -> f32 {
        // Simplified — in production, measure actual throughput
        1000.0 // 1 Mbps
    }
}