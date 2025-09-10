use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    pub enable: bool,
    pub interval_ms: u64,
    pub enable_alerts: bool,
    pub alert_pin: u8,
    pub debounce_ms: u64,

    pub thresholds: Thresholds,
    pub degradation: DegradationConfig,
    pub network: NetworkConfig,
    pub thermal: ThermalConfig,
    pub disk: DiskConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thresholds {
    pub cpu_critical_percent: f32,
    pub cpu_warning_percent: f32,
    pub memory_critical_percent: f32,
    pub memory_warning_percent: f32,
    pub disk_critical_percent: f32,
    pub disk_warning_percent: f32,
    pub temp_critical_c: f32,
    pub temp_warning_c: f32,
    pub network_latency_critical_ms: f32,
    pub network_latency_warning_ms: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationConfig {
    pub enable_auto_degrade: bool,
    pub cpu_degrade_threshold: f32,
    pub memory_degrade_threshold: f32,
    pub disk_degrade_threshold: f32,
    pub network_degrade_threshold: f32,
    pub camera_fps_degrade_step: u32,
    pub ml_model_disable_order: Vec<String>, // ["license_plate", "cargo_tamper", "lane_departure", "drowsiness"]
    pub sensor_rate_degrade_step: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub ping_host: String,
    pub ping_interval_ms: u64,
    pub mqtt_health_check_interval_ms: u64,
    pub http_health_check_interval_ms: u64,
    pub max_latency_ms: f32,
    pub max_packet_loss_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalConfig {
    pub enable_thermal_throttling: bool,
    pub throttle_at_temp_c: f32,
    pub critical_shutdown_temp_c: f32,
    pub thermal_zone_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskConfig {
    pub enable_disk_pressure: bool,
    pub pressure_threshold_percent: f32,
    pub wal_rotate_early_at_percent: f32,
    pub camera_frame_drop_at_percent: f32,
}