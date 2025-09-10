use thiserror::Error;

#[derive(Error, Debug)]
pub enum OtaError {
    #[error("Download error: {0}")]
    DownloadError(#[from] reqwest::Error),

    #[error("Filesystem error: {0}")]
    FsError(#[from] std::io::Error),

    #[error("Signature verification failed: {0}")]
    SignatureError(String),

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumError { expected: String, actual: String },

    #[error("Update conflict: {0}")]
    ConflictError(String),

    #[error("Insufficient disk space: required {required} bytes, available {available} bytes")]
    InsufficientSpace { required: u64, available: u64 },

    #[error("Bandwidth limit exceeded")]
    BandwidthLimitExceeded,

    #[error("Update timeout")]
    Timeout,

    #[error("Rollback failed: {0}")]
    RollbackFailed(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("OTA closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, OtaError>;