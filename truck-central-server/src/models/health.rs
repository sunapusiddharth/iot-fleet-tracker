use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub id: Uuid,
    pub truck_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub status: HealthStatusType,
    pub resources: ResourceUsage,
    pub tasks: Vec<TaskStatus>,
    pub alerts: Vec<HealthAlert>,
    pub actions_taken: Vec<HealthAction>,
    pub meta HealthEventMetadata,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatusType {
    Ok,
    Warning,
    Critical,
    Degraded,
    ShutdownPending,
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
pub struct TaskStatus {
    pub name: String,
    pub is_alive: bool,
    pub last_seen_ms: u64,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: u64,
    pub restarts: u32,
    pub last_restart: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAlert {
    pub alert_id: String,
    pub alert_type: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: DateTime<Utc>,
    pub source: String,
    pub recommended_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub executed_at: DateTime<Utc>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub hardware_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    pub truck_id: Uuid,
    pub last_timestamp: DateTime<Utc>,
    pub status: HealthStatusType,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_percent: f32,
    pub temperature_c: f32,
    pub uptime_sec: u64,
    pub active_alerts: i32,
    pub health_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStats {
    pub total_health_events: i64,
    pub by_status: std::collections::HashMap<String, i64>,
    pub by_truck: std::collections::HashMap<Uuid, i64>,
    pub avg_cpu_percent: f32,
    pub avg_memory_percent: f32,
    pub avg_disk_percent: f32,
    pub avg_temperature_c: f32,
    pub last_24_hours: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
}