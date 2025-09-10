use crate::health::types::{AlertInfo, AlertSeverity, TaskStatus};
use std::collections::HashMap;
use std::time::Instant;
use tokio_metrics::TaskMonitor;
use tracing::{error, warn};

pub struct TaskSupervisor {
    monitors: HashMap<String, TaskMonitor>,
    last_seen: HashMap<String, Instant>,
}

impl TaskSupervisor {
    pub fn new() -> Self {
        Self {
            monitors: HashMap::new(),
            last_seen: HashMap::new(),
        }
    }

    pub fn register_task(&mut self, name: &str) -> TaskMonitor {
        let monitor = TaskMonitor::new();
        self.monitors.insert(name.to_string(), monitor.clone());
        self.last_seen.insert(name.to_string(), Instant::now());
        monitor
    }

    pub fn heartbeat(&mut self, task_name: &str) {
        if self.last_seen.contains_key(task_name) {
            self.last_seen.insert(task_name.to_string(), Instant::now());
        }
    }

    pub fn check_tasks(&mut self) -> (Vec<TaskStatus>, Vec<AlertInfo>) {
        let mut tasks = Vec::new();
        let mut alerts = Vec::new();
        let now = Instant::now();

        for (name, monitor) in &self.monitors {
            let last_seen = self.last_seen.get(name).unwrap_or(&now);
            let is_alive = now.duration_since(*last_seen).as_secs() < 60; // 60s timeout

            let stats = monitor.cumulative();
            let cpu_usage_percent =
                stats.busy_duration().as_secs_f32() / stats.total_duration().as_secs_f32() * 100.0;
            let memory_usage_mb = stats.total_memory() / 1024 / 1024;

            tasks.push(TaskStatus {
                name: name.clone(),
                is_alive,
                last_seen_ms: last_seen.elapsed().as_millis() as u64,
                cpu_usage_percent,
                memory_usage_mb,
            });

            if !is_alive {
                alerts.push(AlertInfo {
                    alert_type: "task_dead".to_string(),
                    severity: AlertSeverity::Critical,
                    message: format!("Task {} is dead", name),
                    triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                });
            } else if cpu_usage_percent > 90.0 {
                alerts.push(AlertInfo {
                    alert_type: "task_cpu_high".to_string(),
                    severity: AlertSeverity::Warning,
                    message: format!("Task {} CPU usage high: {:.1}%", name, cpu_usage_percent),
                    triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                });
            }
        }

        (tasks, alerts)
    }
}
