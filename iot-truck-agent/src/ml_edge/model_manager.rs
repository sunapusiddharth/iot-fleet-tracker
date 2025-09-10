use crate::ml_edge::types::ModelConfig;
use crate::ml_edge::engine::{InferenceEngine, EngineFactory};
use crate::ml_edge::security::verify_model_signature;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

pub struct ModelManager {
    engines: RwLock<HashMap<String, Arc<dyn InferenceEngine>>>,
    configs: HashMap<String, ModelConfig>,
    device_id: String,
}

impl ModelManager {
    pub async fn new(configs: Vec<ModelConfig>, device_id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut engines = HashMap::new();
        let mut config_map = HashMap::new();

        for config in configs {
            config_map.insert(config.name.clone(), config.clone());
            
            if config.enabled {
                // Verify model signature
                if let Err(e) = verify_model_signature(&format!("./models/{}/model.onnx", config.name), &format!("./models/{}/model.sig", config.name)) {
                    error!(model=%config.name, error=%e, "❌ Model signature verification failed");
                    continue;
                }

                match EngineFactory::create_engine(&config, device_id) {
                    Ok(engine) => {
                        engines.insert(config.name.clone(), engine);
                        info!(model=%config.name, "✅ ML engine initialized");
                    }
                    Err(e) => {
                        error!(model=%config.name, error=%e, "❌ Failed to initialize ML engine");
                    }
                }
            }
        }

        Ok(Self {
            engines: RwLock::new(engines),
            configs: config_map,
            device_id: device_id.to_string(),
        })
    }

    pub async fn infer(
        &self,
        model_name: &str,
        image: &image::DynamicImage,
        context: Option<&crate::ml_edge::types::SensorContext>,
    ) -> Result<crate::ml_edge::types::MLEvent, Box<dyn std::error::Error>> {
        let engines = self.engines.read().await;
        let engine = engines.get(model_name)
            .ok_or_else(|| format!("Model not found or disabled: {}", model_name))?;

        engine.infer(image, context).await
    }

    pub async fn reload_model(&self, model_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.configs.get(model_name)
            .ok_or_else(|| format!("Model config not found: {}", model_name))?;

        // Verify signature
        if let Err(e) = verify_model_signature(&format!("./models/{}/model.onnx", model_name), &format!("./models/{}/model.sig", model_name)) {
            return Err(format!("Model signature verification failed: {}", e).into());
        }

        let new_engine = EngineFactory::create_engine(config, &self.device_id)?;
        
        let mut engines = self.engines.write().await;
        engines.insert(model_name.to_string(), new_engine);

        info!(model=%model_name, "🔄 Model reloaded successfully");
        Ok(())
    }

    pub async fn update_model_config(&mut self, config: ModelConfig) -> Result<(), Box<dyn std::error::Error>> {
        let model_name = config.name.clone();
        
        // Verify signature if model file changed
        if let Some(old_config) = self.configs.get(&model_name) {
            if old_config.model_file != config.model_file || old_config.model_checksum != config.model_checksum {
                if let Err(e) = verify_model_signature(&format!("./models/{}/model.onnx", model_name), &format!("./models/{}/model.sig", model_name)) {
                    return Err(format!("Model signature verification failed: {}", e).into());
                }
            }
        }

        self.configs.insert(model_name.clone(), config.clone());

        // Reload engine if enabled
        if config.enabled {
            let new_engine = EngineFactory::create_engine(&config, &self.device_id)?;
            let mut engines = self.engines.write().await;
            engines.insert(model_name, new_engine);
        } else {
            // Disable engine
            let mut engines = self.engines.write().await;
            engines.remove(&model_name);
        }

        Ok(())
    }
}