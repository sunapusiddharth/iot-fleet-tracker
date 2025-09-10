use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEvent {
    pub event_id: String,
    pub timestamp: u64,
    pub status: HealthStatus,
    pub resources: ResourceUsage,
    pub tasks: Vec<TaskStatus>,
    pub alerts: Vec<AlertInfo>,
    pub meta HealthEventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Ok,
    Warning,
    Critical,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub disk_percent: f32,
    pub disk_used_gb: u64,
    pub disk_total_gb: u64,
    pub temperature_c: f32,
    pub uptime_sec: u64,
    pub load_average: (f32, f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub name: String,
    pub is_alive: bool,
    pub last_seen_ms: u64,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertInfo {
    pub alert_type: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEventMetadata {
    pub device_id: String,
    pub version: String,
    pub hostname: String,
    pub ip_address: String,
    pub location: Option<(f64, f64)>, // Last known GPS
}

impl HealthEvent {
    pub fn new(
        status: HealthStatus,
        resources: ResourceUsage,
        device_id: &str,
    ) -> Self {
        Self {
            event_id: format!("health-{}", chrono::Utc::now().timestamp_nanos()),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            status,
            resources,
            tasks: Vec::new(),
            alerts: Vec::new(),
            meta HealthEventMetadata {
                device_id: device_id.to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                hostname: hostname::get().unwrap_or_default().to_string_lossy().to_string(),
                ip_address: local_ip::get().unwrap_or("unknown".to_string()),
                location: None, // Will be filled by GPS
            },
        }
    }

    pub fn is_critical(&self) -> bool {
        self.status == HealthStatus::Critical
    }

    pub fn is_degraded(&self) -> bool {
        self.status == HealthStatus::Degraded
    }
}


use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEvent {
    pub event_id: String,
    pub timestamp: u64,
    pub status: HealthStatus,
    pub resources: ResourceUsage,
    pub network: NetworkHealth,
    pub tasks: Vec<TaskStatus>,
    pub alerts: Vec<AlertInfo>,
    pub actions_taken: Vec<HealthAction>,
    pub meta HealthEventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum HealthStatus {
    Ok = 0,
    Warning = 1,
    Critical = 2,
    Degraded = 3,
    ShutdownPending = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f32,
    pub cpu_cores: usize,
    pub memory_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_available_mb: u64,
    pub swap_percent: f32,
    pub disk_percent: f32,
    pub disk_used_gb: u64,
    pub disk_total_gb: u64,
    pub disk_available_gb: u64,
    pub temperature_c: f32,
    pub thermal_throttling: bool,
    pub uptime_sec: u64,
    pub load_average: (f32, f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkHealth {
    pub mqtt_connected: bool,
    pub http_connected: bool,
    pub latency_ms: f32,
    pub last_heartbeat_ack: u64,
    pub packets_lost: u32,
    pub bandwidth_kbps: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub name: String,
    pub is_alive: bool,
    pub last_seen_ms: u64,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: u64,
    pub restarts: u32,
    pub last_restart: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertInfo {
    pub alert_id: String,
    pub alert_type: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: u64,
    pub source: String,
    pub recommended_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAction {
    pub action_id: String,
    pub action_type: ActionType,
    pub target_module: String,
    pub parameters: serde_json::Value,
    pub executed_at: u64,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionType {
    ThrottleCameraFps,
    DisableMlModel,
    ReduceSensorRate,
    RotateWalEarly,
    DropCameraFrames,
    ReduceLogLevel,
    RebootSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEventMetadata {
    pub device_id: String,
    pub version: String,
    pub hostname: String,
    pub ip_address: String,
    pub mac_address: String,
    pub location: Option<(f64, f64)>,
    pub hardware_model: String, // "Raspberry Pi 4", "Jetson Nano", etc.
}