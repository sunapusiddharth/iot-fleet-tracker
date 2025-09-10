use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLEvent {
    pub event_id: String,
    pub model_name: String,
    pub timestamp: u64,
    pub result: InferenceResult,
    pub confidence: f32,
    pub latency_ms: f32,
    pub input_shape: (u32, u32),
    pub meta MLEventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InferenceResult {
    Drowsiness(DrowsinessResult),
    LaneDeparture(LaneDepartureResult),
    CargoTamper(CargoTamperResult),
    LicensePlate(LicensePlateResult),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrowsinessResult {
    pub is_drowsy: bool,
    pub eye_closure_ratio: f32,
    pub head_pose: (f32, f32, f32), // yaw, pitch, roll
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneDepartureResult {
    pub is_departing: bool,
    pub deviation_pixels: i32,
    pub lane_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoTamperResult {
    pub is_tampered: bool,
    pub motion_score: f32,
    pub object_count_change: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePlateResult {
    pub plate_text: String,
    pub plate_confidence: f32,
    pub bounding_box: (f32, f32, f32, f32), // x, y, w, h
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLEventMetadata {
    pub device_id: String,
    pub camera_id: String,
    pub frame_timestamp: u64,
    pub cpu_usage_percent: f32,
    pub memory_used_bytes: u64,
    pub model_version: String,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub model_file: String,
    pub enabled: bool,
    pub threshold: f32,
    pub input_width: u32,
    pub input_height: u32,
    pub roi: Option<(f32, f32, f32, f32)>, // x, y, w, h
    pub max_fps: u32,
}

impl MLEvent {
    pub fn new(
        model_name: &str,
        result: InferenceResult,
        confidence: f32,
        latency_ms: f32,
        input_shape: (u32, u32),
        device_id: &str,
        camera_id: &str,
        frame_timestamp: u64,
    ) -> Self {
        Self {
            event_id: format!("ml-{}-{}", model_name, frame_timestamp),
            model_name: model_name.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            result,
            confidence,
            latency_ms,
            input_shape,
            meta MLEventMetadata {
                device_id: device_id.to_string(),
                camera_id: camera_id.to_string(),
                frame_timestamp,
                cpu_usage_percent: 0.0, // Will be filled by engine
                memory_used_bytes: 0,   // Will be filled by engine
                model_version: "1.0".to_string(),
                retry_count: 0,
            },
        }
    }

    pub fn is_alert(&self) -> bool {
        match &self.result {
            InferenceResult::Drowsiness(d) => d.is_drowsy && self.confidence > 0.8,
            InferenceResult::LaneDeparture(l) => l.is_departing && self.confidence > 0.7,
            InferenceResult::CargoTamper(c) => c.is_tampered && self.confidence > 0.8,
            InferenceResult::LicensePlate(_) => self.confidence > 0.9, // High threshold
            _ => false,
        }
    }
}

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLEvent {
    pub event_id: String,
    pub model_name: String,
    pub model_version: String,
    pub timestamp: u64,
    pub result: InferenceResult,
    pub confidence: f32,
    pub calibrated_confidence: f32, // After calibration
    pub latency_ms: f32,
    pub input_shape: (u32, u32),
    pub hardware_used: HardwareType,
    pub meta MLEventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HardwareType {
    Cpu,
    Cuda,
    OpenVino,
    Fallback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InferenceResult {
    Drowsiness(DrowsinessResult),
    LaneDeparture(LaneDepartureResult),
    CargoTamper(CargoTamperResult),
    LicensePlate(LicensePlateResult),
    Weather(WeatherResult),
    Unknown,
}

// ... (previous result structs)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherResult {
    pub weather_type: WeatherType,
    pub confidence: f32,
    pub visibility_m: f32,
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
pub struct MLEventMetadata {
    pub device_id: String,
    pub truck_id: String,
    pub route_id: String,
    pub driver_id: String,
    pub camera_id: String,
    pub frame_timestamp: u64,
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
    pub time_of_day: String, // "day", "night", "dusk"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub model_file: String,
    pub model_checksum: String,
    pub enabled: bool,
    pub threshold: f32,
    pub calibrated_threshold: f32,
    pub input_width: u32,
    pub input_height: u32,
    pub roi: Option<(f32, f32, f32, f32)>,
    pub max_fps: u32,
    pub hardware_preference: Vec<HardwareType>,
    pub requires_sensor_fusion: bool,
    pub fallback_to_cloud: bool,
    pub calibration_params: CalibrationParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationParams {
    pub temperature_coefficient: f32,
    pub speed_coefficient: f32,
    pub time_of_day_bias: std::collections::HashMap<String, f32>,
    pub route_specific_calibration: std::collections::HashMap<String, f32>,
}