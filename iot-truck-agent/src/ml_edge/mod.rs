use crate::config::Config;
use crate::camera::types::CameraFrame;
use crate::stream::types::StreamEvent;
use crate::ml_edge::types::{MLEvent, ModelConfig};
use crate::ml_edge::models::ModelRegistry;
use image::DynamicImage;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

pub mod types;
pub mod error;
pub mod engine;
pub mod models;
pub mod preprocess;
pub mod postprocess;

// Metrics
metrics::describe_counter!("ml_inferences_total", "Total ML inferences");
metrics::describe_gauge!("ml_inference_latency_ms", "ML inference latency in ms");
metrics::describe_gauge!("ml_confidence", "ML inference confidence");
metrics::describe_counter!("ml_errors_total", "ML inference errors");
metrics::describe_gauge!("ml_engine_status", "ML engine status (1=up, 0=down)");

pub struct MLEdgeManager {
    registry: ModelRegistry,
    tx: broadcast::Sender<StreamEvent>,
    device_id: String,
}

impl MLEdgeManager {
    pub async fn new(config: &Config, tx: broadcast::Sender<StreamEvent>) -> Result<Self, Box<dyn std::error::Error>> {
        // Build model configs from TOML
        let mut model_configs = Vec::new();

        if config.ml_edge.enable_drowsiness {
            model_configs.push(ModelConfig {
                name: "drowsiness".to_string(),
                model_file: config.drowsiness.model_file.clone(),
                enabled: true,
                threshold: config.drowsiness.threshold,
                input_width: config.drowsiness.input_width,
                input_height: config.drowsiness.input_height,
                roi: Some((
                    config.drowsiness.roi_x,
                    config.drowsiness.roi_y,
                    config.drowsiness.roi_width,
                    config.drowsiness.roi_height,
                )),
                max_fps: 10,
            });
        }

        if config.ml_edge.enable_lane_departure {
            model_configs.push(ModelConfig {
                name: "lane_departure".to_string(),
                model_file: config.lane_departure.model_file.clone(),
                enabled: true,
                threshold: config.lane_departure.threshold,
                input_width: config.lane_departure.input_width,
                input_height: config.lane_departure.input_height,
                roi: None,
                max_fps: 5,
            });
        }

        if config.ml_edge.enable_cargo_tamper {
            model_configs.push(ModelConfig {
                name: "cargo_tamper".to_string(),
                model_file: config.cargo_tamper.model_file.clone(),
                enabled: true,
                threshold: config.cargo_tamper.threshold,
                input_width: config.cargo_tamper.input_width,
                input_height: config.cargo_tamper.input_height,
                roi: None,
                max_fps: 2,
            });
        }

        let registry = ModelRegistry::new(model_configs).await?;
        let device_id = config.device_id.clone();

        info!("✅ ML Edge Manager initialized with {} models", model_configs.len());

        Ok(Self {
            registry,
            tx,
            device_id,
        })
    }

    pub async fn process_frame(&self, frame: &CameraFrame) -> Result<(), Box<dyn std::error::Error>> {
        // Convert to DynamicImage
        let img = image::load_from_memory(&frame.data)
            .map_err(|e| format!("Failed to decode image: {}", e))?;

        // Route to appropriate model based on camera_id
        let model_name = match &frame.camera_id {
            crate::camera::types::CameraId::Driver => "drowsiness",
            crate::camera::types::CameraId::Front => "lane_departure",
            crate::camera::types::CameraId::Cargo => "cargo_tamper",
            _ => return Ok(()), // No model for this camera
        };

        // Run inference
        match self.registry.infer(model_name, &img).await {
            Ok(ml_event) => {
                // Send to streamer
                let stream_event = StreamEvent::new_ml_event(ml_event, &self.device_id);
                if self.tx.send(stream_event).is_err() {
                    warn!("ML event channel full — dropping event");
                }

                // Trigger local alert if needed
                if ml_event.is_alert() {
                    self.trigger_local_alert(&ml_event).await;
                }
            }
            Err(e) => {
                error!(error=%e, model=%model_name, "ML inference failed");
                metrics::counter!("ml_errors_total", "model" => model_name.to_string()).increment(1);
            }
        }

        Ok(())
    }

    async fn trigger_local_alert(&self, event: &MLEvent) {
        // In future: trigger GPIO buzzer, LED, etc.
        info!(event_id=%event.event_id, "🚨 LOCAL ALERT TRIGGERED: {:?}", event.result);
        metrics::counter!("ml_alerts_triggered_total", "model" => event.model_name.clone()).increment(1);
    }
}

// Add to StreamEvent
impl StreamEvent {
    pub fn new_ml_event(ml_event: MLEvent, device_id: &str) -> Self {
        Self {
            event_id: ml_event.event_id.clone(),
            event_type: crate::stream::types::EventType::Ml,
            payload: crate::stream::types::EventPayload::Ml(ml_event),
            timestamp: ml_event.timestamp,
            meta crate::stream::types::EventMetadata {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(),
                sequence_number: 0, // Will be assigned by WAL
                retry_count: 0,
                source_module: "ml_edge".to_string(),
            },
        }
    }
}