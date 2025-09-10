use crate::wal::types::WalEntry;
use sled::{Db, Tree};
use tracing::{info, warn};
use std::collections::HashSet;

pub struct AckManager {
    db: Db,
    acked_tree: Tree,
    pending_acks: std::sync::RwLock<HashSet<String>>,
}

impl AckManager {
    pub fn new(db: Db) -> Result<Self, Box<dyn std::error::Error>> {
        let acked_tree = db.open_tree("acked")?;
        Ok(Self {
            db,
            acked_tree,
            pending_acks: std::sync::RwLock::new(HashSet::new()),
        })
    }

    pub async fn mark_acked(&self, event_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut pending = self.pending_acks.write().unwrap();
        pending.insert(event_id.to_string());

        // Also store in WAL for persistence
        self.acked_tree.insert(event_id, b"1")?;
        self.db.flush()?;

        info!(event_id, "✅ Event marked as acknowledged");
        metrics::counter!("wal_events_acked_total").increment(1);

        Ok(())
    }

    pub async fn is_acked(&self, event_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let pending = self.pending_acks.read().unwrap();
        if pending.contains(event_id) {
            return Ok(true);
        }

        Ok(self.acked_tree.contains_key(event_id)?)
    }

    pub async fn get_pending_acks(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let pending = self.pending_acks.read().unwrap();
        Ok(pending.iter().cloned().collect())
    }
}