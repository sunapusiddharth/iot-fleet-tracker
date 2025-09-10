use crate::wal::types::WalEntry;
use tokio::sync::Mutex;
use std::collections::VecDeque;

pub struct WriteBuffer {
    buffer: Mutex<VecDeque<(u64, WalEntry)>>,
    max_size_bytes: usize,
    current_size: Mutex<usize>,
    next_sequence: std::sync::atomic::AtomicU64,
}

impl WriteBuffer {
    pub fn new(max_size_bytes: usize) -> Self {
        Self {
            buffer: Mutex::new(VecDeque::new()),
            max_size_bytes,
            current_size: Mutex::new(0),
            next_sequence: std::sync::atomic::AtomicU64::new(1),
        }
    }

    pub async fn write_entry(&self, entry: WalEntry) -> Result<u64, Box<dyn std::error::Error>> {
        let seq = self.next_sequence.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        let mut buffer = self.buffer.lock().await;
        let mut current_size = self.current_size.lock().await;
        
        *current_size += entry.size_bytes;
        buffer.push_back((seq, entry));
        
        Ok(seq)
    }

    pub async fn flush(&self) -> Result<Vec<(u64, WalEntry)>, Box<dyn std::error::Error>> {
        let mut buffer = self.buffer.lock().await;
        let mut current_size = self.current_size.lock().await;
        
        let entries = buffer.drain(..).collect::<Vec<_>>();
        *current_size = 0;
        
        Ok(entries)
    }

    pub async fn size(&self) -> usize {
        *self.current_size.lock().await
    }

    pub async fn should_flush(&self) -> bool {
        self.size().await >= self.max_size_bytes / 2
    }
}