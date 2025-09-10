use crate::simulator::types::TruckState;
use tokio::sync::broadcast;
use axum::{
    Router,
    routing::get,
    response::Json,
    extract::Path,
};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};

#[derive(Clone)]
pub struct HttpHandler {
    port: u16,
    state_store: Arc<Mutex<HashMap<String, TruckState>>>,
}

impl HttpHandler {
    pub fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            port,
            state_store: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    pub async fn start(&self, mut rx: broadcast::Receiver<TruckState>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting HTTP server on port {}", self.port);
        
        // Clone state store for HTTP handler
        let state_store = self.state_store.clone();
        
        // Start state update task
        tokio::spawn(async move {
            while let Ok(state) = rx.recv().await {
                let mut store = state_store.lock().await;
                store.insert(state.truck_id.clone(), state);
            }
        });
        
        // Create router
        let app = Router::new()
            .route("/trucks", get(get_all_trucks))
            .route("/trucks/:id", get(get_truck))
            .with_state(self.state_store.clone());
        
        // Start server
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
        
        Ok(())
    }
}

async fn get_all_trucks(
    axum::extract::State(state_store): axum::extract::State<Arc<Mutex<HashMap<String, TruckState>>>>,
) -> Json<Value> {
    let store = state_store.lock().await;
    let trucks: Vec<_> = store.values().collect();
    Json(serde_json::to_value(trucks).unwrap_or(serde_json::json!([])))
}

async fn get_truck(
    axum::extract::State(state_store): axum::extract::State<Arc<Mutex<HashMap<String, TruckState>>>>,
    Path(truck_id): Path<String>,
) -> Json<Value> {
    let store = state_store.lock().await;
    if let Some(state) = store.get(&truck_id) {
        Json(serde_json::to_value(state).unwrap_or(serde_json::json!({})))
    } else {
        Json(serde_json::json!({"error": "Truck not found"}))
    }
}