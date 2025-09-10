use crate::health::types::{HealthEvent, HealthStatus, ResourceUsage, AlertInfo, AlertSeverity};
use crate::config::Config;
use sysinfo::{System, SystemExt, CpuExt, ProcessExt, DiskExt};
use tokio::sync::broadcast;
use tracing::{error, info, warn};

pub struct ResourceMonitor {
    system: System,
    config: Config,
}

impl ResourceMonitor {
    pub fn new(config: Config) -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system,
            config,
        }
    }

    pub fn collect(&mut self) -> Result<(ResourceUsage, Vec<AlertInfo>, HealthStatus), HealthError> {
        self.system.refresh_all();

        let cpu_percent = self.system.global_processor_info().cpu_usage();
        let memory_used = self.system.used_memory();
        let memory_total = self.system.total_memory();
        let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;

        let disks = self.system.disks();
        let disk = disks.first().unwrap_or(&disks[0]);
        let disk_used = disk.available_space();
        let disk_total = disk.total_space();
        let disk_percent = ((disk_total - disk_used) as f32 / disk_total as f32) * 100.0;

        let temperature_c = self.read_temperature().unwrap_or(45.0);
        let uptime_sec = self.system.uptime();
        let load_avg = self.system.load_average();

        let resources = ResourceUsage {
            cpu_percent,
            memory_percent,
            memory_used_mb: memory_used / 1024 / 1024,
            memory_total_mb: memory_total / 1024 / 1024,
            disk_percent,
            disk_used_gb: (disk_total - disk_used) / 1024 / 1024 / 1024,
            disk_total_gb: disk_total / 1024 / 1024 / 1024,
            temperature_c,
            uptime_sec,
            load_average: (load_avg.one, load_avg.five, load_avg.fifteen),
        };

        let mut alerts = Vec::new();
        let mut status = HealthStatus::Ok;

        // Check thresholds
        if cpu_percent > self.config.health.thresholds.cpu_critical_percent as f32 {
            alerts.push(AlertInfo {
                alert_type: "cpu_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("CPU usage critical: {:.1}%", cpu_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            });
            status = HealthStatus::Critical;
        } else if cpu_percent > self.config.health.thresholds.cpu_warning_percent as f32 {
            alerts.push(AlertInfo {
                alert_type: "cpu_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("CPU usage warning: {:.1}%", cpu_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            });
            if status == HealthStatus::Ok {
                status = HealthStatus::Warning;
            }
        }

        if memory_percent > self.config.health.thresholds.memory_critical_percent as f32 {
            alerts.push(AlertInfo {
                alert_type: "memory_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("Memory usage critical: {:.1}%", memory_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            });
            status = HealthStatus::Critical;
        } else if memory_percent > self.config.health.thresholds.memory_warning_percent as f32 {
            alerts.push(AlertInfo {
                alert_type: "memory_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Memory usage warning: {:.1}%", memory_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            });
            if status == HealthStatus::Ok {
                status = HealthStatus::Warning;
            }
        }

        if disk_percent > self.config.health.thresholds.disk_critical_percent as f32 {
            alerts.push(AlertInfo {
                alert_type: "disk_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("Disk usage critical: {:.1}%", disk_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            });
            status = HealthStatus::Critical;
        } else if disk_percent > self.config.health.thresholds.disk_warning_percent as f32 {
            alerts.push(AlertInfo {
                alert_type: "disk_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Disk usage warning: {:.1}%", disk_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            });
            if status == HealthStatus::Ok {
                status = HealthStatus::Warning;
            }
        }

        if temperature_c > self.config.health.thresholds.temp_critical_c as f32 {
            alerts.push(AlertInfo {
                alert_type: "temp_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("Temperature critical: {:.1}°C", temperature_c),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            });
            status = HealthStatus::Critical;
        } else if temperature_c > self.config.health.thresholds.temp_warning_c as f32 {
            alerts.push(AlertInfo {
                alert_type: "temp_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Temperature warning: {:.1}°C", temperature_c),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
            });
            if status == HealthStatus::Ok {
                status = HealthStatus::Warning;
            }
        }

        Ok((resources, alerts, status))
    }

    fn read_temperature(&self) -> Result<f32, HealthError> {
        // Try to read from thermal zone (Linux)
        if let Ok(content) = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
            if let Ok(temp) = content.trim().parse::<f32>() {
                return Ok(temp / 1000.0);
            }
        }

        // Fallback to CPU temp if available
        for component in self.system.components() {
            if let Some(temp) = component.temperature() {
                return Ok(temp);
            }
        }

        // Last resort: estimate from CPU usage
        Ok(45.0 + (self.system.global_processor_info().cpu_usage() / 10.0))
    }
}