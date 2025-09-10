use crate::health::types::ResourceUsage;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HealthIntegration {
    resource_usage: Arc<RwLock<ResourceUsage>>,
    is_throttled: std::sync::atomic::AtomicBool,
}

impl HealthIntegration {
    pub fn new(resource_usage: Arc<RwLock<ResourceUsage>>) -> Self {
        Self {
            resource_usage,
            is_throttled: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub async fn should_throttle_writes(&self) -> bool {
        let resources = self.resource_usage.read().await;

        let should_throttle = resources.disk_percent > 85.0
            || resources.temperature_c > 75.0
            || resources.memory_percent > 90.0;

        self.is_throttled
            .store(should_throttle, std::sync::atomic::Ordering::Relaxed);

        if should_throttle {
            tracing::warn!("🛑 WAL writes throttled due to system health");
            metrics::gauge!("wal_throttled").set(1.0);
        } else {
            metrics::gauge!("wal_throttled").set(0.0);
        }

        should_throttle
    }

    pub fn is_throttled(&self) -> bool {
        self.is_throttled.load(std::sync::atomic::Ordering::Relaxed)
    }
}
