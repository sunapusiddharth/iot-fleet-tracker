use crate::models::ml::{MlEvent, MlResult, HardwareType, MlEventMetadata, SensorContext, MlEventSummary, MlStats};
use crate::server::storage::StorageManager;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct MlService {
    storage_manager: Arc<StorageManager>,
}

impl MlService {
    pub fn new(storage_manager: Arc<StorageManager>) -> Self {
        Self {
            storage_manager,
        }
    }

    pub async fn store_ml_event(&self, ml_event: &MlEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_ml_event(ml_event).await?;

        metrics::counter!("ml_events_stored_total", "model" => ml_event.model_name.clone(), "hardware" => format!("{:?}", ml_event.hardware_used)).increment(1);
        tracing::info!(event_id=%ml_event.id, truck_id=%ml_event.truck_id, "🧠 ML event stored");

        Ok(())
    }

    pub async fn get_ml_event(&self, id: &Uuid) -> Result<Option<MlEvent>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ml_event = store.get_ml_event(id).await?;
        
        if let Some(ref m) = ml_event {
            metrics::counter!("ml_events_retrieved_total").increment(1);
            tracing::info!(event_id=%id, "📤 ML event retrieved");
        }

        Ok(ml_event)
    }

    pub async fn list_ml_events(&self, params: MlEventListParams) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let events = store.list_ml_events(params).await?;
        
        metrics::counter!("ml_events_listed_total").increment(1);
        tracing::info!(count=%events.len(), "📤 ML events listed");

        Ok(events)
    }

    pub async fn get_ml_stats(&self) -> Result<MlStats, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let stats = store.get_ml_stats().await?;
        
        metrics::counter!("ml_stats_retrieved_total").increment(1);
        tracing::info!("📊 ML stats retrieved");

        Ok(stats)
    }

    pub async fn get_truck_ml_events(&self, truck_id: &Uuid, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let events = store.get_truck_ml_events(truck_id, limit, offset).await?;
        
        metrics::counter!("truck_ml_events_retrieved_total").increment(1);
        tracing::info!(truck_id=%truck_id, count=%events.len(), "📤 Truck ML events retrieved");

        Ok(events)
    }

    pub async fn is_alert_event(&self, ml_event: &MlEvent) -> bool {
        match &ml_event.result {
            MlResult::Drowsiness { is_drowsy, eye_closure_ratio: _ } => {
                *is_drowsy && ml_event.confidence > 0.8
            }
            MlResult::LaneDeparture { is_departing, deviation_pixels: _ } => {
                *is_departing && ml_event.confidence > 0.7
            }
            MlResult::CargoTamper { is_tampered, motion_score: _ } => {
                *is_tampered && ml_event.confidence > 0.8
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MlEventListParams {
    pub limit: i64,
    pub offset: i64,
    pub model_name: Option<String>,
    pub result_type: Option<String>,
    pub is_alert: Option<bool>,
    pub truck_id: Option<Uuid>,
    pub start_time: Option<chrono::DateTime<Utc>>,
    pub end_time: Option<chrono::DateTime<Utc>>,
    pub min_confidence: Option<f32>,
    pub max_latency_ms: Option<f32>,
}
use crate::models::ml::{MlEvent, MlEventSummary, MlStats, MlResult};
use crate::server::storage::StorageManager;
use crate::server::realtime::RealtimeManager;
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::sync::Arc;

pub struct MlService {
    storage_manager: Arc<StorageManager>,
    realtime_manager: Arc<RealtimeManager>,
}

impl MlService {
    pub fn new(storage_manager: Arc<StorageManager>, realtime_manager: Arc<RealtimeManager>) -> Self {
        Self {
            storage_manager,
            realtime_manager,
        }
    }

    pub async fn create_ml_event(&self, ml_event: &MlEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_ml_event(ml_event).await?;
        
        // Broadcast to real-time subscribers
        self.realtime_manager.broadcast_ml_event(ml_event).await?;
        
        Ok(())
    }

    pub async fn get_ml_event(&self, id: Uuid) -> Result<MlEvent, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ml_event = store.get_ml_event(id).await?;
        
        Ok(ml_event)
    }

    pub async fn list_ml_events(&self, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ml_events = store.list_ml_events(limit, offset).await?;
        
        Ok(ml_events)
    }

    pub async fn get_truck_ml_events(&self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ml_events = store.get_truck_ml_events(truck_id, limit, offset).await?;
        
        Ok(ml_events)
    }

    pub async fn get_ml_events_by_model(&self, model_name: &str, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ml_events = store.get_ml_events_by_model(model_name, limit, offset).await?;
        
        Ok(ml_events)
    }

    pub async fn get_ml_events_by_result_type(&self, result_type: &str, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ml_events = store.get_ml_events_by_result_type(result_type, limit, offset).await?;
        
        Ok(ml_events)
    }

    pub async fn get_ml_events_by_time_range(&self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ml_events = store.get_ml_events_by_time_range(start_time, end_time, limit, offset).await?;
        
        Ok(ml_events)
    }

    pub async fn get_ml_stats(&self) -> Result<MlStats, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let stats = store.get_ml_stats().await?;
        
        Ok(stats)
    }

    pub async fn get_ml_events_for_dashboard(&self, hours: i32) -> Result<DashboardMlEvents, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let end_time = Utc::now();
        let start_time = end_time - Duration::hours(hours as i64);
        
        let recent_ml_events = store.get_ml_events_by_time_range(start_time, end_time, 100, 0).await?;
        let stats = store.get_ml_stats().await?;
        
        // Get ML events by model for top 5 models
        let top_models = store.get_top_ml_models(5).await?;
        
        // Get ML events by result type for pie chart
        let events_by_result = store.get_ml_events_by_result_grouped().await?;
        
        // Get ML events by truck for top 10 trucks
        let top_trucks = store.get_top_ml_trucks(10).await?;
        
        Ok(DashboardMlEvents {
            recent_ml_events,
            stats,
            top_models,
            events_by_result,
            top_trucks,
            time_range_hours: hours,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashboardMlEvents {
    pub recent_ml_events: Vec<MlEventSummary>,
    pub stats: MlStats,
    pub top_models: Vec<(String, i64)>,
    pub events_by_result: std::collections::HashMap<String, i64>,
    pub top_trucks: Vec<(Uuid, i64)>,
    pub time_range_hours: i32,
}