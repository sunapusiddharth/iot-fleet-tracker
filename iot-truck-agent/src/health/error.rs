use thiserror::Error;

#[derive(Error, Debug)]
pub enum HealthError {
    #[error("System monitoring error: {0}")]
    SysInfoError(String),

    #[error("GPIO error: {0}")]
    GpioError(String),

    #[error("Task supervision error: {0}")]
    TaskError(String),

    #[error("Health monitor closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, HealthError>;