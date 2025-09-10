use warp::Filter;
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, error};
use crate::server::ingestion::IngestionEvent;
use tokio::sync::broadcast;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct WebSocketIngestionHandler {
    port: u16,
}

impl WebSocketIngestionHandler {
    pub fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            port,
        })
    }
    
    pub async fn start(&self, tx: broadcast::Sender<IngestionEvent>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting WebSocket ingestion on port {}", self.port);
        
        let tx_clone = tx.clone();
        let ws_route = warp::path("ingest")
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let tx = tx_clone.clone();
                ws.on_upgrade(move |websocket| handle_websocket_connection(websocket, tx))
            });
        
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        warp::serve(ws_route)
            .run(addr)
            .await?;
        
        Ok(())
    }
}

async fn handle_websocket_connection(
    websocket: warp::ws::WebSocket,
    tx: broadcast::Sender<IngestionEvent>,
) {
    let (mut ws_tx, mut ws_rx) = websocket.split();
    
    // Send welcome message
    if let Err(e) = ws_tx.send(Message::text("Connected to Truck Central Server")).await {
        error!("Failed to send welcome message: {}", e);
        return;
    }
    
    // Handle incoming messages
    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_text().unwrap_or_default();
                    
                    // Try to parse as JSON and determine event type
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                        if let Some(event_type) = json.get("type").and_then(|t| t.as_str()) {
                            match event_type {
                                "telemetry" => {
                                    if let Ok(telemetry) = serde_json::from_value::<crate::models::telemetry::TelemetryData>(json) {
                                        if let Err(e) = tx.send(IngestionEvent::Telemetry(telemetry)) {
                                            error!("Failed to send telemetry event: {}", e);
                                        }
                                    }
                                }
                                "alert" => {
                                    if let Ok(alert) = serde_json::from_value::<crate::models::alert::Alert>(json) {
                                        if let Err(e) = tx.send(IngestionEvent::Alert(alert)) {
                                            error!("Failed to send alert event: {}", e);
                                        }
                                    }
                                }
                                "ml_event" => {
                                    if let Ok(ml_event) = serde_json::from_value::<crate::models::ml::MlEvent>(json) {
                                        if let Err(e) = tx.send(IngestionEvent::MlEvent(ml_event)) {
                                            error!("Failed to send ML event: {}", e);
                                        }
                                    }
                                }
                                "health_status" => {
                                    if let Ok(health_status) = serde_json::from_value::<crate::models::health::HealthStatus>(json) {
                                        if let Err(e) = tx.send(IngestionEvent::HealthStatus(health_status)) {
                                            error!("Failed to send health status: {}", e);
                                        }
                                    }
                                }
                                _ => {
                                    error!("Unknown event type: {}", event_type);
                                }
                            }
                        }
                    }
                } else if msg.is_ping() {
                    if let Err(e) = ws_tx.send(Message::pong(msg.into_data())).await {
                        error!("Failed to send pong: {}", e);
                        break;
                    }
                } else if msg.is_close() {
                    info!("WebSocket connection closed");
                    break;
                }
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }
}