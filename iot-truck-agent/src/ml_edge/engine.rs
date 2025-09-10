use crate::ml_edge::types::{MLEvent, ModelConfig, InferenceResult};
use crate::ml_edge::preprocess::preprocess_image;
use crate::ml_edge::postprocess::postprocess_output;
use tract_onnx::prelude::*;
use image::{DynamicImage, ImageBuffer, Rgb};
use ndarray::Array;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{error, info, warn};

pub struct MLEngine {
    models: Arc<Mutex<HashMap<String, TypedModel>>>,
    config: ModelConfig,
    last_inference: std::time::Instant,
    throttle_ms: u64,
}

impl MLEngine {
    pub fn new(config: ModelConfig, throttle_ms: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let model_path = format!("./models/{}", config.model_file);
        let model = tract_onnx::onnx()
            .model_for_path(model_path)?
            .into_optimized()?
            .into_runnable()?;

        let mut models = HashMap::new();
        models.insert(config.name.clone(), model);

        Ok(Self {
            models: Arc::new(Mutex::new(models)),
            config,
            last_inference: std::time::Instant::now(),
            throttle_ms,
        })
    }

    pub fn load_model(&mut self, name: &str, model_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let model_path = format!("./models/{}", model_file);
        let model = tract_onnx::onnx()
            .model_for_path(model_path)?
            .into_optimized()?
            .into_runnable()?;

        let mut models = self.models.lock().unwrap();
        models.insert(name.to_string(), model);
        info!(model=%name, "✅ Model loaded successfully");
        Ok(())
    }

    pub fn infer(&self, image: &DynamicImage) -> Result<MLEvent, Box<dyn std::error::Error>> {
        // Throttle
        let elapsed = self.last_inference.elapsed().as_millis();
        if elapsed < self.throttle_ms {
            return Err("Throttling — too frequent inference".into());
        }
        self.last_inference = std::time::Instant::now();

        let start = std::time::Instant::now();

        // Preprocess
        let (input_tensor, input_shape) = preprocess_image(
            image,
            self.config.input_width,
            self.config.input_height,
            self.config.roi,
        )?;

        // Run inference
        let models = self.models.lock().unwrap();
        let model = models.get(&self.config.name)
            .ok_or_else(|| format!("Model not found: {}", self.config.name))?;

        let result = model.run(tvec!(input_tensor.into()))?;

        // Postprocess
        let inference_result = postprocess_output(
            &result[0].to_array_view::<f32>()?,
            &self.config.name,
            self.config.threshold,
        )?;

        let latency_ms = start.elapsed().as_secs_f32() * 1000.0;

        let event = MLEvent::new(
            &self.config.name,
            inference_result,
            0.95, // Placeholder — calculate from model output
            latency_ms,
            input_shape,
            "TRK-001", // Placeholder — from config
            "driver",  // Placeholder — from camera
            chrono::Utc::now().timestamp_nanos() as u64,
        );

        metrics::counter!("ml_inferences_total", "model" => self.config.name.clone()).increment(1);
        metrics::gauge!("ml_inference_latency_ms", "model" => self.config.name.clone()).set(latency_ms as f64);
        metrics::gauge!("ml_confidence", "model" => self.config.name.clone()).set(event.confidence as f64);

        Ok(event)
    }
}