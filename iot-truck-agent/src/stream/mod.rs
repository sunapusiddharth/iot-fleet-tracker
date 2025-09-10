use crate::config::Config;
use crate::stream::backpressure;
use crate::stream::batcher::IntelligentBatcher;
use crate::stream::compressor::AdaptiveCompressor;
use crate::stream::http::HttpStreamer;
use crate::stream::mqtt::MqttStreamer;
use crate::stream::types::{Ack, Batch, StreamEvent};
use crate::wal::WalManager;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

pub mod auth;
pub mod backpressure;
pub mod batcher;
pub mod compressor;
pub mod deduplicator;
pub mod error;
pub mod http;
pub mod monitor;
pub mod mqtt;
pub mod types;

// Metrics
metrics::describe_counter!("batches_created_total", "Total batches created");
metrics::describe_gauge!("batch_size_bytes", "Current batch size in bytes");
metrics::describe_gauge!("batch_compression_ratio", "Compression ratio of batches");
metrics::describe_counter!("mqtt_batches_sent_total", "MQTT batches sent");
metrics::describe_counter!("mqtt_events_sent_total", "MQTT events sent");
metrics::describe_gauge!("mqtt_connected", "MQTT connection status");
metrics::describe_counter!("http_batches_sent_total", "HTTP batches sent");
metrics::describe_counter!("http_events_sent_total", "HTTP events sent");
metrics::describe_counter!("stream_retries_total", "Total retries");
metrics::describe_counter!("stream_errors_total", "Total stream errors");
metrics::describe_gauge!("network_latency_ms", "Network latency in ms");
metrics::describe_gauge!("network_packet_loss_percent", "Network packet loss percent");

pub struct StreamManager {
    batch_rx: mpsc::Receiver<Batch>,
    wal_manager: WalManager,
    device_id: String,
    mqtt_streamer: MqttStreamer,
    http_streamer: HttpStreamer,
    network_monitor: crate::stream::monitor::NetworkMonitor,
}

impl StreamManager {
    pub async fn new(
        config: &Config,
        wal_manager: WalManager,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (event_tx, event_rx) = mpsc::channel(1000);
        let (batch_tx, batch_rx) = mpsc::channel(100);

        // Initialize WAL for backpressure
        backpressure::init_wal_manager(wal_manager.clone()).await;

        // Start batcher
        let batcher = IntelligentBatcher::new(
            256 * 1024, // 256KB max batch
            100,        // 100 events max
            1000,       // 1s timeout
        );
        tokio::spawn(async move {
            if let Err(e) = batcher.start_batcher(event_rx, batch_tx).await {
                error!(error=%e, "Batcher crashed");
            }
        });

        // Start MQTT client
        let mut mqtt_streamer = MqttStreamer::new(
            &config.mqtt.broker_url,
            &config.mqtt.client_id,
            &config.device_id,
        )
        .await?;

        // Start HTTP fallback
        let http_streamer = HttpStreamer::new(
            &format!(
                "https://api.yourcompany.com/v1/telemetry/{}",
                config.device_id
            ),
            30,
        );

        // Start network monitor
        let network_monitor =
            crate::stream::monitor::NetworkMonitor::new(mqtt_streamer.get_connection_quality());

        info!("✅ Stream manager initialized with intelligent batching and compression");

        Ok(Self {
            batch_rx,
            wal_manager,
            device_id: config.device_id.clone(),
            mqtt_streamer,
            http_streamer,
            network_monitor,
        })
    }

    pub async fn start_streaming_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut pending_batches = std::collections::VecDeque::new();

        loop {
            tokio::select! {
                Some(batch) = self.batch_rx.recv() => {
                    pending_batches.push_back(batch);
                }
                _ = sleep(Duration::from_secs(1)) => {
                    if !pending_batches.is_empty() {
                        let batch = pending_batches.pop_front().unwrap();
                        let result = self.send_with_retry_and_compress(batch).await;

                        match result {
                            Ok(ack) => {
                                info!(batch_id=%ack.batch_id, "✅ Batch sent successfully");
                                // Process ACK
                                self.mqtt_streamer.process_ack(&ack.batch_id).await?;
                            }
                            Err(e) => {
                                error!(error=%e, "❌ Failed to send batch — requeuing");
                                pending_batches.push_front(batch); // retry later
                                metrics::counter!("stream_errors_total").increment(1);
                            }
                        }
                    }
                }
            }
        }
    }

    async fn send_with_retry_and_compress(
        &self,
        mut batch: Batch,
    ) -> Result<Ack, Box<dyn std::error::Error>> {
        let network_quality = self.network_monitor.get_quality().await;

        // Compress events based on network quality
        for event in &mut batch.events {
            AdaptiveCompressor::compress_event(event, &network_quality)?;
        }

        let mut retry_count = 0;
        let max_retries = 5;

        loop {
            // Try MQTT first
            if self.mqtt_streamer.is_connected() {
                match self.mqtt_streamer.send_batch(batch.clone()).await {
                    Ok(ack) => {
                        metrics::counter!("stream_retries_total").increment(retry_count as u64);
                        return Ok(ack);
                    }
                    Err(e) => {
                        warn!(error=%e, "MQTT send failed — trying HTTP fallback");
                    }
                }
            }

            // Fallback to HTTP
            match self.http_streamer.send_batch(batch.clone()).await {
                Ok(ack) => {
                    metrics::counter!("stream_retries_total").increment(retry_count as u64);
                    return Ok(ack);
                }
                Err(e) => {
                    error!(error=%e, "HTTP send failed");
                }
            }

            // Retry with backoff
            if retry_count >= max_retries {
                // Buffer to WAL
                for event in batch.events {
                    if let Err(e) = backpressure::buffer_to_wal(event).await {
                        error!(error=%e, "Failed to buffer event to WAL");
                    }
                }
                return Err("Max retries exceeded — buffered to WAL".into());
            }

            retry_count += 1;
            let delay = Duration::from_secs(2u64.pow(retry_count));
            warn!(retry_count, delay=?delay, "⏳ Retrying in {:?}", delay);
            tokio::time::sleep(delay).await;
        }
    }

    pub async fn send_event(&self, event: StreamEvent) -> Result<(), Box<dyn std::error::Error>> {
        // Send to batcher
        // In production, we'd have a channel to the batcher
        // For now, just log
        info!(event_id=%event.event_id, priority=?event.priority, "📤 Queued event for streaming");
        Ok(())
    }
}
