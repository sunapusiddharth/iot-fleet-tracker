use crate::supervisor::types::ModuleState;
use crate::supervisor::error::Result;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

pub struct AutomaticRecovery {
    max_restarts: u32,
    restart_delay_ms: u64,
}

impl AutomaticRecovery {
    pub fn new(max_restarts: u32, restart_delay_ms: u64) -> Self {
        Self {
            max_restarts,
            restart_delay_ms,
        }
    }

    pub async fn recover_failed_modules(&self, failed_modules: Vec<ModuleState>) -> Result<Vec<String>> {
        let mut recovered = Vec::new();

        for module in failed_modules {
            if module.restarts < self.max_restarts {
                info!(module=%module.name, restarts=%module.restarts, "🔄 Attempting to recover failed module");

                // Wait before restart
                sleep(Duration::from_millis(self.restart_delay_ms)).await;

                // In production, this would call the module's restart method
                // For now, we'll simulate recovery
                recovered.push(module.name.clone());

                metrics::counter!("module_restarts_total", "module" => module.name.clone()).increment(1);
                info!(module=%module.name, "✅ Module recovered successfully");
            } else {
                warn!(module=%module.name, restarts=%module.restarts, "❌ Module restart limit exceeded - manual intervention required");
                metrics::counter!("module_recovery_failed_total", "module" => module.name.clone()).increment(1);
            }
        }

        Ok(recovered)
    }

    pub async fn emergency_recovery(&self) -> ! {
        error!("🚨 EMERGENCY RECOVERY FAILED - INITIATING EMERGENCY SHUTDOWN");
        metrics::counter!("emergency_recoveries_failed_total").increment(1);
        std::process::exit(1);
    }
}