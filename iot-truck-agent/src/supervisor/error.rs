use thiserror::Error;

#[derive(Error, Debug)]
pub enum SupervisorError {
    #[error("Shutdown timeout: {0}")]
    ShutdownTimeout(String),

    #[error("Module shutdown failed: {0}")]
    ModuleShutdownFailed(String),

    #[error("System state error: {0}")]
    StateError(String),

    #[error("Watchdog error: {0}")]
    WatchdogError(String),

    #[error("Panic handler error: {0}")]
    PanicError(String),

    #[error("Signal handler error: {0}")]
    SignalError(String),

    #[error("Supervisor closed")]
    Closed,
}

pub type Result<T> = std::result::Result<T, SupervisorError>;