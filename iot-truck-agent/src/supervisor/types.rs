use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub state: SystemStateType,
    pub uptime_sec: u64,
    pub last_heartbeat: u64,
    pub modules: Vec<ModuleState>,
    pub resources: ResourceUsage,
    pub meta SystemMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemStateType {
    Starting,
    Running,
    Degraded,
    ShuttingDown,
    Shutdown,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleState {
    pub name: String,
    pub status: ModuleStatus,
    pub last_heartbeat: u64,
    pub restarts: u32,
    pub last_restart: Option<u64>,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleStatus {
    Starting,
    Running,
    Degraded,
    Stopping,
    Stopped,
    Failed,
    Restarting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_percent: f32,
    pub temperature_c: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetadata {
    pub device_id: String,
    pub version: String,
    pub hostname: String,
    pub start_time: u64,
    pub shutdown_reason: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownSequence {
    pub sequence_id: String,
    pub reason: ShutdownReason,
    pub initiated_at: u64,
    pub timeout_sec: u32,
    pub steps: Vec<ShutdownStep>,
    pub status: ShutdownStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShutdownReason {
    Normal,
    Emergency,
    Update,
    Failure,
    Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownStep {
    pub step_id: String,
    pub module: String,
    pub action: ShutdownAction,
    pub timeout_sec: u32,
    pub status: StepStatus,
    pub completed_at: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShutdownAction {
    FlushData,
    StopProcessing,
    SaveState,
    CloseConnections,
    PowerOff,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    Pending,
    Executing,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShutdownStatus {
    InProgress,
    Completed,
    Failed,
    Timeout,
}

impl SystemState {
    pub fn new(device_id: &str) -> Self {
        Self {
            state: SystemStateType::Starting,
            uptime_sec: 0,
            last_heartbeat: chrono::Utc::now().timestamp_nanos() as u64,
            modules: Vec::new(),
            resources: ResourceUsage {
                cpu_percent: 0.0,
                memory_percent: 0.0,
                disk_percent: 0.0,
                temperature_c: 0.0,
            },
            meta SystemMetadata {
                device_id: device_id.to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                hostname: hostname::get().unwrap_or_default().to_string_lossy().to_string(),
                start_time: chrono::Utc::now().timestamp_nanos() as u64,
                shutdown_reason: None,
                last_error: None,
            },
        }
    }

    pub fn is_running(&self) -> bool {
        self.state == SystemStateType::Running
    }

    pub fn is_shutting_down(&self) -> bool {
        self.state == SystemStateType::ShuttingDown
    }
}