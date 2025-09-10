use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AlertType {
    // ML-based alerts
    DrowsyDriver,
    LaneDeparture,
    CargoTamper,
    LicensePlateMatch,
    WeatherHazard,

    // Health-based alerts
    HighTemperature,
    LowDiskSpace,
    HighCpuUsage,
    NetworkFailure,
    SensorFailure,

    // Sensor-based alerts
    HarshBraking,
    RapidAcceleration,
    SeatbeltNotFastened,
    DoorOpenWhileMoving,
    OverSpeeding,

    // System alerts
    UpdateAvailable,
    UpdateFailed,
    RollbackTriggered,
    ConfigError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_id: String,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: u64,
    pub source: String,
    pub context: AlertContext,
    pub actions: Vec<AlertAction>,
    pub status: AlertStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info = 0,
    Warning = 1,
    Critical = 2,
    Emergency = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertContext {
    pub device_id: String,
    pub truck_id: String,
    pub location: Option<(f64, f64)>,
    pub speed_kmh: Option<f32>,
    pub confidence: Option<f32>,
    pub sensor_values: Option<serde_json::Value>,
    pub ml_results: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertAction {
    pub action_id: String,
    pub action_type: ActionType,
    pub target: String,
    pub parameters: serde_json::Value,
    pub executed_at: Option<u64>,
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
    SendSms, // Future
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    Triggered,
    Acknowledged,
    Resolved,
    Suppressed,
}

impl Alert {
    pub fn new(
        alert_type: AlertType,
        severity: AlertSeverity,
        message: &str,
        device_id: &str,
    ) -> Self {
        Self {
            alert_id: format!("alert-{}-{}", alert_type, chrono::Utc::now().timestamp_nanos()),
            alert_type,
            severity,
            message: message.to_string(),
            triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            source: "unknown".to_string(),
            context: AlertContext {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(),
                location: None,
                speed_kmh: None,
                confidence: None,
                sensor_values: None,
                ml_results: None,
            },
            actions: Vec::new(),
            status: AlertStatus::Triggered,
        }
    }

    pub fn is_emergency(&self) -> bool {
        self.severity == AlertSeverity::Emergency
    }

    pub fn is_critical(&self) -> bool {
        self.severity == AlertSeverity::Critical
    }

    pub fn requires_immediate_action(&self) -> bool {
        self.severity >= AlertSeverity::Critical
    }
}