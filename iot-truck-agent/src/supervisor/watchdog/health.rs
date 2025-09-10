use crate::supervisor::types::{ModuleState, ModuleStatus};
use tokio_metrics::TaskMonitor;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use tracing::{error, warn};

pub struct ModuleHealthMonitor {
    monitors: HashMap<String, TaskMonitor>,
    last_seen: HashMap<String, Instant>,
    restarts: HashMap<String, u32>,
    last_restart: HashMap<String, Option<Instant>>,
    status: HashMap<String, ModuleStatus>,
}

impl ModuleHealthMonitor {
    pub fn new() -> Self {
        Self {
            monitors: HashMap::new(),
            last_seen: HashMap::new(),
            restarts: HashMap::new(),
            last_restart: HashMap::new(),
            status: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, name: &str) -> TaskMonitor {
        let monitor = TaskMonitor::new();
        self.monitors.insert(name.to_string(), monitor.clone());
        self.last_seen.insert(name.to_string(), Instant::now());
        self.restarts.entry(name.to_string()).or_insert(0);
        self.last_restart.insert(name.to_string(), None);
        self.status.insert(name.to_string(), ModuleStatus::Starting);
        monitor
    }

    pub fn heartbeat(&mut self, module_name: &str) {
        if self.last_seen.contains_key(module_name) {
            self.last_seen.insert(module_name.to_string(), Instant::now());
            self.status.insert(module_name.to_string(), ModuleStatus::Running);
        }
    }

    pub fn set_status(&mut self, module_name: &str, status: ModuleStatus) {
        self.status.insert(module_name.to_string(), status);
    }

    pub fn restart_module(&mut self, module_name: &str) {
        *self.restarts.entry(module_name.to_string()).or_insert(0) += 1;
        self.last_restart.insert(module_name.to_string(), Some(Instant::now()));
        self.last_seen.insert(module_name.to_string(), Instant::now());
        self.status.insert(module_name.to_string(), ModuleStatus::Restarting);
    }

    pub fn check_modules(&mut self) -> Vec<ModuleState> {
        let mut modules = Vec::new();
        let now = Instant::now();

        for (name, monitor) in &self.monitors {
            let last_seen = self.last_seen.get(name).unwrap_or(&now);
            let is_alive = now.duration_since(*last_seen) < Duration::from_secs(60);
            let restarts = *self.restarts.get(name).unwrap_or(&0);
            let last_restart = self.last_restart.get(name).unwrap_or(&None).map(|i| i.elapsed().as_secs());
            let status = self.status.get(name).unwrap_or(&ModuleStatus::Running).clone();

            let stats = monitor.cumulative();
            let cpu_usage_percent = if stats.total_duration().as_secs() > 0 {
                stats.busy_duration().as_secs_f32() / stats.total_duration().as_secs_f32() * 100.0
            } else {
                0.0
            };
            let memory_usage_mb = stats.total_memory() / 1024 / 1024;

            // Update status if not alive
            if !is_alive && status == ModuleStatus::Running {
                self.status.insert(name.clone(), ModuleStatus::Failed);
                warn!(module=%name, "⚠️  Module not responding - marked as failed");
            }

            modules.push(ModuleState {
                name: name.clone(),
                status,
                last_heartbeat: last_seen.elapsed().as_millis() as u64,
                restarts,
                last_restart: last_restart.map(|s| s as u64),
                cpu_usage_percent,
                memory_usage_mb,
            });
        }

        modules
    }

    pub fn get_module_status(&self, module_name: &str) -> Option<ModuleStatus> {
        self.status.get(module_name).cloned()
    }

    pub fn is_any_module_failed(&self) -> bool {
        self.status.values().any(|s| *s == ModuleStatus::Failed)
    }
}