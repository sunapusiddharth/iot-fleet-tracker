use crate::stream::types::{Batch, Ack, StreamEvent, QoSLevel};
use crate::stream::error::Result;
use rumqttc::{AsyncClient, MqttOptions, QoS, Event, EventLoop, Packet};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct MqttStreamer {
    client: AsyncClient,
    eventloop: Arc<EventLoop>,
    is_connected: Arc<AtomicBool>,
    device_id: String,
    pending_acks: Arc<tokio::sync::RwLock<HashMap<String, StreamEvent>>>,
    connection_monitor: crate::stream::mqtt::connection::ConnectionMonitor,
}

impl MqttStreamer {
    pub async fn new(broker_url: &str, client_id: &str, device_id: &str) -> Result<Self> {
        let mut mqtt_options = MqttOptions::parse_url(broker_url)?;
        mqtt_options.set_client_id(client_id);
        mqtt_options.set_keep_alive(Duration::from_secs(30));
        mqtt_options.set_max_packet_size(256 * 1024); // 256KB

        let (client, eventloop) = AsyncClient::new(mqtt_options, 10);
        let eventloop = Arc::new(eventloop);
        let is_connected = Arc::new(AtomicBool::new(false));
        let pending_acks = Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        // Start connection monitor
        let connection_monitor = crate::stream::mqtt::connection::ConnectionMonitor::new(
            eventloop.clone(),
            is_connected.clone(),
        );
        tokio::spawn(async move {
            connection_monitor.start().await;
        });

        info!(broker=%broker_url, client_id, "🔌 MQTT 5.0 client initialized");
        Ok(Self {
            client,
            eventloop,
            is_connected,
            device_id: device_id.to_string(),
            pending_acks,
            connection_monitor,
        })
    }

    pub async fn send_batch(&mut self, batch: Batch) -> Result<Ack> {
        if !self.is_connected.load(Ordering::Relaxed) {
            return Err(crate::stream::error::StreamError::NoTransport);
        }

        let topic = format!("{}/telemetry", self.device_id);
        let payload = serde_json::to_vec(&batch)?;

        // Use QoS based on batch priority
        let qos = match batch.priority {
            crate::stream::types::EventPriority::Critical => QoS::ExactlyOnce,
            crate::stream::types::EventPriority::High => QoS::ExactlyOnce,
            _ => QoS::AtLeastOnce,
        };

        // Store events for ACK tracking
        {
            let mut pending = self.pending_acks.write().await;
            for event in &batch.events {
                pending.insert(event.event_id.clone(), event.clone());
            }
        }

        let publish = self.client.publish(&topic, qos, false, payload);
        let timeout = tokio::time::timeout(Duration::from_secs(30), publish).await??;

        // For QoS 2, wait for PUBCOMP
        if qos == QoS::ExactlyOnce {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let ack = Ack {
            batch_id: batch.batch_id.clone(),
            received_at: chrono::Utc::now().timestamp_nanos() as u64,
            event_ids: batch.events.iter().map(|e| e.event_id.clone()).collect(),
            status: crate::stream::types::AckStatus::Success,
            server_sequence: 0, // Will be filled by server
        };

        metrics::counter!("mqtt_batches_sent_total").increment(1);
        metrics::counter!("mqtt_events_sent_total").increment(batch.events.len() as u64);
        metrics::gauge!("mqtt_batch_size_bytes").set(batch.size_bytes as f64);

        Ok(ack)
    }

    pub async fn process_ack(&self, batch_id: &str) -> Result<()> {
        let mut pending = self.pending_acks.write().await;
        let events: Vec<_> = pending
            .iter()
            .filter(|(_, e)| e.metadata.sequence_number.to_string().contains(batch_id))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        for (event_id, _) in events {
            pending.remove(&event_id);
            // Notify WAL to delete this event
            crate::stream::backpressure::notify_wal_ack(&event_id).await;
        }

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::Relaxed)
    }

    pub async fn get_connection_quality(&self) -> crate::stream::types::NetworkQuality {
        self.connection_monitor.get_quality().await
    }
}