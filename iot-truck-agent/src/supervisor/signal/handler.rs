use crate::supervisor::types::ShutdownReason;
use crate::supervisor::error::Result;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use tokio::signal::unix::{signal, SignalKind};
use tracing::{error, info, warn};

pub struct SignalHandler {
    shutdown_reason: ShutdownReason,
}

impl SignalHandler {
    pub fn new(shutdown_reason: ShutdownReason) -> Self {
        Self {
            shutdown_reason,
        }
    }

    pub async fn setup_signal_handlers(&self) -> Result<()> {
        // Handle SIGINT (Ctrl+C)
        let mut sigint = signal(SignalKind::interrupt())?;
        
        // Handle SIGTERM (kill)
        let mut sigterm = signal(SignalKind::terminate())?;
        
        // Handle SIGQUIT (Ctrl+\)
        let mut sigquit = signal(SignalKind::quit())?;

        info!("✅ Signal handlers installed");

        tokio::spawn(async move {
            tokio::select! {
                _ = sigint.recv() => {
                    info!("👋 SIGINT received - initiating graceful shutdown");
                    crate::supervisor::lifecycle::state::set_shutdown_reason(ShutdownReason::Normal);
                    crate::supervisor::shutdown::timeout::ShutdownTimeoutHandler::new().emergency_shutdown();
                }
                _ = sigterm.recv() => {
                    info!("👋 SIGTERM received - initiating graceful shutdown");
                    crate::supervisor::lifecycle::state::set_shutdown_reason(ShutdownReason::Normal);
                    crate::supervisor::shutdown::timeout::ShutdownTimeoutHandler::new().emergency_shutdown();
                }
                _ = sigquit.recv() => {
                    info!("👋 SIGQUIT received - initiating emergency shutdown");
                    crate::supervisor::lifecycle::state::set_shutdown_reason(ShutdownReason::Emergency);
                    crate::supervisor::shutdown::timeout::ShutdownTimeoutHandler::new().emergency_shutdown();
                }
            }
        });

        Ok(())
    }

    pub fn block_signals(&self) -> Result<()> {
        let signals = &[Signal::SIGINT, Signal::SIGTERM, Signal::SIGQUIT];
        signal::block(signals)?;
        info!("🚧 Signals blocked during critical operations");
        Ok(())
    }

    pub fn unblock_signals(&self) -> Result<()> {
        let signals = &[Signal::SIGINT, Signal::SIGTERM, Signal::SIGQUIT];
        signal::unblock(signals)?;
        info!("✅ Signals unblocked");
        Ok(())
    }
}