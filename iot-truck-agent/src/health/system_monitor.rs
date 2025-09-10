use crate::health::types::{ResourceUsage, AlertInfo, AlertSeverity};
use crate::health::config::HealthConfig;
use sysinfo::{System, SystemExt, CpuExt, ProcessExt, DiskExt};
use std::fs;
use tracing::{error, warn};

pub struct SystemMonitor {
    system: System,
    config: HealthConfig,
}

impl SystemMonitor {
    pub fn new(config: HealthConfig) -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system,
            config,
        }
    }

    pub fn collect(&mut self) -> Result<(ResourceUsage, Vec<AlertInfo>), Box<dyn std::error::Error>> {
        self.system.refresh_all();

        let cpu_percent = self.system.global_processor_info().cpu_usage();
        let memory_used = self.system.used_memory();
        let memory_total = self.system.total_memory();
        let memory_available = self.system.available_memory();
        let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;

        let swap_used = self.system.used_swap();
        let swap_total = self.system.total_swap();
        let swap_percent = if swap_total > 0 {
            (swap_used as f32 / swap_total as f32) * 100.0
        } else {
            0.0
        };

        let disks = self.system.disks();
        let disk = disks.first().unwrap_or(&disks[0]);
        let disk_used = disk.available_space();
        let disk_total = disk.total_space();
        let disk_available = disk.available_space();
        let disk_percent = ((disk_total - disk_available) as f32 / disk_total as f32) * 100.0;

        let temperature_c = self.read_temperature()?;
        let thermal_throttling = self.check_thermal_throttling()?;

        let uptime_sec = self.system.uptime();
        let load_avg = self.system.load_average();

        let resources = ResourceUsage {
            cpu_percent,
            cpu_cores: self.system.cpus().len(),
            memory_percent,
            memory_used_mb: memory_used / 1024 / 1024,
            memory_total_mb: memory_total / 1024 / 1024,
            memory_available_mb: memory_available / 1024 / 1024,
            swap_percent,
            disk_percent,
            disk_used_gb: (disk_total - disk_available) / 1024 / 1024 / 1024,
            disk_total_gb: disk_total / 1024 / 1024 / 1024,
            disk_available_gb: disk_available / 1024 / 1024 / 1024,
            temperature_c,
            thermal_throttling,
            uptime_sec,
            load_average: (load_avg.one, load_avg.five, load_avg.fifteen),
        };

        let mut alerts = Vec::new();

        // CPU alerts
        if cpu_percent > self.config.thresholds.cpu_critical_percent {
            alerts.push(AlertInfo {
                alert_id: format!("cpu-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "cpu_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("CPU usage critical: {:.1}%", cpu_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "system_monitor".to_string(),
                recommended_action: "Reduce camera FPS, disable non-critical ML models".to_string(),
            });
        } else if cpu_percent > self.config.thresholds.cpu_warning_percent {
            alerts.push(AlertInfo {
                alert_id: format!("cpu-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "cpu_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("CPU usage warning: {:.1}%", cpu_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "system_monitor".to_string(),
                recommended_action: "Monitor for trends, consider degradation".to_string(),
            });
        }

        // Memory alerts
        if memory_percent > self.config.thresholds.memory_critical_percent {
            alerts.push(AlertInfo {
                alert_id: format!("mem-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "memory_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("Memory usage critical: {:.1}%", memory_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "system_monitor".to_string(),
                recommended_action: "Reduce frame buffer size, disable ML models".to_string(),
            });
        } else if memory_percent > self.config.thresholds.memory_warning_percent {
            alerts.push(AlertInfo {
                alert_id: format!("mem-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "memory_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Memory usage warning: {:.1}%", memory_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "system_monitor".to_string(),
                recommended_action: "Monitor memory trends".to_string(),
            });
        }

        // Disk alerts
        if disk_percent > self.config.thresholds.disk_critical_percent {
            alerts.push(AlertInfo {
                alert_id: format!("disk-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "disk_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("Disk usage critical: {:.1}%", disk_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "system_monitor".to_string(),
                recommended_action: "Force WAL checkpoint, drop camera frames".to_string(),
            });
        } else if disk_percent > self.config.thresholds.disk_warning_percent {
            alerts.push(AlertInfo {
                alert_id: format!("disk-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "disk_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Disk usage warning: {:.1}%", disk_percent),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "system_monitor".to_string(),
                recommended_action: "Monitor disk usage, prepare for checkpoint".to_string(),
            });
        }

        // Thermal alerts
        if temperature_c > self.config.thresholds.temp_critical_c {
            alerts.push(AlertInfo {
                alert_id: format!("temp-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "temp_critical".to_string(),
                severity: AlertSeverity::Critical,
                message: format!("Temperature critical: {:.1}°C", temperature_c),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "system_monitor".to_string(),
                recommended_action: "Reduce CPU load, check cooling, prepare for shutdown".to_string(),
            });
        } else if temperature_c > self.config.thresholds.temp_warning_c {
            alerts.push(AlertInfo {
                alert_id: format!("temp-{}", chrono::Utc::now().timestamp_nanos()),
                alert_type: "temp_warning".to_string(),
                severity: AlertSeverity::Warning,
                message: format!("Temperature warning: {:.1}°C", temperature_c),
                triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                source: "system_monitor".to_string(),
                recommended_action: "Monitor temperature trends".to_string(),
            });
        }

        Ok((resources, alerts))
    }

    fn read_temperature(&self) -> Result<f32, Box<dyn std::error::Error>> {
        // Try thermal zone
        let path = &self.config.thermal.thermal_zone_path;
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(temp) = content.trim().parse::<f32>() {
                return Ok(temp / 1000.0);
            }
        }

        // Try components
        for component in self.system.components() {
            if let Some(temp) = component.temperature() {
                return Ok(temp);
            }
        }

        // Estimate
        Ok(45.0 + (self.system.global_processor_info().cpu_usage() / 10.0))
    }

    fn check_thermal_throttling(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // Check for thermal throttling on Raspberry Pi
        if let Ok(content) = fs::read_to_string("/sys/devices/platform/soc/soc:firmware/get_throttled") {
            if let Ok(throttled) = u32::from_str_radix(content.trim(), 16) {
                return Ok(throttled & 0x50000 != 0); // Bit 18: throttling active
            }
        }
        Ok(false)
    }
}