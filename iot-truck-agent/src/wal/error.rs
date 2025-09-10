use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalError {
    #[error("WAL I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializeError(#[from] Box<bincode::ErrorKind>),

    #[error("Sled error: {0}")]
    SledError(#[from] sled::Error),

    #[error("Disk full — cannot write WAL entry")]
    DiskFull,

    #[error("Corrupt WAL entry at seq {0}")]
    CorruptEntry(u64),

    #[error("Checkpoint conflict: {0}")]
    CheckpointConflict(String),

    #[error("WAL closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, WalError>;
