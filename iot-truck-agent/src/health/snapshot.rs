use crate::health::types::HealthEvent;
use serde_json;
use sled::Db;
use tracing::{error, info};

pub struct HealthSnapshotter {
    db: Db,
}

impl HealthSnapshotter {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = sled::open(db_path)?;
        Ok(Self { db })
    }

    pub fn save_snapshot(
        &self,
        health_event: &HealthEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("snapshot-{}", health_event.timestamp);
        let serialized = serde_json::to_vec(health_event)?;

        self.db.insert(key, serialized)?;
        self.db.flush()?;

        info!(timestamp=%health_event.timestamp, "💾 Health snapshot saved");

        // Keep only last 100 snapshots
        let mut iter = self.db.iter();
        let keys: Vec<_> = iter.keys().filter_map(|k| k.ok()).collect();
        if keys.len() > 100 {
            for key in keys.iter().take(keys.len() - 100) {
                self.db.remove(key)?;
            }
            self.db.flush()?;
        }

        Ok(())
    }

    pub fn get_latest_snapshot(&self) -> Result<Option<HealthEvent>, Box<dyn std::error::Error>> {
        let mut iter = self.db.iter().rev();
        if let Some(Ok((_, value))) = iter.next() {
            let health_event: HealthEvent = serde_json::from_slice(&value)?;
            Ok(Some(health_event))
        } else {
            Ok(None)
        }
    }

    pub fn get_snapshots_since(
        &self,
        timestamp: u64,
    ) -> Result<Vec<HealthEvent>, Box<dyn std::error::Error>> {
        let mut snapshots = Vec::new();
        let start_key = format!("snapshot-{}", timestamp);
        let mut iter = self.db.range(start_key..);

        while let Some(Ok((_, value))) = iter.next() {
            let health_event: HealthEvent = serde_json::from_slice(&value)?;
            snapshots.push(health_event);
        }

        Ok(snapshots)
    }
}
