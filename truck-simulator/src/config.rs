use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatorConfig {
    pub server: ServerConfig,
    pub simulation: SimulationConfig,
    pub scenarios: Vec<ScenarioConfig>,
    pub trucks: Vec<TruckConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub metrics_port: u16,
    pub mqtt_port: u16,
    pub http_port: u16,
    pub websocket_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub num_trucks: usize,
    pub update_interval_ms: u64,
    pub enable_mqtt: bool,
    pub enable_http: bool,
    pub enable_websocket: bool,
    pub base_latitude: f64,
    pub base_longitude: f64,
    pub route_radius_km: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioConfig {
    pub name: String,
    pub description: String,
    pub probability: f32,
    pub duration_minutes: u32,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruckConfig {
    pub id: String,
    pub model: String,
    pub initial_latitude: f64,
    pub initial_longitude: f64,
    pub initial_speed: f32,
    pub sensor_config: SensorConfig,
    pub camera_config: CameraConfig,
    pub ml_config: MlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorConfig {
    pub gps_update_rate_hz: u32,
    pub obd_update_rate_hz: u32,
    pub imu_update_rate_hz: u32,
    pub tpms_update_rate_hz: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub front_camera: bool,
    pub driver_camera: bool,
    pub cargo_camera: bool,
    pub resolution: String,
    pub fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlConfig {
    pub drowsiness_detection: bool,
    pub lane_departure_detection: bool,
    pub cargo_tamper_detection: bool,
    pub license_plate_detection: bool,
}

impl SimulatorConfig {
    pub fn load_from_file<P: Into<PathBuf>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.into();
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}