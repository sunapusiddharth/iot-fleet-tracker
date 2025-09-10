use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub message_queue: MessageQueueSettings,
    pub storage: StorageSettings,
    pub auth: AuthSettings,
    pub telemetry: TelemetrySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub bind_address: String,
    pub http_port: u16,
    pub websocket_port: u16,
    pub metrics_port: u16,
    pub max_connections: usize,
    pub request_timeout_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub redis_uri: String,
    pub influxdb_url: String,
    pub influxdb_org: String,
    pub influxdb_bucket: String,
    pub influxdb_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQueueSettings {
    pub kafka_brokers: Vec<String>,
    pub kafka_topic: String,
    pub mqtt_broker: String,
    pub mqtt_username: Option<String>,
    pub mqtt_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSettings {
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSettings {
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub admin_username: String,
    pub admin_password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySettings {
    pub enable_tracing: bool,
    pub enable_metrics: bool,
    pub log_level: String,
}

impl ServerConfig {
    pub fn load_from_file<P: Into<PathBuf>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.into();
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}