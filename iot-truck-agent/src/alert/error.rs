use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlertError {
    #[error("GPIO error: {0}")]
    GpioError(String),

    #[error("CAN bus error: {0}")]
    CanError(String),

    #[error("Display error: {0}")]
    DisplayError(String),

    #[error("Relay error: {0}")]
    RelayError(String),

    #[error("Alert policy error: {0}")]
    PolicyError(String),

    #[error("Alert trigger error: {0}")]
    TriggerError(String),

    #[error("Actuator not found: {0}")]
    ActuatorNotFound(String),

    #[error("Alert suppressed due to cooldown")]
    Suppressed,

    #[error("Alert closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, AlertError>;