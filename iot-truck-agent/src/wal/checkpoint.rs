use crate::wal::types::{WalEntry, CheckpointMarker};
use crate::wal::error::Result;
use sled::{Db, Tree};
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use std::sync::Arc;

pub struct Checkpointer {
    db: Arc<Db>,
    tree: Arc<Tree>,
    interval_sec: u64,
}

impl Checkpointer {
    pub fn new(db: Arc<Db>, tree: Arc<Tree>, interval_sec: u64) -> Self {
        Self {
            db,
            tree,
            interval_sec,
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!(interval_sec=self.interval_sec, "🔄 Starting checkpoint loop");

        loop {
            sleep(Duration::from_secs(self.interval_sec)).await;

            match self.create_checkpoint().await {
                Ok(_) => {
                    info!("✅ Checkpoint created successfully");
                }
                Err(e) => {
                    error!(error=%e, "Checkpoint failed");
                    metrics::counter!("checkpoint_errors_total").increment(1);
                }
            }
        }
    }

    async fn create_checkpoint(&self) -> Result<()> {
        let checkpoint_id = format!("ckpt-{}", chrono::Utc::now().timestamp());
        let timestamp = chrono::Utc::now().timestamp_nanos() as u64;
        let total_entries = self.tree.len() as u64;

        // Find safe deletion point (last acknowledged to server)
        // For now, we'll delete entries older than 1 hour
        let cutoff = timestamp - (3600 * 1_000_000_000); // 1 hour ago

        let mut safe_to_delete_before = 0u64;
        let mut iter = self.tree.iter();
        while let Some(Ok((key, _))) = iter.next() {
            let seq = u64::from_be_bytes(key.as_ref().try_into().unwrap());
            let entry_bytes = self.tree.get(&key)?;
            if let Some(data) = entry_bytes {
                let entry: WalEntry = deserialize_wal_entry(&data)?;
                if entry.timestamp() < cutoff {
                    safe_to_delete_before = seq;
                }
            }
        }

        // Write checkpoint marker
        let marker = WalEntry::Checkpoint(CheckpointMarker {
            checkpoint_id: checkpoint_id.clone(),
            timestamp,
            total_entries,
            safe_to_delete_before,
        });

        let serialized = bincode::serialize(&marker)
            .map_err(|e| WalError::SerializeError(Box::new(e)))?;
        let seq = self.tree.len() as u64;
        self.tree.insert(seq.to_be_bytes(), serialized)?;

        // Compact: delete old entries
        if safe_to_delete_before > 0 {
            let deleted = self.delete_entries_before(safe_to_delete_before).await?;
            info!(deleted, "🗑️  {} old entries deleted", deleted);
        }

        // Flush to disk
        self.db.flush()?;

        metrics::counter!("checkpoints_total").increment(1);
        metrics::gauge!("wal_size_after_checkpoint_bytes").set(self.tree.len() as f64);

        Ok(())
    }

    async fn delete_entries_before(&self, seq: u64) -> Result<u64> {
        let mut deleted = 0u64;
        let mut batch = sled::Batch::default();

        let mut iter = self.tree.iter();
        while let Some(Ok((key, _))) = iter.next() {
            let entry_seq = u64::from_be_bytes(key.as_ref().try_into().unwrap());
            if entry_seq < seq {
                batch.remove(key);
                deleted += 1;
            }
        }

        self.tree.apply_batch(batch)?;
        Ok(deleted)
    }
}

fn deserialize_wal_entry( &[u8]) -> Result<WalEntry> {
    if data.starts_with(b"zstd:") {
        let compressed = base64::decode(&data[5..])?;
        let decompressed = zstd::decode_all(&compressed[..])?;
        bincode::deserialize(&decompressed).map_err(|e| WalError::SerializeError(Box::new(e)))
    } else {
        bincode::deserialize(data).map_err(|e| WalError::SerializeError(Box::new(e)))
    }
}