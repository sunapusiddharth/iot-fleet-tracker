use crate::server::config::ServerConfig;
use crate::server::ingestion::IngestionManager;
use crate::server::storage::StorageManager;
use crate::server::processing::ProcessingManager;
use crate::server::realtime::RealtimeManager;
use crate::server::api;
use tokio::sync::broadcast;
use tracing::{info, error};

pub mod config;
pub mod ingestion;
pub mod storage;
pub mod processing;
pub mod realtime;
pub mod api;
pub mod auth;

pub struct CentralServer {
    config: ServerConfig,
    ingestion_manager: IngestionManager,
    storage_manager: StorageManager,
    processing_manager: ProcessingManager,
    realtime_manager: RealtimeManager,
}

impl CentralServer {
    pub async fn new(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let ingestion_manager = IngestionManager::new(config.clone()).await?;
        let storage_manager = StorageManager::new(config.clone()).await?;
        let processing_manager = ProcessingManager::new(config.clone()).await?;
        let realtime_manager = RealtimeManager::new(config.clone()).await?;
        
        Ok(Self {
            config,
            ingestion_manager,
            storage_manager,
            processing_manager,
            realtime_manager,
        })
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting Central Server");
        
        // Start ingestion manager
        let ingestion_manager = self.ingestion_manager.clone();
        tokio::spawn(async move {
            if let Err(e) = ingestion_manager.start().await {
                error!("Ingestion manager failed: {}", e);
            }
        });
        
        // Start processing manager
        let processing_manager = self.processing_manager.clone();
        let ingestion_rx = self.ingestion_manager.get_receiver();
        tokio::spawn(async move {
            if let Err(e) = processing_manager.start(ingestion_rx).await {
                error!("Processing manager failed: {}", e);
            }
        });
        
        // Start realtime manager
        let realtime_manager = self.realtime_manager.clone();
        tokio::spawn(async move {
            if let Err(e) = realtime_manager.start().await {
                error!("Realtime manager failed: {}", e);
            }
        });
        
        // Start HTTP API server
        let storage_manager = Arc::new(self.storage_manager.clone());
        let realtime_manager = Arc::new(self.realtime_manager.clone());
        let router = api::create_router(storage_manager, realtime_manager);
        
        let addr = format!("{}:{}", self.config.server.bind_address, self.config.server.http_port)
            .parse::<SocketAddr>()?;
        
        info!("🚀 HTTP API server starting on {}", addr);
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await?;
        
        Ok(())
    }
}