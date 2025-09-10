use crate::config::Config;
use crate::supervisor::types::{SystemState, SystemStateType, ShutdownReason};
use crate::supervisor::error::Result;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

pub mod types;
pub mod error;
pub mod shutdown;
pub mod watchdog;
pub mod panic;
pub mod lifecycle;
pub mod signal;

// Metrics
metrics::describe_counter!("shutdown_sequences_total", "Total shutdown sequences");
metrics::describe_counter!("shutdown_sequences_success", "Successful shutdown sequences");
metrics::describe_counter!("shutdown_sequences_failed", "Failed shutdown sequences");
metrics::describe_counter!("emergency_shutdowns_total", "Total emergency shutdowns");
metrics::describe_counter!("panics_total", "Total panics");
metrics::describe_counter!("module_restarts_total", "Total module restarts");
metrics::describe_counter!("module_recovery_failed_total", "Module recovery failed");
metrics::describe_gauge!("system_uptime_sec", "System uptime in seconds");
metrics::describe_gauge!("system_state", "System state (0=Starting, 1=Running, 2=Degraded, 3=ShuttingDown, 4=Shutdown, 5=Failed)");

pub struct Supervisor {
    config: Config,
    health_monitor: crate::supervisor::watchdog::health::ModuleHealthMonitor,
    recovery_manager: crate::supervisor::watchdog::recovery::AutomaticRecovery,
    shutdown_manager: crate::supervisor::shutdown::sequence::ShutdownSequenceManager,
    timeout_handler: crate::supervisor::shutdown::timeout::ShutdownTimeoutHandler,
    signal_handler: crate::supervisor::signal::handler::SignalHandler,
    panic_handler: crate::supervisor::panic::PanicHandler,
    tx: broadcast::Sender<crate::stream::types::StreamEvent>,
    device_id: String,
}

impl Supervisor {
    pub async fn new(config: Config, tx: broadcast::Sender<crate::stream::types::StreamEvent>) -> Result<Self> {
        let health_monitor = crate::supervisor::watchdog::health::ModuleHealthMonitor::new();
        let recovery_manager = crate::supervisor::watchdog::recovery::AutomaticRecovery::new(3, 5000);
        let shutdown_manager = crate::supervisor::shutdown::sequence::ShutdownSequenceManager::new();
        let timeout_handler = crate::supervisor::shutdown::timeout::ShutdownTimeoutHandler::new();
        let signal_handler = crate::supervisor::signal::handler::SignalHandler::new(ShutdownReason::Normal);
        let panic_handler = crate::supervisor::panic::PanicHandler::new();

        // Initialize system state
        crate::supervisor::lifecycle::state::initialize_system_state(&config.device_id);

        // Setup panic handler
        panic_handler.setup_panic_handler();

        // Setup signal handlers
        signal_handler.setup_signal_handlers().await?;

        info!("✅ Supervisor initialized");

        Ok(Self {
            config,
            health_monitor,
            recovery_manager,
            shutdown_manager,
            timeout_handler,
            signal_handler,
            panic_handler,
            tx,
            device_id: config.device_id.clone(),
        })
    }

    pub fn get_health_monitor(&mut self) -> &mut crate::supervisor::watchdog::health::ModuleHealthMonitor {
        &mut self.health_monitor
    }

    pub async fn start_monitoring(&self) -> Result<()> {
        info!("👁️  Starting system monitoring");

        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

        loop {
            interval.tick().await;

            // Check module health
            let modules = self.health_monitor.check_modules();
            
            // Update system state
            let system_state = crate::supervisor::lifecycle::state::get_system_state();
            crate::supervisor::lifecycle::state::update_system_state(|s| {
                s.modules = modules.clone();
                s.last_heartbeat = chrono::Utc::now().timestamp_nanos() as u64;
                s.uptime_sec = (chrono::Utc::now().timestamp_nanos() as u64 - s.meta.start_time) / 1_000_000_000;
            });

            // Check for failed modules
            let failed_modules: Vec<_> = modules.iter()
                .filter(|m| m.status == crate::supervisor::types::ModuleStatus::Failed)
                .cloned()
                .collect();

            if !failed_modules.is_empty() {
                warn!(count=%failed_modules.len(), "⚠️  {} modules failed - attempting recovery", failed_modules.len());
                
                match self.recovery_manager.recover_failed_modules(failed_modules).await {
                    Ok(recovered) => {
                        info!(count=%recovered.len(), "✅ {} modules recovered", recovered.len());
                    }
                    Err(e) => {
                        error!(error=%e, "❌ Module recovery failed");
                        metrics::counter!("module_recovery_failed_total").increment(1);
                    }
                }
            }

            // Update metrics
            let state = crate::supervisor::lifecycle::state::get_system_state();
            metrics::gauge!("system_uptime_sec").set(state.uptime_sec as f64);
            metrics::gauge!("system_state").set(match state.state {
                SystemStateType::Starting => 0.0,
                SystemStateType::Running => 1.0,
                SystemStateType::Degraded => 2.0,
                SystemStateType::ShuttingDown => 3.0,
                SystemStateType::Shutdown => 4.0,
                SystemStateType::Failed => 5.0,
            });

            // Send system state to streamer
            let stream_event = crate::stream::types::StreamEvent::new_system_state(state, &self.device_id);
            if self.tx.send(stream_event).is_err() {
                warn!("System state channel full — dropping event");
            }
        }
    }

    pub async fn shutdown(&self, reason: ShutdownReason) -> Result<()> {
        info!(reason=%format!("{:?}", reason), "🛑 Initiating system shutdown");

        // Set system state
        crate::supervisor::lifecycle::state::set_system_state(SystemStateType::ShuttingDown);
        crate::supervisor::lifecycle::state::set_shutdown_reason(reason.clone());

        // Create shutdown sequence
        let timeout_sec = match reason {
            ShutdownReason::Emergency => 10,
            _ => 60,
        };

        let sequence = self.shutdown_manager.create_shutdown_sequence(reason.clone(), timeout_sec);
        
        // Execute shutdown sequence
        match self.shutdown_manager.execute_shutdown_sequence(sequence).await {
            Ok(completed_sequence) => {
                info!(sequence_id=%completed_sequence.sequence_id, "✅ Shutdown completed successfully");
                crate::supervisor::lifecycle::state::set_system_state(SystemStateType::Shutdown);
                metrics::counter!("shutdown_sequences_success").increment(1);
            }
            Err(e) => {
                error!(error=%e, "❌ Shutdown failed");
                metrics::counter!("shutdown_sequences_failed").increment(1);
                
                // Force emergency shutdown
                if reason != ShutdownReason::Emergency {
                    self.timeout_handler.emergency_shutdown().await;
                }
            }
        }

        Ok(())
    }

    pub async fn emergency_shutdown(&self) -> ! {
        error!("🚨 EMERGENCY SHUTDOWN INITIATED");
        metrics::counter!("emergency_shutdowns_total").increment(1);
        self.timeout_handler.emergency_shutdown().await
    }
}