use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruckState {
    pub truck_id: String,
    pub timestamp: u64,
    pub location: (f64, f64),
    pub speed_kmh: f32,
    pub heading: f32,
    pub scenario: String,
    pub sensors: SensorData,
    pub cameras: CameraData,
    pub ml_events: Vec<MlEvent>,
    pub health_status: HealthStatus,
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
    pub front_camera: Option<CameraFrame>,
    pub driver_camera: Option<CameraFrame>,
    pub cargo_camera: Option<CameraFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFrame {
    pub timestamp: u64,
    pub width: u32,
    pub height: u32,
    pub format: String,
     Vec<u8>,
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
pub struct MlEvent {
    pub event_id: String,
    pub model_name: String,
    pub timestamp: u64,
    pub result: MlResult,
    pub confidence: f32,
    pub latency_ms: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MlResult {
    Drowsiness { is_drowsy: bool, eye_closure_ratio: f32 },
    LaneDeparture { is_departing: bool, deviation_pixels: i32 },
    CargoTamper { is_tampered: bool, motion_score: f32 },
    LicensePlate { plate_text: String, bounding_box: (f32, f32, f32, f32) },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_percent: f32,
    pub temperature_c: f32,
    pub network_quality: NetworkQuality,
    pub alerts: Vec<HealthAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkQuality {
    pub latency_ms: f32,
    pub packet_loss_percent: f32,
    pub bandwidth_kbps: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAlert {
    pub alert_type: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaCommand {
    pub command_id: String,
    pub truck_id: String,
    pub command_type: CommandType,
    pub parameters: serde_json::Value,
    pub issued_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandType {
    UpdateConfig,
    UpdateFirmware,
    Reboot,
    Shutdown,
    GetDiagnostics,
}