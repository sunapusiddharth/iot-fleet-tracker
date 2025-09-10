use crate::server::config::ServerConfig;
use crate::models::telemetry::TelemetryData;
use crate::models::alert::Alert;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;
use tokio::sync::broadcast;
use tracing::{info, error};

pub mod alerts;
pub mod ml;
pub mod aggregation;
pub mod enrichment;

pub struct ProcessingManager {
    config: ServerConfig,
    alert_processor: alerts::AlertProcessor,
    ml_processor: ml::MlProcessor,
    aggregation_processor: aggregation::AggregationProcessor,
    enrichment_processor: enrichment::EnrichmentProcessor,
}

impl ProcessingManager {
    pub async fn new(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let alert_processor = alerts::AlertProcessor::new();
        let ml_processor = ml::MlProcessor::new();
        let aggregation_processor = aggregation::AggregationProcessor::new();
        let enrichment_processor = enrichment::EnrichmentProcessor::new();
        
        Ok(Self {
            config,
            alert_processor,
            ml_processor,
            aggregation_processor,
            enrichment_processor,
        })
    }
    
    pub async fn start(&self, mut rx: broadcast::Receiver<crate::server::ingestion::IngestionEvent>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting processing manager");
        
        while let Ok(event) = rx.recv().await {
            match event {
                crate::server::ingestion::IngestionEvent::Telemetry(telemetry) => {
                    // Enrich telemetry with truck info
                    let enriched_telemetry = self.enrichment_processor.enrich_telemetry(telemetry).await?;
                    
                    // Process for alerts
                    let alerts = self.alert_processor.process_telemetry(&enriched_telemetry).await?;
                    for alert in alerts {
                        // Send to alert processing
                        // In production, send to storage and real-time
                    }
                    
                    // Aggregate for dashboards
                    self.aggregation_processor.aggregate_telemetry(&enriched_telemetry).await?;
                }
                crate::server::ingestion::IngestionEvent::Alert(alert) => {
                    // Process alert
                    let processed_alert = self.alert_processor.process_alert(alert).await?;
                    
                    // Send to storage and real-time
                }
                crate::server::ingestion::IngestionEvent::MlEvent(ml_event) => {
                    // Process ML event
                    let processed_ml_event = self.ml_processor.process_ml_event(ml_event).await?;
                    
                    // Generate alerts if needed
                    let alerts = self.alert_processor.process_ml_event(&processed_ml_event).await?;
                    for alert in alerts {
                        // Send to alert processing
                    }
                    
                    // Aggregate for ML dashboards
                    self.aggregation_processor.aggregate_ml_event(&processed_ml_event).await?;
                }
                crate::server::ingestion::IngestionEvent::HealthStatus(health_status) => {
                    // Process health status
                    let processed_health_status = self.enrichment_processor.enrich_health_status(health_status).await?;
                    
                    // Generate health alerts if needed
                    let alerts = self.alert_processor.process_health_status(&processed_health_status).await?;
                    for alert in alerts {
                        // Send to alert processing
                    }
                    
                    // Aggregate for health dashboards
                    self.aggregation_processor.aggregate_health_status(&processed_health_status).await?;
                }
            }
        }
        
        Ok(())
    }
}