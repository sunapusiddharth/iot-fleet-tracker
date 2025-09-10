use crate::models::alert::{Alert, AlertType, AlertSeverity, AlertStatus, AlertSummary, AlertStats, AlertAction, ActionType};
use crate::server::storage::StorageManager;
use crate::server::realtime::RealtimeManager;
use uuid::Uuid;
use chrono::Utc;

pub struct AlertService {
    storage_manager: std::sync::Arc<tokio::sync::Mutex<StorageManager>>,
    realtime_manager: std::sync::Arc<tokio::sync::Mutex<RealtimeManager>>,
}

impl AlertService {
    pub fn new(
        storage_manager: std::sync::Arc<tokio::sync::Mutex<StorageManager>>,
        realtime_manager: std::sync::Arc<tokio::sync::Mutex<RealtimeManager>>,
    ) -> Self {
        Self {
            storage_manager,
            realtime_manager,
        }
    }

    pub async fn create_alert(&self, alert: Alert) -> Result<Alert, Box<dyn std::error::Error>> {
        // Store alert
        {
            let storage = self.storage_manager.lock().await;
            storage.store_alert(&alert).await?;
        }

        // Broadcast to realtime
        {
            let realtime = self.realtime_manager.lock().await;
            realtime.broadcast_alert(&alert).await?;
        }

        Ok(alert)
    }

    pub async fn get_alert(&self, id: Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let storage = self.storage_manager.lock().await;
        let document_store = storage.get_document_store().await;
        let mut store = document_store.lock().await;
        
        store.get_alert(id).await
    }

    pub async fn acknowledge_alert(&self, id: Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let mut alert = self.get_alert(id).await?;
        alert.status = AlertStatus::Acknowledged;
        alert.acknowledged_at = Some(Utc::now());
        alert.updated_at = Utc::now();

        {
            let storage = self.storage_manager.lock().await;
            storage.store_alert(&alert).await?;
        }

        // Broadcast to realtime
        {
            let realtime = self.realtime_manager.lock().await;
            realtime.broadcast_alert(&alert).await?;
        }

        Ok(alert)
    }

    pub async fn resolve_alert(&self, id: Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let mut alert = self.get_alert(id).await?;
        alert.status = AlertStatus::Resolved;
        alert.resolved_at = Some(Utc::now());
        alert.updated_at = Utc::now();

        {
            let storage = self.storage_manager.lock().await;
            storage.store_alert(&alert).await?;
        }

        // Broadcast to realtime
        {
            let realtime = self.realtime_manager.lock().await;
            realtime.broadcast_alert(&alert).await?;
        }

        Ok(alert)
    }

    pub async fn list_alerts(&self, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let storage = self.storage_manager.lock().await;
        let document_store = storage.get_document_store().await;
        let mut store = document_store.lock().await;
        
        store.list_alerts(limit, offset).await
    }

    pub async fn get_truck_alerts(&self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let storage = self.storage_manager.lock().await;
        let document_store = storage.get_document_store().await;
        let mut store = document_store.lock().await;
        
        store.get_truck_alerts(truck_id, limit, offset).await
    }

    pub async def get_active_alerts(&self, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let storage = self.storage_manager.lock().await;
        let document_store = storage.get_document_store().await;
        let mut store = document_store.lock().await;
        
        store.get_active_alerts(limit, offset).await
    }

    pub async fn get_alert_stats(&self) -> Result<AlertStats, Box<dyn std::error::Error>> {
        let storage = self.storage_manager.lock().await;
        let document_store = storage.get_document_store().await;
        let mut store = document_store.lock().await;
        
        store.get_alert_stats().await
    }

    pub async fn get_alerts_by_type(&self, alert_type: AlertType, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let storage = self.storage_manager.lock().await;
        let document_store = storage.get_document_store().await;
        let mut store = document_store.lock().await;
        
        store.get_alerts_by_type(alert_type, limit, offset).await
    }

