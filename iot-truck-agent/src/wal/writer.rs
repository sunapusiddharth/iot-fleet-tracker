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
}

impl WalWriter {
    pub fn new(wal_path: &str, max_size_bytes: u64) -> Result<Self> {
        let db = sled::open(wal_path)?;
        let tree = db.open_tree("main")?;

        info!(path=%wal_path, "📂 WAL database opened");

        Ok(Self {
            db: Arc::new(db),
            tree: Arc::new(tree),
            is_closed: Arc::new(AtomicBool::new(false)),
            max_size_bytes,
        })
    }

    // Async write entry — returns sequence number
    pub async fn write_entry(&self, entry: WalEntry) -> Result<u64> {
        if self.is_closed.load(Ordering::Relaxed) {
            return Err(WalError::Closed);
        }

        // Check disk space (approximate)
        if self.tree.len() as u64 > self.max_size_bytes / 100 {
            warn!("WAL approaching max size — triggering checkpoint soon");
        }

        let serialized =
            bincode::serialize(&entry).map_err(|e| WalError::SerializeError(Box::new(e)))?;

        // Compress if large (camera blobs)
        let data = if serialized.len() > 8192 {
            let compressed = zstd::encode_all(&serialized[..], 3)?;
            format!("zstd:{}", base64::encode(compressed)).into_bytes()
        } else {
            serialized
        };

        // Atomic write
        let seq = self.tree.len() as u64;
        self.tree.insert(seq.to_be_bytes(), data)?;

        metrics::counter!("wal_entries_written_total").increment(1);
        metrics::gauge!("wal_size_bytes").set(self.tree.len() as f64);

        Ok(seq)
    }

    pub async fn close(&self) -> Result<()> {
        self.is_closed.store(true, Ordering::Relaxed);
        self.db.flush()?;
        info!("✅ WAL writer closed cleanly");
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.load(Ordering::Relaxed)
    }
}

// Background writer task
pub async fn start_wal_writer(
    wal_path: String,
    max_size_bytes: u64,
    mut rx: mpsc::Receiver<WalEntry>,
) -> Result<()> {
    let writer = WalWriter::new(&wal_path, max_size_bytes)?;

    info!("✍️  WAL writer started");

    while let Some(entry) = rx.recv().await {
        match writer.write_entry(entry).await {
            Ok(seq) => {
                tracing::trace!(seq, "WAL entry written");
            }
            Err(WalError::DiskFull) => {
                error!("Disk full — dropping WAL entry");
                metrics::counter!("wal_dropped_entries_total").increment(1);
                // Trigger emergency checkpoint
                tokio::spawn(async move {
                    // Future: call checkpoint module
                });
            }
            Err(e) => {
                error!(error=%e, "WAL write failed");
                metrics::counter!("wal_write_errors_total").increment(1);
            }
        }
    }

    writer.close().await?;
    Ok(())
}
