use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub alert_id: String,
    pub truck_id: Uuid,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub source: String,
    pub context: serde_json::Value,
    pub actions: Vec<AlertAction>,
    pub status: AlertStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertType {
    DrowsyDriver,
    LaneDeparture,
    CargoTamper,
    LicensePlateMatch,
    WeatherHazard,
    HighTemperature,
    LowDiskSpace,
    HighCpuUsage,
    NetworkFailure,
    SensorFailure,
    HarshBraking,
    RapidAcceleration,
    SeatbeltNotFastened,
    DoorOpenWhileMoving,
    OverSpeeding,
    UpdateAvailable,
    UpdateFailed,
    RollbackTriggered,
    ConfigError,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertAction {
    pub action_id: String,
    pub action_type: ActionType,
    pub target: String,
    pub parameters: serde_json::Value,
    pub executed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    TriggerBuzzer,
    FlashLed,
    ShowOnDisplay,
    SendCanMessage,
    ActivateRelay,
    LogToServer,
    SendSms,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    Triggered,
    Acknowledged,
    Resolved,
    Suppressed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSummary {
    pub id: Uuid,
    pub alert_id: String,
    pub truck_id: Uuid,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: DateTime<Utc>,
    pub status: AlertStatus,
    pub truck_license_plate: String,
    pub truck_model: String,
    pub truck_make: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStats {
    pub total_alerts: i64,
    pub active_alerts: i64,
    pub acknowledged_alerts: i64,
    pub resolved_alerts: i64,
    pub by_severity: std::collections::HashMap<String, i64>,
    pub by_type: std::collections::HashMap<String, i64>,
    pub by_truck: std::collections::HashMap<Uuid, i64>,
    pub last_24_hours: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
}