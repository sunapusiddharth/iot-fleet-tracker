use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use geo::Point;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    pub id: Uuid,
    pub truck_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub location: Point<f64>,
    pub speed_kmh: f32,
    pub heading: f32,
    pub sensors: SensorData,
    pub cameras: Option<CameraData>,
    pub scenario: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub gps: GpsData,
    pub obd: ObdData,
    pub imu: ImuData,
    pub tpms: TpmsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub speed_kmh: f32,
    pub heading: f32,
    pub satellites: u8,
    pub fix_quality: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObdData {
    pub rpm: u16,
    pub speed_kmh: u8,
    pub coolant_temp: i8,
    pub fuel_level: u8,
    pub engine_load: u8,
    pub throttle_pos: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TpmsData {
    pub front_left: TireSensor,
    pub front_right: TireSensor,
    pub rear_left: TireSensor,
    pub rear_right: TireSensor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TireSensor {
    pub pressure_psi: f32,
    pub temperature_c: f32,
    pub battery_percent: u8,
    pub alert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraData {
    pub front_camera: Option<CameraFrameRef>,
    pub driver_camera: Option<CameraFrameRef>,
    pub cargo_camera: Option<CameraFrameRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFrameRef {
    pub frame_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_bytes: u64,
    pub is_keyframe: bool,
    pub meta FrameMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    pub exposure_us: Option<u32>,
    pub gain_db: Option<f32>,
    pub temperature_c: Option<f32>,
    pub gps_lat: Option<f64>,
    pub gps_lon: Option<f64>,
    pub speed_kmh: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySummary {
    pub truck_id: Uuid,
    pub last_timestamp: DateTime<Utc>,
    pub last_location: Point<f64>,
    pub last_speed_kmh: f32,
    pub last_heading: f32,
    pub last_rpm: u16,
    pub last_coolant_temp: i8,
    pub last_fuel_level: u8,
    pub last_accel_x: f32,
    pub last_accel_y: f32,
    pub last_accel_z: f32,
    pub tire_pressures: [f32; 4],
    pub tire_temperatures: [f32; 4],
}