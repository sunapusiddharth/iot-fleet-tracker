use crate::wal::types::WalEntry;
use crate::wal::error::Result;
use sled::{Db, Tree};
use tracing::{info, warn};
use std::collections::VecDeque;

pub struct WalReader {
    tree: Tree,
}

impl WalReader {
    pub fn new(db: &Db) -> Result<Self> {
        let tree = db.open_tree("main")?;
        Ok(Self { tree })
    }

    // Replay all entries from sequence number
    pub fn replay_from(&self, start_seq: u64) -> Result<VecDeque<WalEntry>> {
        let mut entries = VecDeque::new();
        let mut iter = self.tree.range(start_seq.to_be_bytes()..);

        while let Some(Ok((key, value))) = iter.next() {
            let seq = u64::from_be_bytes(key.as_ref().try_into().unwrap());
            let entry = deserialize_wal_entry(&value)?;
            entries.push_back(entry);
            metrics::counter!("wal_entries_replayed_total").increment(1);
        }

        info!(start_seq, count=entries.len(), "🔁 Replayed {} WAL entries", entries.len());
        Ok(entries)
    }

    // Get last sequence number
    pub fn last_sequence(&self) -> Result<u64> {
        let last = self.tree.iter().rev().next();
        match last {
            Some(Ok((key, _))) => {
                let seq = u64::from_be_bytes(key.as_ref().try_into().unwrap());
                Ok(seq)
            }
            _ => Ok(0),
        }
    }

    // Mark entries as acknowledged (for safe deletion)
    pub fn mark_acked(&self, seq: u64) -> Result<()> {
        // In future, we'll store acked sequence in separate tree
        // For now, just log
        info!(seq, "✅ Entries up to {} marked as acknowledged", seq);
        Ok(())
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