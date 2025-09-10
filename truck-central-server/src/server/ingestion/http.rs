use axum::{
    Router,
    routing::post,
    Json,
    http::StatusCode,
};
use tracing::{info, error};
use crate::server::ingestion::IngestionEvent;
use tokio::sync::broadcast;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct HttpIngestionHandler {
    port: u16,
}

impl HttpIngestionHandler {
    pub fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            port,
        })
    }
    
    pub async fn start(&self, tx: broadcast::Sender<IngestionEvent>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting HTTP ingestion on port {}", self.port);
        
        let tx_clone = tx.clone();
        let app = Router::new()
            .route("/ingest/telemetry", post(handle_telemetry))
            .route("/ingest/alert", post(handle_alert))
            .route("/ingest/ml", post(handle_ml_event))
            .route("/ingest/health", post(handle_health_status))
            .with_state(tx_clone);
        
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
        
        Ok(())
    }
}

async fn handle_telemetry(
    axum::extract::State(tx): axum::extract::State<broadcast::Sender<IngestionEvent>>,
    Json(payload): Json<crate::models::telemetry::TelemetryData>,
) -> Result<StatusCode, StatusCode> {
    if let Err(e) = tx.send(IngestionEvent::Telemetry(payload)) {
        error!("Failed to send telemetry event: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(StatusCode::OK)
}

async fn handle_alert(
    axum::extract::State(tx): axum::extract::State<broadcast::Sender<IngestionEvent>>,
    Json(payload): Json<crate::models::alert::Alert>,
) -> Result<StatusCode, StatusCode> {
    if let Err(e) = tx.send(IngestionEvent::Alert(payload)) {
        error!("Failed to send alert event: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(StatusCode::OK)
}

async fn handle_ml_event(
    axum::extract::State(tx): axum::extract::State<broadcast::Sender<IngestionEvent>>,
    Json(payload): Json<crate::models::ml::MlEvent>,
) -> Result<StatusCode, StatusCode> {
    if let Err(e) = tx.send(IngestionEvent::MlEvent(payload)) {
        error!("Failed to send ML event: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(StatusCode::OK)
}

async fn handle_health_status(
    axum::extract::State(tx): axum::extract::State<broadcast::Sender<IngestionEvent>>,
    Json(payload): Json<crate::models::health::HealthStatus>,
) -> Result<StatusCode, StatusCode> {
    if let Err(e) = tx.send(IngestionEvent::HealthStatus(payload)) {
        error!("Failed to send health status: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(StatusCode::OK)
}