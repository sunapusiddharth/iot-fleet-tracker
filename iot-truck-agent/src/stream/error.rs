use thiserror::Error;

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("MQTT error: {0}")]
    MqttError(#[from] rumqttc::ClientError),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializeError(#[from] serde_json::Error),

    #[error("Network timeout")]
    Timeout,

    #[error("Authentication failed")]
    AuthFailed,

    #[error("Server rejected batch: {0}")]
    ServerRejected(String),

    #[error("No active transport available")]
    NoTransport,

    #[error("Batch too large: {0} bytes")]
    BatchTooLarge(usize),

    #[error("WAL error: {0}")]
    WalError(String),

    #[error("Stream closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, StreamError>;