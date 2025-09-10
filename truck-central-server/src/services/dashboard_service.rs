use crate::services::{truck_service::TruckService, alert_service::AlertService, ml_service::MlService, health_service::HealthService, ota_service::OtaService};
use crate::models::truck::TruckSummary;
use crate::models::alert::AlertSummary;
use crate::models::ml::MlEventSummary;
use crate::models::health::HealthSummary;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct DashboardService {
    truck_service: Arc<TruckService>,
    alert_service: Arc<AlertService>,
    ml_service: Arc<MlService>,
    health_service: Arc<HealthService>,
    ota_service: Arc<OtaService>,
}

impl DashboardService {
    pub fn new(
        truck_service: Arc<TruckService>,
        alert_service: Arc<AlertService>,
        ml_service: Arc<MlService>,
        health_service: Arc<HealthService>,
        ota_service: Arc<OtaService>,
    ) -> Self {
        Self {
            truck_service,
            alert_service,
            ml_service,
            health_service,
            ota_service,
        }
    }

    pub async fn get_dashboard_summary(&self) -> Result<DashboardSummary, Box<dyn std::error::Error>> {
        // Get truck summary
        let trucks = self.truck_service.list_trucks(1000, 0).await?;
        
        let online_trucks = trucks.iter().filter(|t| t.status == crate::models::truck::TruckStatus::Online).count();
        let offline_trucks = trucks.iter().filter(|t| t.status == crate::models::truck::TruckStatus::Offline).count();
        let maintenance_trucks = trucks.iter().filter(|t| t.status == crate::models::truck::TruckStatus::Maintenance).count();
        
        // Get alert stats
        let alert_stats = self.alert_service.get_alert_stats().await?;
        
        // Get ML stats
        let ml_stats = self.ml_service.get_ml_stats().await?;
        
        // Get health stats
        let health_stats = self.health_service.get_health_stats().await?;
        
        // Get OTA stats
        let ota_stats = self.ota_service.get_ota_stats().await?;
        
        // Get recent alerts
        let recent_alerts = self.alert_service.list_alerts(10, 0).await?;
        
        // Get recent ML events
        let recent_ml_events = self.ml_service.list_ml_events(10, 0).await?;
        
        // Get recent health status
        let recent_health_status = self.health_service.list_health_status(10, 0).await?;
        
        Ok(DashboardSummary {
            total_trucks: trucks.len() as i64,
            online_trucks: online_trucks as i64,
            offline_trucks: offline_trucks as i64,
            maintenance_trucks: maintenance_trucks as i64,
            alert_stats,
            ml_stats,
            health_stats,
            ota_stats,
            recent_alerts,
            recent_ml_events,
            recent_health_status,
            last_updated: Utc::now(),
        })
    }

    pub async fn get_trucks_summary(&self, limit: i64, offset: i64) -> Result<TrucksSummary, Box<dyn std::error::Error>> {
        let trucks = self.truck_service.list_trucks(limit, offset).await?;
        let total_trucks = self.truck_service.list_trucks(10000, 0).await?.len() as i64;
        
        // Get counts by status
        let online_count = trucks.iter().filter(|t| t.status == crate::models::truck::TruckStatus::Online).count() as i64;
        let offline_count = trucks.iter().filter(|t| t.status == crate::models::truck::TruckStatus::Offline).count() as i64;
        let maintenance_count = trucks.iter().filter(|t| t.status == crate::models::truck::TruckStatus::Maintenance).count() as i64;
        
        // Get average health score
        let avg_health_score = trucks.iter().map(|t| t.health_score).sum::<f32>() / trucks.len() as f32;
        
        // Get trucks with active alerts
        let trucks_with_alerts = trucks.iter().filter(|t| t.active_alerts > 0).count() as i64;
        
        Ok(TrucksSummary {
            trucks,
            total_count: total_trucks,
            online_count,
            offline_count,
            maintenance_count,
            avg_health_score,
            trucks_with_alerts,
            limit,
            offset,
        })
    }

