use crate::supervisor::types::ShutdownSequence;
use crate::supervisor::error::Result;
use tokio::time::{sleep, Duration};
use tracing::{error, warn};

pub struct ShutdownTimeoutHandler;

impl ShutdownTimeoutHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn enforce_timeout(&self, sequence: &mut ShutdownSequence, force_shutdown: bool) -> Result<()> {
        let timeout = Duration::from_secs(sequence.timeout_sec as u64);
        let start_time = std::time::Instant::now();

        while start_time.elapsed() < timeout {
            if sequence.status == crate::supervisor::types::ShutdownStatus::Completed {
                return Ok(());
            }
            sleep(Duration::from_secs(1)).await;
        }

        warn!(sequence_id=%sequence.sequence_id, "⏳ Shutdown sequence timed out");

        if force_shutdown {
            sequence.status = crate::supervisor::types::ShutdownStatus::Timeout;
            // Force immediate shutdown
            std::process::exit(1);
        } else {
            return Err(crate::supervisor::error::SupervisorError::ShutdownTimeout(
                "Shutdown sequence timed out".to_string()
            ));
        }
    }

    pub async fn emergency_shutdown(&self) -> ! {
        error!("🚨 EMERGENCY SHUTDOWN INITIATED - FORCING IMMEDIATE EXIT");
        metrics::counter!("emergency_shutdowns_total").increment(1);
        std::process::exit(1);
    }
}