    pub async fn get_alerts_by_severity(&self, severity: AlertSeverity, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let storage = self.storage_manager.lock().await;
        let document_store = storage.get_document_store().await;
        let mut store = document_store.lock().await;
        
        store.get_alerts_by_severity(severity, limit, offset).await
    }
}

use crate::models::alert::{Alert, AlertType, AlertSeverity, AlertStatus, AlertSummary, AlertStats, AlertAction, ActionType};
use crate::server::storage::StorageManager;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct AlertService {
    storage_manager: Arc<StorageManager>,
}

impl AlertService {
    pub fn new(storage_manager: Arc<StorageManager>) -> Self {
        Self {
            storage_manager,
        }
    }

    pub async fn create_alert(&self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_alert(alert).await?;

        // Broadcast alert via realtime manager
        // self.realtime_manager.broadcast_alert(alert).await?;

        metrics::counter!("alerts_created_total", "type" => format!("{:?}", alert.alert_type), "severity" => format!("{:?}", alert.severity)).increment(1);
        tracing::info!(alert_id=%alert.id, truck_id=%alert.truck_id, "🚨 Alert created");

        Ok(())
    }

    pub async fn get_alert(&self, id: &Uuid) -> Result<Option<Alert>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alert = store.get_alert(id).await?;
        
        if let Some(ref a) = alert {
            metrics::counter!("alerts_retrieved_total").increment(1);
            tracing::info!(alert_id=%id, "📤 Alert retrieved");
        }

        Ok(alert)
    }

    pub async fn acknowledge_alert(&self, id: &Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let mut alert = store.get_alert(id).await?
            .ok_or_else(|| "Alert not found".to_string())?;

        alert.status = AlertStatus::Acknowledged;
        alert.acknowledged_at = Some(Utc::now());
        alert.updated_at = Utc::now();

        store.store_alert(&alert).await?;

        metrics::counter!("alerts_acknowledged_total").increment(1);
        tracing::info!(alert_id=%id, "✅ Alert acknowledged");

        Ok(alert)
    }

    pub async fn resolve_alert(&self, id: &Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let mut alert = store.get_alert(id).await?
            .ok_or_else(|| "Alert not found".to_string())?;

        alert.status = AlertStatus::Resolved;
        alert.resolved_at = Some(Utc::now());
        alert.updated_at = Utc::now();

        store.store_alert(&alert).await?;

        metrics::counter!("alerts_resolved_total").increment(1);
        tracing::info!(alert_id=%id, "✅ Alert resolved");

        Ok(alert)
    }

    pub async fn list_alerts(&self, params: AlertListParams) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alerts = store.list_alerts(params).await?;
        
        metrics::counter!("alerts_listed_total").increment(1);
        tracing::info!(count=%alerts.len(), "📤 Alerts listed");

        Ok(alerts)
    }

    pub async fn get_alert_stats(&self) -> Result<AlertStats, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let stats = store.get_alert_stats().await?;
        
        metrics::counter!("alert_stats_retrieved_total").increment(1);
        tracing::info!("📊 Alert stats retrieved");

        Ok(stats)
    }

    pub async fn get_truck_alerts(&self, truck_id: &Uuid, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alerts = store.get_truck_alerts(truck_id, limit, offset).await?;
        
        metrics::counter!("truck_alerts_retrieved_total").increment(1);
        tracing::info!(truck_id=%truck_id, count=%alerts.len(), "📤 Truck alerts retrieved");

        Ok(alerts)
    }
}

#[derive(Debug, Clone)]
pub struct AlertListParams {
    pub limit: i64,
    pub offset: i64,
    pub severity: Option<AlertSeverity>,
    pub alert_type: Option<AlertType>,
    pub status: Option<AlertStatus>,
    pub truck_id: Option<Uuid>,
    pub start_time: Option<chrono::DateTime<Utc>>,
    pub end_time: Option<chrono::DateTime<Utc>>,
}


use crate::models::alert::{Alert, AlertSummary, AlertStats, AlertType, AlertSeverity, AlertStatus};
use crate::server::storage::StorageManager;
use crate::server::realtime::RealtimeManager;
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::sync::Arc;

