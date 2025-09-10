use crate::supervisor::types::{ShutdownSequence, ShutdownStep, ShutdownStatus, StepStatus, ShutdownReason};
use crate::supervisor::error::Result;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

pub struct ShutdownSequenceManager;

impl ShutdownSequenceManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_shutdown_sequence(&self, reason: ShutdownReason, timeout_sec: u32) -> ShutdownSequence {
        let sequence_id = format!("shutdown-{}", chrono::Utc::now().timestamp_nanos());

        let mut steps = Vec::new();

        // Define shutdown sequence based on reason
        match reason {
            ShutdownReason::Emergency => {
                // Fast shutdown - save critical data only
                steps.push(ShutdownStep {
                    step_id: "flush_critical_data".to_string(),
                    module: "wal".to_string(),
                    action: crate::supervisor::types::ShutdownAction::FlushData,
                    timeout_sec: 5,
                    status: StepStatus::Pending,
                    completed_at: None,
                    error: None,
                });

                steps.push(ShutdownStep {
                    step_id: "stop_ml".to_string(),
                    module: "ml_edge".to_string(),
                    action: crate::supervisor::types::ShutdownAction::StopProcessing,
                    timeout_sec: 2,
                    status: StepStatus::Pending,
                    completed_at: None,
                    error: None,
                });
            }
            _ => {
                // Normal shutdown sequence
                steps.push(ShutdownStep {
                    step_id: "flush_all_data".to_string(),
                    module: "wal".to_string(),
                    action: crate::supervisor::types::ShutdownAction::FlushData,
                    timeout_sec: 30,
                    status: StepStatus::Pending,
                    completed_at: None,
                    error: None,
                });

                steps.push(ShutdownStep {
                    step_id: "stop_camera".to_string(),
                    module: "camera".to_string(),
                    action: crate::supervisor::types::ShutdownAction::StopProcessing,
                    timeout_sec: 10,
                    status: StepStatus::Pending,
                    completed_at: None,
                    error: None,
                });

                steps.push(ShutdownStep {
                    step_id: "stop_sensors".to_string(),
                    module: "sensors".to_string(),
                    action: crate::supervisor::types::ShutdownAction::StopProcessing,
                    timeout_sec: 5,
                    status: StepStatus::Pending,
                    completed_at: None,
                    error: None,
                });

                steps.push(ShutdownStep {
                    step_id: "stop_ml".to_string(),
                    module: "ml_edge".to_string(),
                    action: crate::supervisor::types::ShutdownAction::StopProcessing,
                    timeout_sec: 10,
                    status: StepStatus::Pending,
                    completed_at: None,
                    error: None,
                });

                steps.push(ShutdownStep {
                    step_id: "close_connections".to_string(),
                    module: "stream".to_string(),
                    action: crate::supervisor::types::ShutdownAction::CloseConnections,
                    timeout_sec: 15,
                    status: StepStatus::Pending,
                    completed_at: None,
                    error: None,
                });

                steps.push(ShutdownStep {
                    step_id: "save_state".to_string(),
                    module: "health".to_string(),
                    action: crate::supervisor::types::ShutdownAction::SaveState,
                    timeout_sec: 5,
                    status: StepStatus::Pending,
                    completed_at: None,
                    error: None,
                });
            }
        }

        ShutdownSequence {
            sequence_id,
            reason,
            initiated_at: chrono::Utc::now().timestamp_nanos() as u64,
            timeout_sec,
            steps,
            status: crate::supervisor::types::ShutdownStatus::InProgress,
        }
    }

    pub async fn execute_shutdown_sequence(&self, mut sequence: ShutdownSequence) -> Result<ShutdownSequence> {
        info!(sequence_id=%sequence.sequence_id, reason=%sequence.reason, "🔄 Starting shutdown sequence");

        let start_time = std::time::Instant::now();
        let timeout = Duration::from_secs(sequence.timeout_sec as u64);

        for i in 0..sequence.steps.len() {
            let step = &mut sequence.steps[i];
            
            if start_time.elapsed() > timeout {
                sequence.status = crate::supervisor::types::ShutdownStatus::Timeout;
                return Err(SupervisorError::ShutdownTimeout("Shutdown sequence timed out".to_string()));
            }

            info!(step_id=%step.step_id, module=%step.module, "🔧 Executing shutdown step");

            step.status = crate::supervisor::types::StepStatus::Executing;

            let step_result = self.execute_shutdown_step(step).await;

            match step_result {
                Ok(()) => {
                    step.status = crate::supervisor::types::StepStatus::Completed;
                    step.completed_at = Some(chrono::Utc::now().timestamp_nanos() as u64);
                    info!(step_id=%step.step_id, "✅ Shutdown step completed");
                }
                Err(e) => {
                    error!(step_id=%step.step_id, error=%e, "❌ Shutdown step failed");
                    step.status = crate::supervisor::types::StepStatus::Failed;
                    step.error = Some(e.to_string());
                    
                    // For emergency shutdown, continue on failure
                    if sequence.reason != ShutdownReason::Emergency {
                        sequence.status = crate::supervisor::types::ShutdownStatus::Failed;
                        return Err(SupervisorError::ModuleShutdownFailed(format!("Step {} failed: {}", step.step_id, e)));
                    }
                }
            }

            // Small delay between steps
            sleep(Duration::from_millis(100)).await;
        }

        sequence.status = crate::supervisor::types::ShutdownStatus::Completed;
        info!(sequence_id=%sequence.sequence_id, "✅ Shutdown sequence completed successfully");

        metrics::counter!("shutdown_sequences_total").increment(1);
        metrics::counter!("shutdown_sequences_success").increment(1);

        Ok(sequence)
    }

    async fn execute_shutdown_step(&self, step: &ShutdownStep) -> Result<()> {
        // In production, this would call into the specific module's shutdown method
        // For now, we'll simulate with delays based on step type

        match step.action {
            crate::supervisor::types::ShutdownAction::FlushData => {
                // Simulate data flush
                sleep(Duration::from_secs(step.timeout_sec as u64 / 2)).await;
            }
            crate::supervisor::types::ShutdownAction::StopProcessing => {
                // Simulate stopping processing
                sleep(Duration::from_secs(1)).await;
            }
            crate::supervisor::types::ShutdownAction::SaveState => {
                // Simulate saving state
                sleep(Duration::from_secs(2)).await;
            }
            crate::supervisor::types::ShutdownAction::CloseConnections => {
                // Simulate closing connections
                sleep(Duration::from_secs(3)).await;
            }
            crate::supervisor::types::ShutdownAction::PowerOff => {
                // Simulate power off
                sleep(Duration::from_secs(1)).await;
            }
        }

        Ok(())
    }
}