use crate::supervisor::error::Result;
use backtrace::Backtrace;
use std::panic;
use tracing::{error, info};

pub struct PanicHandler;

impl PanicHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn setup_panic_handler(&self) {
        let original_hook = panic::take_hook();
        
        panic::set_hook(Box::new(move |panic_info| {
            // Call original hook first
            original_hook(panic_info);
            
            // Capture backtrace
            let backtrace = Backtrace::new();
            
            // Log panic info
            let payload = panic_info.payload();
            let message = if let Some(s) = payload.downcast_ref::<&str>() {
                *s
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.as_str()
            } else {
                "Unknown panic payload"
            };
            
            let location = panic_info.location().map(|l| format!("{}:{}", l.file(), l.line()));
            
            error!(
                message=%message,
                location=?location,
                backtrace=%format!("{:?}", backtrace),
                "💥 PANIC OCCURRED - CAPTURING STATE FOR DEBUGGING"
            );
            
            // Record panic in metrics
            metrics::counter!("panics_total").increment(1);
            
            // In production, save panic info to disk for later analysis
            if let Err(e) = Self::save_panic_info(message, location.as_deref(), &backtrace) {
                error!(error=%e, "Failed to save panic info");
            }
            
            // Initiate emergency shutdown
            Self::emergency_shutdown();
        }));
        
        info!("✅ Panic handler installed");
    }

    fn save_panic_info(message: &str, location: Option<&str>, backtrace: &Backtrace) -> Result<()> {
        let panic_info = serde_json::json!({
            "timestamp": chrono::Utc::now().timestamp_nanos(),
            "message": message,
            "location": location,
            "backtrace": format!("{:?}", backtrace),
            "version": env!("CARGO_PKG_VERSION"),
            "hostname": hostname::get().unwrap_or_default().to_string_lossy().to_string(),
        });
        
        // Save to disk
        let path = "/var/lib/truck-agent/panic.json";
        std::fs::write(path, serde_json::to_string_pretty(&panic_info)?)?;
        
        info!(path=%path, "💾 Panic info saved to disk");
        Ok(())
    }

    fn emergency_shutdown() -> ! {
        error!("💥 EMERGENCY SHUTDOWN INITIATED DUE TO PANIC");
        metrics::counter!("emergency_shutdowns_total").increment(1);
        std::process::exit(1);
    }
}