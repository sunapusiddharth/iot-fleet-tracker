use crate::stream::types::{StreamEvent, Batch, EventPriority};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use std::collections::VecDeque;

pub struct IntelligentBatcher {
    max_batch_size_bytes: usize,
    max_batch_events: usize,
    batch_timeout_ms: u64,
    priority_queues: [VecDeque<StreamEvent>; 4], // Critical, High, Medium, Low
}

impl IntelligentBatcher {
    pub fn new(max_size: usize, max_events: usize, timeout_ms: u64) -> Self {
        Self {
            max_batch_size_bytes: max_size,
            max_batch_events: max_events,
            batch_timeout_ms: timeout_ms,
            priority_queues: [
                VecDeque::new(), // Critical
                VecDeque::new(), // High
                VecDeque::new(), // Medium
                VecDeque::new(), // Low
            ],
        }
    }

    pub async fn start_batcher(
        mut rx: mpsc::Receiver<StreamEvent>,
        batch_tx: mpsc::Sender<Batch>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut timeout = tokio::time::sleep(Duration::from_millis(1000));

        loop {
            tokio::select! {
                Some(event) = rx.recv() => {
                    let priority_index = event.priority as usize;
                    if priority_index < 4 {
                        self.priority_queues[priority_index].push_back(event);
                    }
                    
                    // Try to create batch immediately for critical events
                    if event.priority == EventPriority::Critical {
                        if let Some(batch) = self.try_create_batch() {
                            if batch_tx.send(batch).await.is_err() {
                                return Err("Batch channel closed".into());
                            }
                        }
                    }
                }
                _ = &mut timeout => {
                    if let Some(batch) = self.try_create_batch() {
                        if batch_tx.send(batch).await.is_err() {
                            return Err("Batch channel closed".into());
                        }
                    }
                    timeout = tokio::time::sleep(Duration::from_millis(self.batch_timeout_ms));
                }
            }
        }
    }

    fn try_create_batch(&mut self) -> Option<Batch> {
        let mut events = Vec::new();
        let mut current_size = 0;
        let mut highest_priority = EventPriority::Low;

        // Always include critical events
        while let Some(event) = self.priority_queues[0].pop_front() {
            let event_size = event.size_bytes();
            if current_size + event_size > self.max_batch_size_bytes && !events.is_empty() {
                break;
            }
            if events.len() >= self.max_batch_events && !events.is_empty() {
                break;
            }
            events.push(event);
            current_size += event_size;
            highest_priority = EventPriority::Critical;
        }

        // Include high priority if space
        if current_size < self.max_batch_size_bytes / 2 {
            while let Some(event) = self.priority_queues[1].pop_front() {
                let event_size = event.size_bytes();
                if current_size + event_size > self.max_batch_size_bytes {
                    break;
                }
                if events.len() >= self.max_batch_events {
                    break;
                }
                events.push(event);
                current_size += event_size;
                if highest_priority > EventPriority::High {
                    highest_priority = EventPriority::High;
                }
            }
        }

        // Include medium/low if space and not time-critical
        if !events.is_empty() && events[0].priority > EventPriority::High {
            while current_size < self.max_batch_size_bytes / 4 && events.len() < self.max_batch_events {
                let event = if !self.priority_queues[2].is_empty() {
                    self.priority_queues[2].pop_front()
                } else if !self.priority_queues[3].is_empty() {
                    self.priority_queues[3].pop_front()
                } else {
                    break;
                };

                if let Some(event) = event {
                    let event_size = event.size_bytes();
                    events.push(event);
                    current_size += event_size;
                }
            }
        }

        if events.is_empty() {
            return None;
        }

        let batch_id = format!("batch-{}", chrono::Utc::now().timestamp_nanos());
        let created_at = chrono::Utc::now().timestamp_nanos() as u64;

        // Compress batch
        let uncompressed = serde_json::to_vec(&events)?;
        let compressed = zstd::encode_all(&uncompressed[..], 3)?;
        let compression_ratio = uncompressed.len() as f32 / compressed.len() as f32;

        Some(Batch {
            batch_id,
            events,
            created_at,
            size_bytes: compressed.len(),
            compression_ratio,
            priority: highest_priority,
            estimated_latency_ms: 0.0, // Will be filled by network monitor
        })
    }
}