use crate::sensors::types::SensorEvent;
use crate::camera::types::CameraFrame;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalEntry {
    Sensor(SensorEvent),
    CameraMeta(CameraFrameMeta), // Store metadata + pointer to blob
    CameraBlob {
        blob_id: String,
         Vec<u8>,           // Full JPEG bytes
    },
    Checkpoint(CheckpointMarker),
    Heartbeat(HeartbeatMarker),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFrameMeta {
    pub camera_id: String,
    pub timestamp: u64,          // nanos since epoch
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub blob_id: String,         // points to CameraBlob entry
    pub is_keyframe: bool,
    pub trigger_event: Option<String>,
    pub meta FrameMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    pub exposure_us: Option<u32>,
    pub gain_db: Option<f32>,
    pub temperature_c: Option<f32>,
    pub gps_lat: Option<f64>,
    pub gps_lon: Option<f64>,
    pub speed_kmh: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMarker {
    pub checkpoint_id: String,
    pub timestamp: u64,
    pub total_entries: u64,
    pub safe_to_delete_before: u64, // sequence number
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatMarker {
    pub timestamp: u64,
    pub uptime_sec: u64,
    pub memory_used_bytes: u64,
}

impl WalEntry {
    pub fn timestamp(&self) -> u64 {
        match self {
            WalEntry::Sensor(e) => e.timestamp.timestamp_nanos() as u64,
            WalEntry::CameraMeta(m) => m.timestamp,
            WalEntry::CameraBlob { .. } => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            WalEntry::Checkpoint(c) => c.timestamp,
            WalEntry::Heartbeat(h) => h.timestamp,
        }
    }

    pub fn size_bytes(&self) -> usize {
        match self {
            WalEntry::Sensor(_) => 256, // approx
            WalEntry::CameraMeta(_) => 512, // approx
            WalEntry::CameraBlob { data, .. } => data.len(),
            WalEntry::Checkpoint(_) => 128,
            WalEntry::Heartbeat(_) => 64,
        }
    }
}
use crate::sensors::types::SensorEvent;
use crate::camera::types::CameraFrameMeta;
use crate::ml_edge::types::MLEvent;
use crate::health::types::HealthEvent;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalEntry {
    pub entry_id: String,
    pub entry_type: EntryType,
    pub payload: EntryPayload,
    pub timestamp: u64,
    pub priority: EntryPriority,
    pub size_bytes: usize,
    pub compression: CompressionInfo,
    pub encryption: Option<EncryptionInfo>,
    pub metadata: EntryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryPriority {
    Critical = 0,    // Safety alerts, drowsy driver
    High = 1,        // ML events, lane departure
    Medium = 2,      // Sensor data
    Low = 3,         // Heartbeats, health events
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    Sensor,
    CameraMeta,
    CameraBlob,
    Ml,
    Health,
    Heartbeat,
    Checkpoint,
    Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryPayload {
    Sensor(SensorEvent),
    CameraMeta(CameraFrameMeta),
    CameraBlob {
        blob_id: String,
         Vec<u8>,
        format: String,
    },
    Ml(MLEvent),
    Health(HealthEvent),
    Heartbeat(HeartbeatData),
    Checkpoint(CheckpointData),
    Command(CommandData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatData {
    pub uptime_sec: u64,
    pub memory_used_bytes: u64,
    pub disk_used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointData {
    pub checkpoint_id: String,
    pub timestamp: u64,
    pub total_entries: u64,
    pub safe_to_delete_before: u64,
    pub compaction_stats: CompactionStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandData {
    pub command_id: String,
    pub command_type: String,
    pub parameters: serde_json::Value,
    pub issued_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionStats {
    pub entries_compacted: u64,
    pub bytes_saved: u64,
    pub compaction_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionInfo {
    pub algorithm: String,
    pub level: u8,
    pub original_size: usize,
    pub compressed_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    pub algorithm: String,
    pub key_id: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    pub device_id: String,
    pub truck_id: String,
    pub sequence_number: u64,
    pub source_module: String,
    pub requires_ack: bool,
    pub acked: bool,
    pub retention_policy: RetentionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetentionPolicy {
    TimeBased { max_age_hours: u32 },
    SizeBased { max_size_mb: u32 },
    PriorityBased { min_priority: EntryPriority },
    Custom(String),
}

impl WalEntry {
    pub fn new_sensor(event: SensorEvent, device_id: &str, seq: u64) -> Self {
        let json = serde_json::to_vec(&event).unwrap();
        let size = json.len();
        
        Self {
            entry_id: format!("wal-{}-{}", device_id, seq),
            entry_type: EntryType::Sensor,
            payload: EntryPayload::Sensor(event),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
            priority: EntryPriority::Medium,
            size_bytes: size,
            compression: CompressionInfo {
                algorithm: "none".to_string(),
                level: 0,
                original_size: size,
                compressed_size: size,
            },
            encryption: None,
            meta EntryMetadata {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(),
                sequence_number: seq,
                source_module: "sensor".to_string(),
                requires_ack: true,
                acked: false,
                retention_policy: RetentionPolicy::TimeBased { max_age_hours: 72 },
            },
        }
    }

    // ... other constructors

    pub fn is_critical(&self) -> bool {
        self.priority == EntryPriority::Critical
    }

    pub fn should_retain(&self, current_time: u64, disk_usage_percent: f32) -> bool {
        match &self.metadata.retention_policy {
            RetentionPolicy::TimeBased { max_age_hours } => {
                let age_hours = (current_time - self.timestamp) / (3600 * 1_000_000_000);
                age_hours < (*max_age_hours as u64)
            }
            RetentionPolicy::SizeBased { max_size_mb } => {
                // In production, check actual disk usage
                disk_usage_percent < 85.0
            }
            RetentionPolicy::PriorityBased { min_priority } => {
                self.priority <= *min_priority
            }
            RetentionPolicy::Custom(_) => true, // Custom logic
        }
    }
}