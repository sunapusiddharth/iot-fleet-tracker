use crate::error::{ConfigError, Result};
use config::{Config as ConfigLoader, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};
use parking_lot::RwLock;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

// Metrics
static CONFIG_RELOAD_COUNT: AtomicU64 = AtomicU64::new(0);
static CONFIG_ERRORS_TOTAL: AtomicU64 = AtomicU64::new(0);

// Global config — safe to clone via Arc
static GLOBAL_CONFIG: Lazy<Arc<RwLock<Config>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Config::default()))
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub device_id: String,
    pub log_level: String,

    #[serde(default = "default_true")]
    pub enable_hot_reload: bool,

    pub mqtt: MqttConfig,
    pub sensors: SensorsConfig,
    pub camera: CameraConfig,
    pub storage: StorageConfig,
    pub alerts: AlertsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    pub broker_url: String,
    pub client_id: String,
    pub topic_prefix: String,
    pub qos: u8,
    pub keep_alive: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorsConfig {
    pub gps_device: String,
    pub obd_device: String,
    pub imu_device: String,
    pub sample_rate_hz: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub devices: Vec<String>, // e.g., ["/dev/video0", "/dev/video1"]
    pub resolution: String,   // "1920x1080"
    pub fps: u32,
    pub encode_quality: u8,   // 1-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub wal_path: String,
    pub max_wal_size_mb: u64,
    pub checkpoint_interval_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertsConfig {
    pub enable_local_alerts: bool,
    pub gpio_buzzer_pin: u8,
    pub alert_debounce_sec: u64,
}

fn default_true() -> bool { true }

impl Default for Config {
    fn default() -> Self {
        Self {
            device_id: "TRK-DEFAULT".to_string(),
            log_level: "info".to_string(),
            enable_hot_reload: true,
            mqtt: MqttConfig {
                broker_url: "mqtt://localhost:1883".to_string(),
                client_id: "truck-agent".to_string(),
                topic_prefix: "truck".to_string(),
                qos: 1,
                keep_alive: 30,
            },
            sensors: SensorsConfig {
                gps_device: "/dev/ttyUSB0".to_string(),
                obd_device: "/dev/ttyUSB1".to_string(),
                imu_device: "/dev/i2c-1".to_string(),
                sample_rate_hz: 10,
            },
            camera: CameraConfig {
                devices: vec!["/dev/video0".to_string()],
                resolution: "1280x720".to_string(),
                fps: 15,
                encode_quality: 85,
            },
            storage: StorageConfig {
                wal_path: "/var/lib/truck-agent/wal".to_string(),
                max_wal_size_mb: 1024,
                checkpoint_interval_sec: 300,
            },
            alerts: AlertsConfig {
                enable_local_alerts: true,
                gpio_buzzer_pin: 18,
                alert_debounce_sec: 10,
            },
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.device_id.is_empty() {
            return Err(ConfigError::ValidationError("device_id cannot be empty".to_string()));
        }
        if self.mqtt.qos > 2 {
            return Err(ConfigError::ValidationError("MQTT QoS must be 0, 1, or 2".to_string()));
        }
        if self.camera.fps == 0 {
            return Err(ConfigError::ValidationError("Camera FPS must be > 0".to_string()));
        }
        if self.storage.max_wal_size_mb == 0 {
            return Err(ConfigError::ValidationError("max_wal_size_mb must be > 0".to_string()));
        }
        Ok(())
    }

    pub fn load_from_file<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        if !path.exists() {
            return Err(ConfigError::FileNotFound(path.display().to_string()));
        }

        let mut cfg = ConfigLoader::new();
        cfg.merge(File::new(&path.to_string_lossy(), FileFormat::Toml))?;

        let config: Config = cfg.try_into()?;
        config.validate()?;

        info!(path = %path.display(), "✅ Config loaded and validated");
        Ok(config)
    }

    pub async fn start_hot_reload<P: Into<PathBuf> + Send + 'static>(
        config_path: P,
        shutdown: tokio::sync::broadcast::Receiver<()>,
        reload_tx: broadcast::Sender<()>,
    ) -> Result<()> {
        let config_path = config_path.into();
        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(event) => {
                    if event.kind.is_modify() {
                        info!("Config file modified, triggering reload...");
                        let _ = reload_tx.send(());
                    }
                }
                Err(e) => {
                    error!(error = %e, "Config watcher error");
                    CONFIG_ERRORS_TOTAL.fetch_add(1, Ordering::Relaxed);
                }
            }
        })?;

        watcher.watch(&config_path, notify::RecursiveMode::NonRecursive)?;

        info!(path = %config_path.display(), "👁️  Hot reload watcher started");

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("🛑 Hot reload watcher shutting down");
                    break;
                }
                _ = sleep(Duration::from_secs(1)) => {
                    // Keep alive
                }
            }
        }

        Ok(())
    }

    pub fn get_global() -> Arc<Config> {
        GLOBAL_CONFIG.read().clone()
    }

    pub fn set_global(new_config: Config) -> Result<()> {
        new_config.validate()?;
        let mut w = GLOBAL_CONFIG.write();
        *w = new_config;
        CONFIG_RELOAD_COUNT.fetch_add(1, Ordering::Relaxed);
        info!("🔄 Global config updated");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_config() {
        let toml = r#"
            device_id = "TRK-001"
            log_level = "debug"
            enable_hot_reload = true

            [mqtt]
            broker_url = "mqtt://broker:8883"
            client_id = "truck-001"
            topic_prefix = "fleet/truck"
            qos = 1
            keep_alive = 60

            [sensors]
            gps_device = "/dev/gps0"
            obd_device = "/dev/obd0"
            imu_device = "/dev/i2c-2"
            sample_rate_hz = 20

            [camera]
            devices = ["/dev/video0", "/dev/video1"]
            resolution = "1920x1080"
            fps = 30
            encode_quality = 90

            [storage]
            wal_path = "/data/wal"
            max_wal_size_mb = 2048
            checkpoint_interval_sec = 600

            [alerts]
            enable_local_alerts = true
            gpio_buzzer_pin = 21
            alert_debounce_sec = 5
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        fs::write(&mut temp_file, toml).unwrap();

        let config = Config::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.device_id, "TRK-001");
        assert_eq!(config.camera.fps, 30);
        assert_eq!(config.storage.max_wal_size_mb, 2048);
    }

    #[test]
    fn test_invalid_qos() {
        let mut config = Config::default();
        config.mqtt.qos = 3;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_empty_device_id() {
        let mut config = Config::default();
        config.device_id = "".to_string();
        assert!(config.validate().is_err());
    }
}