    pub async fn get_alerts_summary(&self, hours: i32) -> Result<AlertsSummary, Box<dyn std::error::Error>> {
        let dashboard_alerts = self.alert_service.get_alerts_for_dashboard(hours).await?;
        
        Ok(AlertsSummary {
            recent_alerts: dashboard_alerts.recent_alerts,
            stats: dashboard_alerts.stats,
            top_trucks: dashboard_alerts.top_trucks,
            alerts_by_type: dashboard_alerts.alerts_by_type,
            alerts_by_severity: dashboard_alerts.alerts_by_severity,
            time_range_hours: dashboard_alerts.time_range_hours,
        })
    }

    pub async fn get_ml_summary(&self, hours: i32) -> Result<MlSummary, Box<dyn std::error::Error>> {
        let dashboard_ml_events = self.ml_service.get_ml_events_for_dashboard(hours).await?;
        
        Ok(MlSummary {
            recent_ml_events: dashboard_ml_events.recent_ml_events,
            stats: dashboard_ml_events.stats,
            top_models: dashboard_ml_events.top_models,
            events_by_result: dashboard_ml_events.events_by_result,
            top_trucks: dashboard_ml_events.top_trucks,
            time_range_hours: dashboard_ml_events.time_range_hours,
        })
    }

    pub async fn get_health_summary(&self, hours: i32) -> Result<HealthSummary, Box<dyn std::error::Error>> {
        let dashboard_health_status = self.health_service.get_health_status_for_dashboard(hours).await?;
        
        Ok(HealthSummary {
            recent_health_status: dashboard_health_status.recent_health_status,
            stats: dashboard_health_status.stats,
            status_by_type: dashboard_health_status.status_by_type,
            top_trucks: dashboard_health_status.top_trucks,
            avg_resources: dashboard_health_status.avg_resources,
            time_range_hours: dashboard_health_status.time_range_hours,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashboardSummary {
    pub total_trucks: i64,
    pub online_trucks: i64,
    pub offline_trucks: i64,
    pub maintenance_trucks: i64,
    pub alert_stats: crate::models::alert::AlertStats,
    pub ml_stats: crate::models::ml::MlStats,
    pub health_stats: crate::models::health::HealthStats,
    pub ota_stats: crate::models::ota::OtaStats,
    pub recent_alerts: Vec<AlertSummary>,
    pub recent_ml_events: Vec<MlEventSummary>,
    pub recent_health_status: Vec<HealthSummary>,
    pub last_updated: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrucksSummary {
    pub trucks: Vec<TruckSummary>,
    pub total_count: i64,
    pub online_count: i64,
    pub offline_count: i64,
    pub maintenance_count: i64,
    pub avg_health_score: f32,
    pub trucks_with_alerts: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlertsSummary {
    pub recent_alerts: Vec<AlertSummary>,
    pub stats: crate::models::alert::AlertStats,
    pub top_trucks: Vec<(Uuid, i64)>,
    pub alerts_by_type: std::collections::HashMap<String, i64>,
    pub alerts_by_severity: std::collections::HashMap<String, i64>,
    pub time_range_hours: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MlSummary {
    pub recent_ml_events: Vec<MlEventSummary>,
    pub stats: crate::models::ml::MlStats,
    pub top_models: Vec<(String, i64)>,
    pub events_by_result: std::collections::HashMap<String, i64>,
    pub top_trucks: Vec<(Uuid, i64)>,
    pub time_range_hours: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthSummary {
    pub recent_health_status: Vec<crate::models::health::HealthSummary>,
    pub stats: crate::models::health::HealthStats,
    pub status_by_type: std::collections::HashMap<String, i64>,
    pub top_trucks: Vec<(Uuid, i64)>,
    pub avg_resources: crate::services::health_service::AverageResources,
    pub time_range_hours: i32,
}