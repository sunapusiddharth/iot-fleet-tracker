use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlEvent {
    pub id: Uuid,
    pub event_id: String,
    pub truck_id: Uuid,
    pub model_name: String,
    pub model_version: String,
    pub timestamp: DateTime<Utc>,
    pub result: MlResult,
    pub confidence: f32,
    pub calibrated_confidence: f32,
    pub latency_ms: f32,
    pub hardware_used: HardwareType,
    pub meta MlEventMetadata,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HardwareType {
    Cpu,
    Cuda,
    OpenVino,
    Fallback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MlResult {
    Drowsiness { is_drowsy: bool, eye_closure_ratio: f32 },
    LaneDeparture { is_departing: bool, deviation_pixels: i32 },
    CargoTamper { is_tampered: bool, motion_score: f32 },
    LicensePlate { plate_text: String, bounding_box: (f32, f32, f32, f32) },
    Weather { weather_type: WeatherType, visibility_m: f32 },
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeatherType {
    Clear,
    Rain,
    Fog,
    Snow,
    Night,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlEventMetadata {
    pub device_id: String,
    pub truck_id: String,
    pub route_id: String,
    pub driver_id: String,
    pub camera_id: String,
    pub frame_timestamp: DateTime<Utc>,
    pub sensor_context: Option<SensorContext>,
    pub cpu_usage_percent: f32,
    pub gpu_usage_percent: f32,
    pub memory_used_bytes: u64,
    pub temperature_c: f32,
    pub model_checksum: String,
    pub retry_count: u32,
    pub fallback_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorContext {
    pub speed_kmh: f32,
    pub acceleration: f32,
    pub steering_angle: f32,
    pub gps_lat: f64,
    pub gps_lon: f64,
    pub time_of_day: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlEventSummary {
    pub id: Uuid,
    pub event_id: String,
    pub truck_id: Uuid,
    pub model_name: String,
    pub result_type: String,
    pub confidence: f32,
    pub timestamp: DateTime<Utc>,
    pub is_alert: bool,
    pub truck_license_plate: String,
    pub truck_model: String,
    pub truck_make: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlStats {
    pub total_events: i64,
    pub alert_events: i64,
    pub by_model: std::collections::HashMap<String, i64>,
    pub by_result: std::collections::HashMap<String, i64>,
    pub by_truck: std::collections::HashMap<Uuid, i64>,
    pub avg_confidence: f32,
    pub avg_latency_ms: f32,
    pub last_24_hours: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
}