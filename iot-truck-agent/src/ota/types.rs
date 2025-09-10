use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaUpdate {
    pub update_id: String,
    pub version: String,
    pub target: UpdateTarget,
    pub url: String,
    pub checksum: String,
    pub signature: String,
    pub size_bytes: u64,
    pub priority: UpdatePriority,
    pub requires_reboot: bool,
    pub deadline: Option<u64>, // Unix timestamp
    pub meta UpdateMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateTarget {
    Agent,      // Update the agent binary
    Model,      // Update ML models
    Config,     // Update configuration
    Firmware,   // Update device firmware
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdatePriority {
    Critical,   // Apply immediately (safety fixes)
    High,       // Apply within 1 hour
    Medium,     // Apply within 24 hours
    Low,        // Apply when convenient
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMetadata {
    pub description: String,
    pub author: String,
    pub release_notes: String,
    pub compatibility: Vec<String>, // Compatible device models
    pub estimated_apply_time_sec: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaStatus {
    pub update_id: String,
    pub status: UpdateStatus,
    pub progress_percent: f32,
    pub current_version: String,
    pub target_version: String,
    pub last_error: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateStatus {
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
    pub command_id: String,
    pub command_type: CommandType,
    pub parameters: serde_json::Value,
    pub issued_at: u64,
    pub deadline: Option<u64>,
    pub requires_ack: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    pub command_id: String,
    pub status: CommandStatus,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub completed_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandStatus {
    Success,
    Failed,
    Timeout,
    Cancelled,
}