pub struct AlertService {
    storage_manager: Arc<StorageManager>,
    realtime_manager: Arc<RealtimeManager>,
}

impl AlertService {
    pub fn new(storage_manager: Arc<StorageManager>, realtime_manager: Arc<RealtimeManager>) -> Self {
        Self {
            storage_manager,
            realtime_manager,
        }
    }

    pub async fn create_alert(&self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_alert(alert).await?;
        
        // Broadcast to real-time subscribers
        self.realtime_manager.broadcast_alert(alert).await?;
        
        Ok(())
    }

    pub async fn get_alert(&self, id: Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alert = store.get_alert(id).await?;
        
        Ok(alert)
    }

    pub async fn acknowledge_alert(&self, id: Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let mut alert = store.get_alert(id).await?;
        alert.status = AlertStatus::Acknowledged;
        alert.acknowledged_at = Some(Utc::now());
        alert.updated_at = Utc::now();
        
        store.update_alert(&alert).await?;
        
        // Broadcast update
        self.realtime_manager.broadcast_alert(&alert).await?;
        
        Ok(alert)
    }

    pub async fn resolve_alert(&self, id: Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let mut alert = store.get_alert(id).await?;
        alert.status = AlertStatus::Resolved;
        alert.resolved_at = Some(Utc::now());
        alert.updated_at = Utc::now();
        
        store.update_alert(&alert).await?;
        
        // Broadcast update
        self.realtime_manager.broadcast_alert(&alert).await?;
        
        Ok(alert)
    }

    pub async fn list_alerts(&self, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alerts = store.list_alerts(limit, offset).await?;
        
        Ok(alerts)
    }

    pub async fn get_truck_alerts(&self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alerts = store.get_truck_alerts(truck_id, limit, offset).await?;
        
        Ok(alerts)
    }

    pub async fn get_alerts_by_type(&self, alert_type: AlertType, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alerts = store.get_alerts_by_type(alert_type, limit, offset).await?;
        
        Ok(alerts)
    }

    pub async fn get_alerts_by_severity(&self, severity: AlertSeverity, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alerts = store.get_alerts_by_severity(severity, limit, offset).await?;
        
        Ok(alerts)
    }

    pub async fn get_alerts_by_status(&self, status: AlertStatus, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alerts = store.get_alerts_by_status(status, limit, offset).await?;
        
        Ok(alerts)
    }

    pub async fn get_alerts_by_time_range(&self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let alerts = store.get_alerts_by_time_range(start_time, end_time, limit, offset).await?;
        
        Ok(alerts)
    }

    pub async fn get_alert_stats(&self) -> Result<AlertStats, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let stats = store.get_alert_stats().await?;
        
        Ok(stats)
    }

    pub async fn get_alerts_for_dashboard(&self, hours: i32) -> Result<DashboardAlerts, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let end_time = Utc::now();
        let start_time = end_time - Duration::hours(hours as i64);
        
        let recent_alerts = store.get_alerts_by_time_range(start_time, end_time, 100, 0).await?;
        let stats = store.get_alert_stats().await?;
        
        // Get alerts by truck for top 10 trucks
        let top_trucks = store.get_top_alert_trucks(10).await?;
        
        // Get alerts by type for pie chart
        let alerts_by_type = store.get_alerts_by_type_grouped().await?;
        
        // Get alerts by severity for donut chart
        let alerts_by_severity = store.get_alerts_by_severity_grouped().await?;
        
        Ok(DashboardAlerts {
            recent_alerts,
            stats,
            top_trucks,
            alerts_by_type,
            alerts_by_severity,
            time_range_hours: hours,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashboardAlerts {
    pub recent_alerts: Vec<AlertSummary>,
    pub stats: AlertStats,
    pub top_trucks: Vec<(Uuid, i64)>,
    pub alerts_by_type: std::collections::HashMap<String, i64>,
    pub alerts_by_severity: std::collections::HashMap<String, i64>,
    pub time_range_hours: i32,
}