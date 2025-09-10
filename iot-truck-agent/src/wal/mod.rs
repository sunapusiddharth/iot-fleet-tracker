use crate::wal::error::Result;
use crate::wal::types::WalEntry;
use sled::{Db, Tree};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub struct WalWriter {
    db: Arc<Db>,
    tree: Arc<Tree>,
    is_closed: Arc<AtomicBool>,
    max_size_bytes: u64,
    buffer: crate::wal::writer::buffer::WriteBuffer,
    encryptor: Option<crate::wal::writer::encryption::DataEncryptor>,
}

impl WalWriter {
    pub fn new(wal_path: &str, max_size_bytes: u64, enable_encryption: bool) -> Result<Self> {
        let db = sled::open(wal_path)?;
        let tree = db.open_tree("main")?;

        let buffer = crate::wal::writer::buffer::WriteBuffer::new(1024 * 1024); // 1MB buffer
        let encryptor = if enable_encryption {
            Some(crate::wal::writer::encryption::DataEncryptor::new()?)
        } else {
            None
        };

        info!(path=%wal_path, "📂 WAL database opened with encryption: {}", enable_encryption);

        Ok(Self {
            db: Arc::new(db),
            tree: Arc::new(tree),
            is_closed: Arc::new(AtomicBool::new(false)),
            max_size_bytes,
            buffer,
            encryptor,
        })
    }

    pub async fn write_entry(&self, entry: WalEntry) -> Result<u64> {
        if self.is_closed.load(Ordering::Relaxed) {
            return Err(crate::wal::error::WalError::Closed);
        }

        // Check disk space
        if self.tree.len() as u64 > self.max_size_bytes * 9 / 10 {
            warn!("WAL approaching max size — triggering retention");
            // In production, trigger retention policy
        }

        // Compress if needed
        let mut entry = self.compress_entry(entry)?;

        // Encrypt if enabled
        if let Some(encryptor) = &self.encryptor {
            entry = encryptor.encrypt_entry(entry)?;
        }

        // Buffer write
        let seq = self.buffer.write_entry(entry.clone()).await?;

        // Flush to disk if buffer is full or entry is critical
        if self.buffer.should_flush() || entry.is_critical() {
            self.flush_buffer().await?;
        }

        metrics::counter!("wal_entries_written_total").increment(1);
        metrics::gauge!("wal_size_bytes").set(self.tree.len() as f64);
        metrics::gauge!("wal_buffer_size_bytes").set(self.buffer.size() as f64);

        Ok(seq)
    }

    async fn flush_buffer(&self) -> Result<()> {
        let entries = self.buffer.flush().await?;
        for (seq, entry) in entries {
            let serialized = bincode::serialize(&entry)
                .map_err(|e| crate::wal::error::WalError::SerializeError(Box::new(e)))?;

            self.tree.insert(seq.to_be_bytes(), serialized)?;
        }
        self.db.flush()?;
        Ok(())
    }

    fn compress_entry(&self, mut entry: WalEntry) -> Result<WalEntry> {
        // Compress based on type and size
        match &mut entry.payload {
            crate::wal::types::EntryPayload::Sensor(_) => {
                // Use delta encoding for sensor data (future)
            }
            crate::wal::types::EntryPayload::CameraBlob { data, .. } => {
                // Don't compress already compressed video
                if entry.compression.algorithm == "none" && data.len() > 8192 {
                    let compressed = zstd::encode_all(data, 3)?;
                    *data = compressed;
                    entry.compression = crate::wal::types::CompressionInfo {
                        algorithm: "zstd".to_string(),
                        level: 3,
                        original_size: entry.compression.original_size,
                        compressed_size: data.len(),
                    };
                    entry.size_bytes = data.len();
                }
            }
            _ => {
                // Compress other types if large
                if entry.size_bytes > 4096 {
                    let json = serde_json::to_vec(&entry.payload)?;
                    let compressed = zstd::encode_all(&json[..], 3)?;
                    // Can't easily replace payload, so skip for now
                }
            }
        }
        Ok(entry)
    }

    pub async fn close(&self) -> Result<()> {
        self.flush_buffer().await?;
        self.is_closed.store(true, Ordering::Relaxed);
        self.db.flush()?;
        info!("✅ WAL writer closed cleanly");
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.load(Ordering::Relaxed)
    }
}
use crate::config::Config;
use crate::health::types::ResourceUsage;
use crate::wal::ack_manager::AckManager;
use crate::wal::compactor::Compactor;
use crate::wal::health_integration::HealthIntegration;
use crate::wal::reader::WalReader;
use crate::wal::retention::RetentionManager;
use crate::wal::types::WalEntry;
use crate::wal::writer::WalWriter;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub mod ack_manager;
pub mod cli;
pub mod compactor;
pub mod error;
pub mod health_integration;
pub mod reader;
pub mod retention;
pub mod types;
pub mod writer;

