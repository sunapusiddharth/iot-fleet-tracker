use crate::health::types::{TaskStatus, AlertInfo, AlertSeverity};
use tokio_metrics::TaskMonitor;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use tracing::{error, warn};

pub struct TaskSupervisor {
    monitors: HashMap<String, TaskMonitor>,
    last_seen: HashMap<String, Instant>,
    restarts: HashMap<String, u32>,
    last_restart: HashMap<String, Option<Instant>>,
}

impl TaskSupervisor {
    pub fn new() -> Self {
        Self {
            monitors: HashMap::new(),
            last_seen: HashMap::new(),
            restarts: HashMap::new(),
            last_restart: HashMap::new(),
        }
    }

    pub fn register_task(&mut self, name: &str) -> TaskMonitor {
        let monitor = TaskMonitor::new();
        self.monitors.insert(name.to_string(), monitor.clone());
        self.last_seen.insert(name.to_string(), Instant::now());
        self.restarts.entry(name.to_string()).or_insert(0);
        self.last_restart.insert(name.to_string(), None);
        monitor
    }

    pub fn heartbeat(&mut self, task_name: &str) {
        if self.last_seen.contains_key(task_name) {
            self.last_seen.insert(task_name.to_string(), Instant::now());
        }
    }

    pub fn restart_task(&mut self, task_name: &str) {
        *self.restarts.entry(task_name.to_string()).or_insert(0) += 1;
        self.last_restart.insert(task_name.to_string(), Some(Instant::now()));
        self.last_seen.insert(task_name.to_string(), Instant::now());
    }

    pub fn check_tasks(&mut self) -> (Vec<TaskStatus>, Vec<AlertInfo>) {
        let mut tasks = Vec::new();
        let mut alerts = Vec::new();
        let now = Instant::now();

        for (name, monitor) in &self.monitors {
            let last_seen = self.last_seen.get(name).unwrap_or(&now);
            let is_alive = now.duration_since(*last_seen) < Duration::from_secs(60);
            let restarts = *self.restarts.get(name).unwrap_or(&0);
            let last_restart = self.last_restart.get(name).unwrap_or(&None).map(|i| i.elapsed().as_secs());

            let stats = monitor.cumulative();
            let cpu_usage_percent = if stats.total_duration().as_secs() > 0 {
                stats.busy_duration().as_secs_f32() / stats.total_duration().as_secs_f32() * 100.0
            } else {
                0.0
            };
            let memory_usage_mb = stats.total_memory() / 1024 / 1024;

            tasks.push(TaskStatus {
                name: name.clone(),
                is_alive,
                last_seen_ms: last_seen.elapsed().as_millis() as u64,
                cpu_usage_percent,
                memory_usage_mb,
                restarts,
                last_restart: last_restart.map(|s| s as u64),
            });

            if !is_alive {
                alerts.push(AlertInfo {
                    alert_id: format!("task-dead-{}-{}", name, chrono::Utc::now().timestamp_nanos()),
                    alert_type: "task_dead".to_string(),
                    severity: AlertSeverity::Critical,
                    message: format!("Task {} is dead", name),
                    triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                    source: "task_supervisor".to_string(),
                    recommended_action: format!("Restart {} task, check logs", name),
                });
            } else if cpu_usage_percent > 90.0 {
                alerts.push(AlertInfo {
                    alert_id: format!("task-cpu-{}-{}", name, chrono::Utc::now().timestamp_nanos()),
                    alert_type: "task_cpu_high".to_string(),
                    severity: AlertSeverity::Warning,
                    message: format!("Task {} CPU usage high: {:.1}%", name, cpu_usage_percent),
                    triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                    source: "task_supervisor".to_string(),
                    recommended_action: format!("Optimize {} task, reduce load", name),
                });
            }

            if restarts > 5 && last_restart.map(|lr| lr < 300).unwrap_or(false) {
                alerts.push(AlertInfo {
                    alert_id: format!("task-flapping-{}-{}", name, chrono::Utc::now().timestamp_nanos()),
                    alert_type: "task_flapping".to_string(),
                    severity: AlertSeverity::Critical,
                    message: format!("Task {} restarting too frequently", name),
                    triggered_at: chrono::Utc::now().timestamp_nanos() as u64,
                    source: "task_supervisor".to_string(),
                    recommended_action: format!("Investigate root cause of {} crashes", name),
                });
            }
        }

        (tasks, alerts)
    }
}