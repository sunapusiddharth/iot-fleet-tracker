use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorEvent {
    pub sensor_id: String,
    pub sensor_type: SensorType,
    pub timestamp: DateTime<Utc>,
    pub values: SensorValues,
    pub raw_payload: Option<String>, // for debugging
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SensorType {
    Gps,
    Obd,
    Imu,
    Tpms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorValues {
    Gps(GpsData),
    Obd(ObdData),
    Imu(ImuData),
    Tpms(TpmsData),
}

// --- GPS ---
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

// --- OBD-II ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObdData {
    pub rpm: u16,
    pub speed_kmh: u8,
    pub coolant_temp: i8,
    pub fuel_level: u8, // 0-100%
    pub engine_load: u8,
    pub throttle_pos: u8,
}

// --- IMU (Accelerometer) ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuData {
    pub accel_x: f32, // g-force
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,  // deg/s (future)
    pub gyro_y: f32,
    pub gyro_z: f32,
}

// --- TPMS (Tire Pressure) ---
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

impl fmt::Display for SensorEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} @ {}",
            self.sensor_type,
            self.sensor_id,
            self.timestamp
        )
    }
}