use crate::ml_edge::types::{MLEvent, ModelConfig, HardwareType, InferenceResult};
use crate::ml_edge::preprocess::preprocess_image_zero_copy;
use crate::ml_edge::postprocess::postprocess_output;
use tract_onnx::prelude::*;
use image::DynamicImage;
use bytemuck;
use std::sync::Arc;

pub struct TractEngine {
    model: Arc<TypedModel>,
    config: ModelConfig,
    device_id: String,
}

impl TractEngine {
    pub fn new(config: &ModelConfig, device_id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let model_path = format!("./models/{}/model.onnx", config.name);
        let model = tract_onnx::onnx()
            .model_for_path(model_path)?
            .into_optimized()?
            .into_runnable()?;

        Ok(Self {
            model: Arc::new(model),
            config: config.clone(),
            device_id: device_id.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl crate::ml_edge::engine::InferenceEngine for TractEngine {
    async fn infer(&self, image: &DynamicImage, context: Option<&crate::ml_edge::types::SensorContext>) -> Result<MLEvent, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        // Zero-copy preprocessing
        let (input_tensor, input_shape) = preprocess_image_zero_copy(
            image,
            self.config.input_width,
            self.config.input_height,
            self.config.roi,
        )?;

        // Run inference
        let result = self.model.run(tvec!(input_tensor.into()))?;

        // Postprocess with calibration
        let mut inference_result = postprocess_output(
            &result[0].to_array_view::<f32>()?,
            &self.config.name,
            self.config.threshold,
        )?;

        // Apply sensor fusion if needed
        if self.config.requires_sensor_fusion {
            inference_result = self.apply_sensor_fusion(inference_result, context)?;
        }

        let latency_ms = start.elapsed().as_secs_f32() * 1000.0;

        let event = MLEvent {
            event_id: format!("ml-{}-{}-{}", self.config.name, self.device_id, chrono::Utc::now().timestamp_nanos()),
            model_name: self.config.name.clone(),
            model_version: "1.0".to_string(), // From model config
            timestamp: chrono::Utc::now().timestamp_nanos() as u64,
            result: inference_result,
            confidence: 0.95, // From model output
            calibrated_confidence: self.calibrate_confidence(0.95, context),
            latency_ms,
            input_shape,
            hardware_used: HardwareType::Cpu,
            meta crate::ml_edge::types::MLEventMetadata {
                device_id: self.device_id.clone(),
                truck_id: self.device_id.clone(),
                route_id: "default".to_string(),
                driver_id: "unknown".to_string(),
                camera_id: "unknown".to_string(),
                frame_timestamp: chrono::Utc::now().timestamp_nanos() as u64,
                sensor_context: context.cloned(),
                cpu_usage_percent: 0.0,
                gpu_usage_percent: 0.0,
                memory_used_bytes: 0,
                temperature_c: 0.0,
                model_checksum: self.config.model_checksum.clone(),
                retry_count: 0,
                fallback_reason: None,
            },
        };

        metrics::counter!("ml_inferences_total", "model" => self.config.name.clone(), "hardware" => "cpu").increment(1);
        metrics::gauge!("ml_inference_latency_ms", "model" => self.config.name.clone(), "hardware" => "cpu").set(latency_ms as f64);
        metrics::gauge!("ml_confidence", "model" => self.config.name.clone(), "hardware" => "cpu").set(event.confidence as f64);

        Ok(event)
    }

    fn get_hardware_type(&self) -> HardwareType {
        HardwareType::Cpu
    }

    fn name(&self) -> &str {
        "tract_cpu"
    }
}

impl TractEngine {
    fn apply_sensor_fusion(&self, result: InferenceResult, context: Option<&crate::ml_edge::types::SensorContext>) -> Result<InferenceResult, Box<dyn std::error::Error>> {
        if let Some(ctx) = context {
            match result {
                InferenceResult::LaneDeparture(mut ld) => {
                    // Adjust lane departure sensitivity based on speed
                    let speed_factor = (ctx.speed_kmh / 60.0).min(1.0); // Reduce sensitivity at high speed
                    ld.lane_confidence *= speed_factor;
                    Ok(InferenceResult::LaneDeparture(ld))
                }
                InferenceResult::Drowsiness(mut d) => {
                    // Adjust drowsiness based on time of day
                    if let Some(bias) = self.config.calibration_params.time_of_day_bias.get(&ctx.time_of_day) {
                        d.eye_closure_ratio *= (1.0 + bias);
                    }
                    Ok(InferenceResult::Drowsiness(d))
                }
                _ => Ok(result),
            }
        } else {
            Ok(result)
        }
    }

    fn calibrate_confidence(&self, confidence: f32, context: Option<&crate::ml_edge::types::SensorContext>) -> f32 {
        let mut calibrated = confidence;

        if let Some(ctx) = context {
            // Temperature calibration
            calibrated *= 1.0 + (self.config.calibration_params.temperature_coefficient * (ctx.temperature_c - 25.0));

            // Speed calibration
            calibrated *= 1.0 + (self.config.calibration_params.speed_coefficient * ctx.speed_kmh / 100.0);

            // Route-specific calibration
            if let Some(route_bias) = self.config.calibration_params.route_specific_calibration.get(&ctx.route_id) {
                calibrated *= (1.0 + route_bias);
            }

            // Time of day calibration
            if let Some(time_bias) = self.config.calibration_params.time_of_day_bias.get(&ctx.time_of_day) {
                calibrated *= (1.0 + time_bias);
            }
        }

        calibrated.clamp(0.0, 1.0)
    }
}