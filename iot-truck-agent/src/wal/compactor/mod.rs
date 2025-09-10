use crate::wal::types::WalEntry;
use crate::wal::error::Result;
use sled::{Db, Tree};
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

pub struct Compactor {
    db: Db,
    tree: Tree,
    interval_sec: u64,
}

impl Compactor {
    pub fn new(db: Db, tree: Tree, interval_sec: u64) -> Self {
        Self {
            db,
            tree,
            interval_sec,
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!(interval_sec=self.interval_sec, "🔄 Starting compaction loop");

        loop {
            sleep(Duration::from_secs(self.interval_sec)).await;

            match self.compact().await {
                Ok(stats) => {
                    info!(entries_compacted=stats.entries_compacted, bytes_saved=stats.bytes_saved, "✅ Compaction completed");
                    metrics::counter!("wal_compactions_total").increment(1);
                    metrics::counter!("wal_entries_compacted_total").increment(stats.entries_compacted);
                    metrics::counter!("wal_bytes_saved_total").increment(stats.bytes_saved);
                }
                Err(e) => {
                    error!(error=%e, "Compaction failed");
                    metrics::counter!("wal_compaction_errors_total").increment(1);
                }
            }
        }
    }

    async fn compact(&self) -> Result<crate::wal::types::CompactionStats> {
        let start = std::time::Instant::now();
        let mut entries_compacted = 0;
        let mut bytes_saved = 0;
        let mut batch = sled::Batch::default();

        let mut iter = self.tree.iter();
        let mut current_size = 0;

        while let Some(Ok((key, value))) = iter.next() {
            let seq = u64::from_be_bytes(key.as_ref().try_into().unwrap());
            let entry: WalEntry = deserialize_wal_entry(&value)?;

            // Skip if already acked and old
            if entry.metadata.acked && entry.should_retain(chrono::Utc::now().timestamp_nanos() as u64, 80.0) {
                batch.remove(key);
                entries_compacted += 1;
                bytes_saved += entry.size_bytes as u64;
            }

            current_size += 1;
            if current_size > 1000 { // Process in batches
                self.tree.apply_batch(batch)?;
                batch = sled::Batch::default();
                current_size = 0;
            }
        }

        if !batch.is_empty() {
            self.tree.apply_batch(batch)?;
        }

        self.db.flush()?;

        Ok(crate::wal::types::CompactionStats {
            entries_compacted,
            bytes_saved,
            compaction_time_ms: start.elapsed().as_millis() as u64,
        })
    }
}

fn deserialize_wal_entry( &[u8]) -> Result<WalEntry> {
    bincode::deserialize(data).map_err(|e| crate::wal::error::WalError::SerializeError(Box::new(e)))
}