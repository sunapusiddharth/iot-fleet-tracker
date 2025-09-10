use axum::{
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use std::net::SocketAddr;
use tokio::sync::Arc;
use uuid::Uuid;

pub mod alerts;
pub mod dashboard;
pub mod health;
pub mod ml;
pub mod ota;
pub mod telemetry;
pub mod trucks;

pub fn create_router(
    storage_manager: Arc<crate::server::storage::StorageManager>,
    realtime_manager: Arc<crate::server::realtime::RealtimeManager>,
) -> Router {
    Router::new()
        .route(
            "/api/trucks",
            get(trucks::list_trucks).post(trucks::create_truck),
        )
        .route(
            "/api/trucks/:id",
            get(trucks::get_truck)
                .put(trucks::update_truck)
                .delete(trucks::delete_truck),
        )
        .route(
            "/api/trucks/:id/telemetry",
            get(telemetry::get_truck_telemetry),
        )
        .route("/api/trucks/:id/alerts", get(alerts::get_truck_alerts))
        .route("/api/trucks/:id/ml-events", get(ml::get_truck_ml_events))
        .route("/api/trucks/:id/health", get(health::get_truck_health))
        .route("/api/trucks/:id/trips", get(trucks::get_truck_trips))
        .route("/api/alerts", get(alerts::list_alerts))
        .route(
            "/api/alerts/:id",
            get(alerts::get_alert).put(alerts::acknowledge_alert),
        )
        .route("/api/ml-events", get(ml::list_ml_events))
        .route("/api/health", get(health::list_health_status))
        .route(
            "/api/ota/updates",
            get(ota::list_updates).post(ota::create_update),
        )
        .route(
            "/api/ota/updates/:id",
            get(ota::get_update).put(ota::update_update),
        )
        .route("/api/ota/commands", post(ota::create_command))
        .route(
            "/api/dashboard/summary",
            get(dashboard::get_dashboard_summary),
        )
        .route("/api/dashboard/alerts", get(dashboard::get_alerts_summary))
        .route("/api/dashboard/trucks", get(dashboard::get_trucks_summary))
        .route("/api/dashboard/ml", get(dashboard::get_ml_summary))
        .route("/api/dashboard/health", get(dashboard::get_health_summary))
        .layer(Extension(storage_manager))
        .layer(Extension(realtime_manager))
}
