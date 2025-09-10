use crate::wal::types::{WalEntry, EntryPriority};
use crate::wal::error::Result;
use sled::{Db, Tree};
use tracing::{info, warn};

pub struct RetentionManager {
    db: Db,
    tree: Tree,
    config: RetentionConfig,
}

pub struct RetentionConfig {
    pub max_age_hours: u32,
    pub max_size_percent: f32,
    pub min_priority_to_retain: EntryPriority,
}

impl RetentionManager {
    pub fn new(db: Db, tree: Tree, config: RetentionConfig) -> Self {
        Self {
            db,
            tree,
            config,
        }
    }

    pub async fn enforce_retention(&self) -> Result<RetentionStats> {
        let start = std::time::Instant::now();
        let mut entries_deleted = 0;
        let mut bytes_deleted = 0;
        let mut batch = sled::Batch::default();

        let current_time = chrono::Utc::now().timestamp_nanos() as u64;
        let disk_usage_percent = get_disk_usage_percent(); // Implement this

        let mut iter = self.tree.iter();
        let mut current_size = 0;

        while let Some(Ok((key, value))) = iter.next() {
            let seq = u64::from_be_bytes(key.as_ref().try_into().unwrap());
            let entry: WalEntry = deserialize_wal_entry(&value)?;

            let should_delete = !entry.should_retain(current_time, disk_usage_percent) ||
                               (disk_usage_percent > self.config.max_size_percent &&
                                entry.priority > self.config.min_priority_to_retain);

            if should_delete {
                batch.remove(key);
                entries_deleted += 1;
                bytes_deleted += entry.size_bytes as u64;
            }

            current_size += 1;
            if current_size > 1000 {
                self.tree.apply_batch(batch)?;
                batch = sled::Batch::default();
                current_size = 0;
            }
        }

        if !batch.is_empty() {
            self.tree.apply_batch(batch)?;
        }

        self.db.flush()?;

        let stats = RetentionStats {
            entries_deleted,
            bytes_deleted,
            retention_time_ms: start.elapsed().as_millis() as u64,
        };

        info!(entries_deleted=stats.entries_deleted, bytes_deleted=stats.bytes_deleted, "🗑️  Retention policy enforced");
        metrics::counter!("wal_retention_runs_total").increment(1);
        metrics::counter!("wal_entries_deleted_total").increment(stats.entries_deleted);
        metrics::counter!("wal_bytes_deleted_total").increment(stats.bytes_deleted);

        Ok(stats)
    }
}

pub struct RetentionStats {
    pub entries_deleted: u64,
    pub bytes_deleted: u64,
    pub retention_time_ms: u64,
}

fn get_disk_usage_percent() -> f32 {
    // Implement disk usage monitoring
    50.0
}

fn deserialize_wal_entry( &[u8]) -> Result<WalEntry> {
    bincode::deserialize(data).map_err(|e| crate::wal::error::WalError::SerializeError(Box::new(e)))
}