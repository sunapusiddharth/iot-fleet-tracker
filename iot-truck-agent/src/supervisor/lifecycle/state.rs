use crate::supervisor::types::{SystemState, SystemStateType, ResourceUsage};
use std::sync::RwLock;
use once_cell::sync::Lazy;

static SYSTEM_STATE: Lazy<RwLock<SystemState>> = Lazy::new(|| {
    RwLock::new(SystemState::new("unknown"))
});

pub fn initialize_system_state(device_id: &str) {
    let mut state = SYSTEM_STATE.write().unwrap();
    *state = SystemState::new(device_id);
    tracing::info!(device_id=%device_id, "✅ System state initialized");
}

pub fn get_system_state() -> SystemState {
    SYSTEM_STATE.read().unwrap().clone()
}

pub fn update_system_state<F>(update_fn: F)
where
    F: FnOnce(&mut SystemState),
{
    let mut state = SYSTEM_STATE.write().unwrap();
    update_fn(&mut state);
}

pub fn set_system_state(state_type: SystemStateType) {
    update_system_state(|s| {
        s.state = state_type;
        tracing::info!(state=%format!("{:?}", state_type), "🔄 System state updated");
    });
}

pub fn set_shutdown_reason(reason: crate::supervisor::types::ShutdownReason) {
    update_system_state(|s| {
        s.meta.shutdown_reason = Some(format!("{:?}", reason));
        tracing::info!(reason=%format!("{:?}", reason), "🔄 Shutdown reason set");
    });
}

pub fn set_last_error(error: &str) {
    update_system_state(|s| {
        s.meta.last_error = Some(error.to_string());
        tracing::error!(error=%error, "❌ Last error set");
    });
}

pub fn update_resource_usage(cpu_percent: f32, memory_percent: f32, disk_percent: f32, temperature_c: f32) {
    update_system_state(|s| {
        s.resources.cpu_percent = cpu_percent;
        s.resources.memory_percent = memory_percent;
        s.resources.disk_percent = disk_percent;
        s.resources.temperature_c = temperature_c;
    });
}

pub fn update_module_state(module_name: &str, status: crate::supervisor::types::ModuleStatus, cpu_usage: f32, memory_usage: u64) {
    update_system_state(|s| {
        if let Some(module) = s.modules.iter_mut().find(|m| m.name == module_name) {
            module.status = status;
            module.cpu_usage_percent = cpu_usage;
            module.memory_usage_mb = memory_usage;
            module.last_heartbeat = chrono::Utc::now().timestamp_nanos() as u64;
        } else {
            s.modules.push(crate::supervisor::types::ModuleState {
                name: module_name.to_string(),
                status,
                last_heartbeat: chrono::Utc::now().timestamp_nanos() as u64,
                restarts: 0,
                last_restart: None,
                cpu_usage_percent: cpu_usage,
                memory_usage_mb: memory_usage,
            });
        }
    });
}