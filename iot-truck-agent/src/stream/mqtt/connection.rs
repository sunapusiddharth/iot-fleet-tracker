use rumqttc::{Event, EventLoop};
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct ConnectionMonitor {
    eventloop: Arc<EventLoop>,
    is_connected: Arc<AtomicBool>,
    latency_ms: tokio::sync::RwLock<f32>,
    packet_loss: tokio::sync::RwLock<f32>,
}

impl ConnectionMonitor {
    pub fn new(eventloop: Arc<EventLoop>, is_connected: Arc<AtomicBool>) -> Self {
        Self {
            eventloop,
            is_connected,
            latency_ms: tokio::sync::RwLock::new(0.0),
            packet_loss: tokio::sync::RwLock::new(0.0),
        }
    }

    pub async fn start(&self) {
        let mut ping_interval = tokio::time::interval(Duration::from_secs(30));
        let mut event_loop = self.eventloop.clone();

        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    self.send_ping().await;
                }
                result = event_loop.poll() => {
                    match result {
                        Ok(Event::Incoming(Packet::ConnAck(_))) => {
                            self.is_connected.store(true, Ordering::Relaxed);
                            info!("✅ MQTT connected");
                            metrics::gauge!("mqtt_connected").set(1.0);
                        }
                        Ok(Event::Incoming(Packet::Disconnect)) => {
                            self.is_connected.store(false, Ordering::Relaxed);
                            warn!("MQTT disconnected");
                            metrics::gauge!("mqtt_connected").set(0.0);
                        }
                        Ok(Event::Incoming(Packet::PingResp)) => {
                            let latency = chrono::Utc::now().timestamp_millis() as f32 - self.last_ping_time().await;
                            self.set_latency(latency).await;
                        }
                        Err(e) => {
                            error!(error=%e, "MQTT connection error");
                            self.is_connected.store(false, Ordering::Relaxed);
                            metrics::gauge!("mqtt_connected").set(0.0);
                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    async fn send_ping(&self) {
        // In production, send actual ping and measure response time
        let _ = self.eventloop.poll();
        self.set_last_ping_time(chrono::Utc::now().timestamp_millis() as f32).await;
    }

    async fn set_latency(&self, latency: f32) {
        let mut l = self.latency_ms.write().await;
        *l = latency;
    }

    async fn last_ping_time(&self) -> f32 {
        *self.latency_ms.read().await
    }

    async fn set_last_ping_time(&self, time: f32) {
        let mut l = self.latency_ms.write().await;
        *l = time;
    }

    pub async fn get_quality(&self) -> crate::stream::types::NetworkQuality {
        crate::stream::types::NetworkQuality {
            latency_ms: *self.latency_ms.read().await,
            packet_loss_percent: *self.packet_loss.read().await,
            bandwidth_kbps: 1000.0, // Placeholder
        }
    }
}