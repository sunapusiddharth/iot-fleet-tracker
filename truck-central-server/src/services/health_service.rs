use crate::models::health::{HealthStatus, HealthSummary, HealthStats, HealthStatusType};
use crate::server::storage::StorageManager;
use crate::server::realtime::RealtimeManager;
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::sync::Arc;

pub struct HealthService {
    storage_manager: Arc<StorageManager>,
    realtime_manager: Arc<RealtimeManager>,
}

impl HealthService {
    pub fn new(storage_manager: Arc<StorageManager>, realtime_manager: Arc<RealtimeManager>) -> Self {
        Self {
            storage_manager,
            realtime_manager,
        }
    }

    pub async fn create_health_status(&self, health_status: &HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_health_status(health_status).await?;
        
        // Broadcast to real-time subscribers
        self.realtime_manager.broadcast_health_status(health_status).await?;
        
        Ok(())
    }

    pub async fn get_health_status(&self, id: Uuid) -> Result<HealthStatus, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let health_status = store.get_health_status(id).await?;
        
        Ok(health_status)
    }

    pub async fn list_health_status(&self, limit: i64, offset: i64) -> Result<Vec<HealthSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let health_status = store.list_health_status(limit, offset).await?;
        
        Ok(health_status)
    }

    pub async fn get_truck_health_status(&self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<HealthSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let health_status = store.get_truck_health_status(truck_id, limit, offset).await?;
        
        Ok(health_status)
    }

    pub async fn get_health_status_by_type(&self, status_type: HealthStatusType, limit: i64, offset: i64) -> Result<Vec<HealthSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let health_status = store.get_health_status_by_type(status_type, limit, offset).await?;
        
        Ok(health_status)
    }

    pub async fn get_health_status_by_time_range(&self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<HealthSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let health_status = store.get_health_status_by_time_range(start_time, end_time, limit, offset).await?;
        
        Ok(health_status)
    }

    pub async fn get_health_stats(&self) -> Result<HealthStats, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let stats = store.get_health_stats().await?;
        
        Ok(stats)
    }

    pub async fn get_health_status_for_dashboard(&self, hours: i32) -> Result<DashboardHealthStatus, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let end_time = Utc::now();
        let start_time = end_time - Duration::hours(hours as i64);
        
        let recent_health_status = store.get_health_status_by_time_range(start_time, end_time, 100, 0).await?;
        let stats = store.get_health_stats().await?;
        
        // Get health status by type for pie chart
        let status_by_type = store.get_health_status_by_type_grouped().await?;
        
        // Get health status by truck for top 10 trucks
        let top_trucks = store.get_top_health_trucks(10).await?;
        
        // Get average resource usage
        let avg_resources = store.get_average_resource_usage(start_time, end_time).await?;
        
        Ok(DashboardHealthStatus {
            recent_health_status,
            stats,
            status_by_type,
            top_trucks,
            avg_resources,
            time_range_hours: hours,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashboardHealthStatus {
    pub recent_health_status: Vec<HealthSummary>,
    pub stats: HealthStats,
    pub status_by_type: std::collections::HashMap<String, i64>,
    pub top_trucks: Vec<(Uuid, i64)>,
    pub avg_resources: AverageResources,
    pub time_range_hours: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AverageResources {
    pub avg_cpu_percent: f32,
    pub avg_memory_percent: f32,
    pub avg_disk_percent: f32,
    pub avg_temperature_c: f32,
}