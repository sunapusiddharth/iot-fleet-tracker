use crate::ml_edge::engine::MLEngine;
use crate::ml_edge::types::ModelConfig;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{error, info};

pub struct ModelRegistry {
    engines: RwLock<HashMap<String, MLEngine>>,
    config: HashMap<String, ModelConfig>,
}

impl ModelRegistry {
    pub async fn new(configs: Vec<ModelConfig>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut engines = HashMap::new();
        let mut config_map = HashMap::new();

        for config in configs {
            config_map.insert(config.name.clone(), config.clone());

            if config.enabled {
                match MLEngine::new(config.clone(), 50) {
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
            config: config_map,
        })
    }

    pub async fn infer(
        &self,
        model_name: &str,
        image: &image::DynamicImage,
    ) -> Result<crate::ml_edge::types::MLEvent, Box<dyn std::error::Error>> {
        let engines = self.engines.read().await;
        let engine = engines
            .get(model_name)
            .ok_or_else(|| format!("Model not found or disabled: {}", model_name))?;

        engine.infer(image)
    }

    pub async fn reload_model(
        &self,
        model_name: &str,
        model_file: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = self
            .config
            .get(model_name)
            .ok_or_else(|| format!("Model config not found: {}", model_name))?;

        let new_engine = MLEngine::new(config.clone(), 50)?;
        let mut engines = self.engines.write().await;
        engines.insert(model_name.to_string(), new_engine);

        info!(model=%model_name, "🔄 Model reloaded successfully");
        Ok(())
    }
}
