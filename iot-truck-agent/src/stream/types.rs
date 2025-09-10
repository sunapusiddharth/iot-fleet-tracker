use crate::sensors::types::SensorEvent;
use crate::camera::types::CameraFrameMeta;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    pub event_id: String,     // UUID or seq for dedup
    pub event_type: EventType,
    pub payload: EventPayload,
    pub timestamp: u64,       // nanos since epoch
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Sensor,
    CameraMeta,
    CameraBlob,
    Heartbeat,
    Checkpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    Sensor(SensorEvent),
    CameraMeta(CameraFrameMeta),
    CameraBlob {
        blob_id: String,
         Vec<u8>,           // Compressed JPEG bytes
    },
    Heartbeat(HeartbeatData),
    Checkpoint(CheckpointData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub device_id: String,
    pub truck_id: String,
    pub sequence_number: u64,
    pub retry_count: u32,
    pub source_module: String, // "sensor", "camera", "wal"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatData {
    pub uptime_sec: u64,
    pub memory_used_bytes: u64,
    pub disk_used_bytes: u64,
    pub last_ack_seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointData {
    pub checkpoint_id: String,
    pub total_entries: u64,
    pub safe_to_delete_before: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub batch_id: String,
    pub events: Vec<StreamEvent>,
    pub created_at: u64,
    pub size_bytes: usize,
    pub compression_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ack {
    pub batch_id: String,
    pub received_at: u64,
    pub event_ids: Vec<String>,
    pub status: AckStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AckStatus {
    Success,
    Partial,
    Failed,
}

impl StreamEvent {
    pub fn new_sensor(event: SensorEvent, device_id: &str, seq: u64) -> Self {
        Self {
            event_id: format!("evt-{}-{}", device_id, seq),
            event_type: EventType::Sensor,
            payload: EventPayload::Sensor(event),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            meta EventMetadata {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(), // or separate field
                sequence_number: seq,
                retry_count: 0,
                source_module: "sensor".to_string(),
            },
        }
    }

    pub fn new_camera_meta(meta: CameraFrameMeta, device_id: &str, seq: u64) -> Self {
        Self {
            event_id: format!("evt-{}-{}", device_id, seq),
            event_type: EventType::CameraMeta,
            payload: EventPayload::CameraMeta(meta),
            timestamp: meta.timestamp,
            meta EventMetadata {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(),
                sequence_number: seq,
                retry_count: 0,
                source_module: "camera".to_string(),
            },
        }
    }

    pub fn new_camera_blob(blob_id: &str,  Vec<u8>, device_id: &str, seq: u64) -> Self {
        Self {
            event_id: format!("evt-{}-{}", device_id, seq),
            event_type: EventType::CameraBlob,
            payload: EventPayload::CameraBlob {
                blob_id: blob_id.to_string(),
                 data,
            },
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            metadata: EventMetadata {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(),
                sequence_number: seq,
                retry_count: 0,
                source_module: "camera".to_string(),
            },
        }
    }
}
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    pub event_id: String,     // UUID or seq for dedup
    pub event_type: EventType,
    pub payload: EventPayload,
    pub timestamp: u64,       // nanos since epoch
    pub priority: EventPriority,
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Critical = 0,    // Safety alerts, drowsy driver
    High = 1,        // ML events, lane departure
    Medium = 2,      // Sensor data
    Low = 3,         // Heartbeats, health events
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Sensor,
    CameraMeta,
    CameraBlob,
    Ml,
    Health,
    Heartbeat,
    Checkpoint,
    CommandResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    Sensor(crate::sensors::types::SensorEvent),
    CameraMeta(crate::camera::types::CameraFrameMeta),
    CameraBlob {
        blob_id: String,
         Vec<u8>,           // Compressed bytes
        compression_type: CompressionType,
    },
    Ml(crate::ml_edge::types::MLEvent),
    Health(crate::health::types::HealthEvent),
    Heartbeat(HeartbeatData),
    Checkpoint(crate::wal::types::CheckpointMarker),
    CommandResponse(CommandResponseData),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionType {
    None,
    Zstd,
    H264,
    Delta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatData {
    pub uptime_sec: u64,
    pub memory_used_bytes: u64,
    pub disk_used_bytes: u64,
    pub last_ack_seq: u64,
    pub network_quality: NetworkQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkQuality {
    pub latency_ms: f32,
    pub packet_loss_percent: f32,
    pub bandwidth_kbps: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponseData {
    pub command_id: String,
    pub status: CommandStatus,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandStatus {
    Success,
    Failed,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub device_id: String,
    pub truck_id: String,
    pub sequence_number: u64,
    pub retry_count: u32,
    pub source_module: String,
    pub requires_ack: bool,
    pub qos: QoSLevel,
    pub encryption: Option<EncryptionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QoSLevel {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    pub algorithm: String,
    pub key_id: String,
    pub iv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub batch_id: String,
    pub events: Vec<StreamEvent>,
    pub created_at: u64,
    pub size_bytes: usize,
    pub compression_ratio: f32,
    pub priority: EventPriority,
    pub estimated_latency_ms: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ack {
    pub batch_id: String,
    pub received_at: u64,
    pub event_ids: Vec<String>,
    pub status: AckStatus,
    pub server_sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AckStatus {
    Success,
    Partial,
    Failed,
    Duplicate,
}

impl StreamEvent {
    pub fn new_sensor(event: crate::sensors::types::SensorEvent, device_id: &str, seq: u64) -> Self {
        Self {
            event_id: format!("evt-{}-{}", device_id, seq),
            event_type: EventType::Sensor,
            payload: EventPayload::Sensor(event),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
            priority: EventPriority::Medium,
            meta EventMetadata {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(),
                sequence_number: seq,
                retry_count: 0,
                source_module: "sensor".to_string(),
                requires_ack: true,
                qos: QoSLevel::AtLeastOnce,
                encryption: None,
            },
        }
    }

    // ... other constructors

    pub fn size_bytes(&self) -> usize {
        match &self.payload {
            EventPayload::Sensor(_) => 256,
            EventPayload::CameraMeta(_) => 512,
            EventPayload::CameraBlob { data, .. } => data.len(),
            EventPayload::Ml(_) => 1024,
            EventPayload::Health(_) => 2048,
            EventPayload::Heartbeat(_) => 128,
            EventPayload::Checkpoint(_) => 256,
            EventPayload::CommandResponse(_) => 512,
        }
    }

    pub fn is_critical(&self) -> bool {
        self.priority == EventPriority::Critical
    }
}