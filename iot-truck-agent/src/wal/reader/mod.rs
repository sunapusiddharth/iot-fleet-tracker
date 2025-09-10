use crate::wal::types::WalEntry;
use crate::wal::error::Result;
use sled::{Db, Tree};
use tracing::{info, warn};

pub struct WalReader {
    tree: Tree,
    encryptor: Option<crate::wal::writer::encryption::DataEncryptor>,
}

impl WalReader {
    pub fn new(db: &Db, enable_encryption: bool) -> Result<Self> {
        let tree = db.open_tree("main")?;
        let encryptor = if enable_encryption {
            Some(crate::wal::writer::encryption::DataEncryptor::new()?)
        } else {
            None
        };

        Ok(Self {
            tree,
            encryptor,
        })
    }

    pub fn replay_from(&self, start_seq: u64) -> Result<Vec<WalEntry>> {
        let mut entries = Vec::new();
        let mut iter = self.tree.range(start_seq.to_be_bytes()..);

        while let Some(Ok((key, value))) = iter.next() {
            let seq = u64::from_be_bytes(key.as_ref().try_into().unwrap());
            let mut entry: WalEntry = deserialize_wal_entry(&value)?;

            // Decrypt if needed
            if let Some(encryptor) = &self.encryptor {
                if entry.encryption.is_some() {
                    entry = encryptor.decrypt_entry(entry)?;
                }
            }

            entries.push(entry);
            metrics::counter!("wal_entries_replayed_total").increment(1);
        }

        info!(start_seq, count=entries.len(), "🔁 Replayed {} WAL entries", entries.len());
        Ok(entries)
    }

    pub fn get_entry(&self, seq: u64) -> Result<Option<WalEntry>> {
        let key = seq.to_be_bytes();
        if let Some(value) = self.tree.get(key)? {
            let mut entry: WalEntry = deserialize_wal_entry(&value)?;
            
            if let Some(encryptor) = &self.encryptor {
                if entry.encryption.is_some() {
                    entry = encryptor.decrypt_entry(entry)?;
                }
            }
            
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }

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

    pub fn mark_acked(&self, seq: u64) -> Result<()> {
        if let Some(mut entry) = self.get_entry(seq)? {
            entry.metadata.acked = true;
            
            let serialized = bincode::serialize(&entry)
                .map_err(|e| crate::wal::error::WalError::SerializeError(Box::new(e)))?;
            let key = seq.to_be_bytes();
            self.tree.insert(key, serialized)?;
            
            info!(seq, "✅ Entry marked as acknowledged");
            Ok(())
        } else {
            warn!(seq, "Entry not found for ACK");
            Ok(())
        }
    }
}

fn deserialize_wal_entry( &[u8]) -> Result<WalEntry> {
    if data.starts_with(b"zstd:") {
        let compressed = base64::decode(&data[5..])?;
        let decompressed = zstd::decode_all(&compressed[..])?;
        bincode::deserialize(&decompressed).map_err(|e| crate::wal::error::WalError::SerializeError(Box::new(e)))
    } else {
        bincode::deserialize(data).map_err(|e| crate::wal::error::WalError::SerializeError(Box::new(e)))
    }
}