use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaUpdate {
    pub id: Uuid,
    pub update_id: String,
    pub truck_id: Option<Uuid>,
    pub fleet_id: Option<Uuid>,
    pub version: String,
    pub target: UpdateTarget,
    pub url: String,
    pub checksum: String,
    pub signature: String,
    pub size_bytes: u64,
    pub priority: UpdatePriority,
    pub requires_reboot: bool,
    pub deadline: Option<DateTime<Utc>>,
    pub meta UpdateMetadata,
    pub status: OtaStatus,
    pub progress_percent: f32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateTarget {
    Agent,
    Model,
    Config,
    Firmware,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdatePriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMetadata {
    pub description: String,
    pub author: String,
    pub release_notes: String,
    pub compatibility: Vec<String>,
    pub estimated_apply_time_sec: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OtaStatus {
    Pending,
    Downloading,
    Verifying,
    Applying,
    Rollback,
    Success,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCommand {
    pub id: Uuid,
    pub command_id: String,
    pub truck_id: Option<Uuid>,
    pub fleet_id: Option<Uuid>,
    pub command_type: CommandType,
    pub parameters: serde_json::Value,
    pub issued_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub requires_ack: bool,
    pub status: CommandStatus,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandType {
    Reboot,
    Shutdown,
    RestartService,
    GetDiagnostics,
    UpdateConfig,
    RunHealthCheck,
    CaptureSnapshot,
    FlushWAL,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandStatus {
    Pending,
    Executing,
    Success,
    Failed,
    Timeout,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaSummary {
    pub id: Uuid,
    pub update_id: String,
    pub truck_id: Option<Uuid>,
    pub fleet_id: Option<Uuid>,
    pub version: String,
    pub target: UpdateTarget,
    pub priority: UpdatePriority,
    pub status: OtaStatus,
    pub progress_percent: f32,
    pub created_at: DateTime<Utc>,
    pub truck_license_plate: Option<String>,
    pub truck_model: Option<String>,
    pub truck_make: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaStats {
    pub total_updates: i64,
    pub pending_updates: i64,
    pub in_progress_updates: i64,
    pub successful_updates: i64,
    pub failed_updates: i64,
    pub rollback_updates: i64,
    pub by_target: std::collections::HashMap<String, i64>,
    pub by_priority: std::collections::HashMap<String, i64>,
    pub by_status: std::collections::HashMap<String, i64>,
    pub last_24_hours: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
}