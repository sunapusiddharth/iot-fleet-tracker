use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("MQTT error: {0}")]
    MqttError(#[from] rumqttc::ClientError),

    #[error("HTTP error: {0}")]
    HttpError(#[from] axum::Error),

    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("Simulation error: {0}")]
    SimulationError(String),

    #[error("Truck not found: {0}")]
    TruckNotFound(String),

    #[error("Scenario not found: {0}")]
    ScenarioNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializeError(#[from] serde_json::Error),

    #[error("Simulator closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, SimulatorError>;