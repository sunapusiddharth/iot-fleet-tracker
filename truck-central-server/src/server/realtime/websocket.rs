use tokio::sync::broadcast;
use warp::Filter;
use tokio_tungstenite::tungstenite::Message;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};
use crate::models::telemetry::TelemetryData;
use crate::models::alert::Alert;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;

#[derive(Clone)]
pub struct WebSocketServer {
    port: u16,
    clients: Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Message>>>>,
}

impl WebSocketServer {
    pub fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            port,
            clients: Arc::new(Mutex::new(Vec::new())),
        })
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting WebSocket server on port {}", self.port);
        
        // Clone clients for state update task
        let clients = self.clients.clone();
        
        // Create WebSocket route
        let clients = self.clients.clone();
        let ws_route = warp::path("ws")
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let clients = clients.clone();
                ws.on_upgrade(move |websocket| handle_connection(websocket, clients))
            });
        
        // Start server
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        warp::serve(ws_route)
            .run(addr)
            .await;
        
        Ok(())
    }
    
    pub async fn broadcast_telemetry(&self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&telemetry).unwrap_or_default();
        let message = Message::text(format!("{{\"type\":\"telemetry\",\"data\":{}}}", json));
        
        let mut clients_lock = self.clients.lock().await;
        clients_lock.retain(|sender| sender.send(message.clone()).is_ok());
        
        Ok(())
    }
    
    pub async fn broadcast_alert(&self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&alert).unwrap_or_default();
        let message = Message::text(format!("{{\"type\":\"alert\",\"data\":{}}}", json));
        
        let mut clients_lock = self.clients.lock().await;
        clients_lock.retain(|sender| sender.send(message.clone()).is_ok());
        
        Ok(())
    }
    
    pub async fn broadcast_ml_event(&self, ml_event: &MlEvent) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&ml_event).unwrap_or_default();
        let message = Message::text(format!("{{\"type\":\"ml_event\",\"data\":{}}}", json));
        
        let mut clients_lock = self.clients.lock().await;
        clients_lock.retain(|sender| sender.send(message.clone()).is_ok());
        
        Ok(())
    }
    
    pub async fn broadcast_health_status(&self, health_status: &HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&health_status).unwrap_or_default();
        let message = Message::text(format!("{{\"type\":\"health_status\",\"data\":{}}}", json));
        
        let mut clients_lock = self.clients.lock().await;
        clients_lock.retain(|sender| sender.send(message.clone()).is_ok());
        
        Ok(())
    }
}

async fn handle_connection(
    websocket: warp::ws::WebSocket,
    clients: Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Message>>>>,
) {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Add to clients
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.push(tx);
    }
    
    // Split websocket
    let (mut ws_tx, mut ws_rx) = websocket.split();
    
    // Handle incoming messages (ping/pong)
    let _ = tokio::spawn(async move {
        while let Some(result) = ws_rx.next().await {
            match result {
                Ok(msg) => {
                    if msg.is_ping() {
                        if let Err(e) = ws_tx.send(Message::pong(msg.into_data())).await {
                            error!("Failed to send pong: {}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            }
        }
    });
    
    // Handle outgoing messages
    let _ = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = ws_tx.send(msg).await {
                error!("Failed to send message: {}", e);
                break;
            }
        }
    });
}