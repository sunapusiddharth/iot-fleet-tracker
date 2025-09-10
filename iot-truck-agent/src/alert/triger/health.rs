use crate::alert::types::{Alert, AlertSeverity, AlertType};
use crate::health::types::HealthEvent;
use tracing::{info, warn};

pub struct HealthTriggerEngine;

impl HealthTriggerEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn trigger_from_health(&self, health_event: &HealthEvent) -> Vec<Alert> {
        let mut alerts = Vec::new();

        if health_event.resources.temperature_c > 80.0 {
            alerts.push(Alert::new(
                AlertType::HighTemperature,
                AlertSeverity::Emergency,
                "System temperature critical - shutdown imminent",
                &health_event.meta.device_id,
            ));
        } else if health_event.resources.temperature_c > 70.0 {
            alerts.push(Alert::new(
                AlertType::HighTemperature,
                AlertSeverity::Critical,
                "System temperature high - reduce load",
                &health_event.meta.device_id,
            ));
        }

        if health_event.resources.disk_percent > 95.0 {
            alerts.push(Alert::new(
                AlertType::LowDiskSpace,
                AlertSeverity::Critical,
                "Disk space critical - deleting old data",
                &health_event.meta.device_id,
            ));
        } else if health_event.resources.disk_percent > 90.0 {
            alerts.push(Alert::new(
                AlertType::LowDiskSpace,
                AlertSeverity::Warning,
                "Disk space low - consider cleanup",
                &health_event.meta.device_id,
            ));
        }

        if health_event.resources.cpu_percent > 95.0 {
            alerts.push(Alert::new(
                AlertType::HighCpuUsage,
                AlertSeverity::Critical,
                "CPU usage critical - system may become unresponsive",
                &health_event.meta.device_id,
            ));
        } else if health_event.resources.cpu_percent > 85.0 {
            alerts.push(Alert::new(
                AlertType::HighCpuUsage,
                AlertSeverity::Warning,
                "CPU usage high - consider reducing load",
                &health_event.meta.device_id,
            ));
        }

        for alert_info in &health_event.alerts {
            if alert_info.severity == crate::health::types::AlertSeverity::Critical {
                alerts.push(Alert::new(
                    AlertType::SensorFailure,
                    AlertSeverity::Critical,
                    &alert_info.message,
                    &health_event.meta.device_id,
                ));
            }
        }

        alerts
    }
}