// Metrics
metrics::describe_counter!("wal_entries_written_total", "Total WAL entries written");
metrics::describe_counter!("wal_dropped_entries_total", "Entries dropped due to errors");
metrics::describe_counter!("wal_write_errors_total", "WAL write errors");
metrics::describe_gauge!("wal_size_bytes", "Current WAL size in bytes");
metrics::describe_gauge!("wal_buffer_size_bytes", "Current WAL buffer size in bytes");
metrics::describe_counter!("checkpoints_total", "Total checkpoints created");
metrics::describe_counter!("checkpoint_errors_total", "Checkpoint errors");
metrics::describe_gauge!(
    "wal_size_after_checkpoint_bytes",
    "WAL size after last checkpoint"
);
metrics::describe_counter!("wal_entries_replayed_total", "Entries replayed on startup");
metrics::describe_counter!("wal_compactions_total", "Total compactions");
metrics::describe_counter!("wal_entries_compacted_total", "Entries compacted");
metrics::describe_counter!("wal_bytes_saved_total", "Bytes saved by compaction");
metrics::describe_counter!("wal_retention_runs_total", "Total retention runs");
metrics::describe_counter!("wal_entries_deleted_total", "Entries deleted by retention");
metrics::describe_counter!("wal_bytes_deleted_total", "Bytes deleted by retention");
metrics::describe_counter!("wal_events_acked_total", "Events acknowledged");
metrics::describe_gauge!("wal_throttled", "WAL writes throttled due to health");

pub struct WalManager {
    writer: WalWriter,
    reader: WalReader,
    compactor: Compactor,
    retention_manager: RetentionManager,
    ack_manager: AckManager,
    health_integration: HealthIntegration,
    tx: mpsc::Sender<WalEntry>,
    device_id: String,
}

impl WalManager {
    pub async fn new(
        config: &Config,
        resource_usage: std::sync::Arc<tokio::sync::RwLock<ResourceUsage>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel(1000);

        let wal_path = &config.storage.wal_path;
        let max_size_bytes = config.storage.max_wal_size_mb * 1024 * 1024;
        let enable_encryption = false; // Set from config

        // Create WAL writer
        let writer = WalWriter::new(wal_path, max_size_bytes, enable_encryption)?;

        // Create WAL reader
        let db_reader = sled::open(wal_path)?;
        let reader = WalReader::new(&db_reader, enable_encryption)?;

        // Create compactor
        let db_compact = sled::open(wal_path)?;
        let tree_compact = db_compact.open_tree("main")?;
        let compactor = Compactor::new(
            db_compact,
            tree_compact,
            config.storage.checkpoint_interval_sec,
        );

        // Create retention manager
        let db_retention = sled::open(wal_path)?;
        let tree_retention = db_retention.open_tree("main")?;
        let retention_manager = RetentionManager::new(
            db_retention,
            tree_retention,
            crate::wal::retention::RetentionConfig {
                max_age_hours: 72,
                max_size_percent: 90.0,
                min_priority_to_retain: crate::wal::types::EntryPriority::Medium,
            },
        );

        // Create ACK manager
        let db_ack = sled::open(wal_path)?;
        let ack_manager = AckManager::new(db_ack)?;

        // Create health integration
        let health_integration = HealthIntegration::new(resource_usage.clone());

        // Start WAL writer task
        tokio::spawn(async move {
            let db_writer = sled::open(wal_path).unwrap();
            let tree_writer = db_writer.open_tree("main").unwrap();
            let compactor = Compactor::new(db_writer, tree_writer, 300);
            if let Err(e) = compactor.start().await {
                error!(error=%e, "Compactor crashed");
            }
        });

        // Start retention task
        let retention_manager_clone = retention_manager.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = retention_manager_clone.enforce_retention().await {
                    error!(error=%e, "Retention failed");
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await; // Run hourly
            }
        });

        // Replay on startup
        let last_seq = reader.last_sequence()?;
        if last_seq > 0 {
            info!(last_seq, "🔍 Found existing WAL — replaying...");
            let entries = reader.replay_from(0)?;
            info!(
                count = entries.len(),
                "🔁 Replayed {} entries",
                entries.len()
            );
        }

        info!(
            "✅ WAL manager initialized with encryption: {}",
            enable_encryption
        );

        Ok(Self {
            writer,
            reader,
            compactor,
            retention_manager,
            ack_manager,
            health_integration,
            tx,
            device_id: config.device_id.clone(),
        })
    }

    pub async fn write_sensor(
        &self,
        event: crate::sensors::types::SensorEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.health_integration.should_throttle_writes().await {
            return Err("WAL writes throttled due to system health".into());
        }

        let entry = WalEntry::new_sensor(
            event,
            &self.device_id,
            chrono::Utc::now().timestamp_nanos() as u64,
        );
        if self.tx.send(entry).await.is_err() {
            return Err("WAL channel closed".into());
        }
        Ok(())
    }

    // ... other write methods

    pub async fn mark_acked(&self, event_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.ack_manager.mark_acked(event_id).await
    }

    pub async fn is_acked(&self, event_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        self.ack_manager.is_acked(event_id).await
    }
}
