use crate::server::config::ServerConfig;
use crate::models::telemetry::TelemetryData;
use crate::models::alert::Alert;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;
use crate::models::truck::Truck;
use mongodb::Client as MongoClient;
use redis::Client as RedisClient;
use influxdb2::Client as InfluxClient;
use rusoto_s3::{S3Client, S3};
use rusoto_core::Region;
use tokio::sync::Mutex;
use std::sync::Arc;

pub mod timeseries;
pub mod document;
pub mod blob;
pub mod cache;

pub struct StorageManager {
    config: ServerConfig,
    mongo_client: MongoClient,
    redis_client: RedisClient,
    influx_client: InfluxClient,
    s3_client: S3Client,
    document_store: Arc<Mutex<document::DocumentStore>>,
    timeseries_store: Arc<Mutex<timeseries::TimeseriesStore>>,
    blob_store: Arc<Mutex<blob::BlobStore>>,
    cache_store: Arc<Mutex<cache::CacheStore>>,
}

impl StorageManager {
    pub async fn new(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize MongoDB
        let mongo_client = MongoClient::with_uri_str(&config.database.mongodb_uri).await?;
        
        // Initialize Redis
        let redis_client = RedisClient::open(&config.database.redis_uri)?;
        
        // Initialize InfluxDB
        let influx_client = InfluxClient::new(
            &config.database.influxdb_url,
            &config.database.influxdb_token,
        );
        
        // Initialize S3
        let region = match config.storage.s3_region.as_str() {
            "us-east-1" => Region::UsEast1,
            "us-west-2" => Region::UsWest2,
            _ => Region::Custom {
                name: config.storage.s3_region.clone(),
                endpoint: config.storage.s3_endpoint.clone(),
            },
        };
        
        let s3_client = S3Client::new(region);
        
        // Initialize stores
        let document_store = Arc::new(Mutex::new(document::DocumentStore::new(mongo_client.clone(), &config.database.mongodb_database)?));
        let timeseries_store = Arc::new(Mutex::new(timeseries::TimeseriesStore::new(influx_client.clone(), &config.database.influxdb_org, &config.database.influxdb_bucket)?));
        let blob_store = Arc::new(Mutex::new(blob::BlobStore::new(s3_client.clone(), &config.storage.s3_bucket)?));
        let cache_store = Arc::new(Mutex::new(cache::CacheStore::new(redis_client.clone())?));
        
        Ok(Self {
            config,
            mongo_client,
            redis_client,
            influx_client,
            s3_client,
            document_store,
            timeseries_store,
            blob_store,
            cache_store,
        })
    }
    
    pub async fn store_telemetry(&self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        // Store in document DB
        {
            let mut store = self.document_store.lock().await;
            store.store_telemetry(telemetry).await?;
        }
        
        // Store in time-series DB
        {
            let mut store = self.timeseries_store.lock().await;
            store.store_telemetry(telemetry).await?;
        }
        
        // Cache recent telemetry
        {
            let mut store = self.cache_store.lock().await;
            store.cache_telemetry(telemetry).await?;
        }
        
        Ok(())
    }
    
    pub async fn store_alert(&self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut store = self.document_store.lock().await;
            store.store_alert(alert).await?;
        }
        
        {
            let mut store = self.cache_store.lock().await;
            store.cache_alert(alert).await?;
        }
        
        Ok(())
    }
    
    pub async fn store_ml_event(&self, ml_event: &MlEvent) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut store = self.document_store.lock().await;
            store.store_ml_event(ml_event).await?;
        }
        
        {
            let mut store = self.cache_store.lock().await;
            store.cache_ml_event(ml_event).await?;
        }
        
        Ok(())
    }
    
    pub async fn store_health_status(&self, health_status: &HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut store = self.document_store.lock().await;
            store.store_health_status(health_status).await?;
        }
        
        {
            let mut store = self.timeseries_store.lock().await;
            store.store_health_status(health_status).await?;
        }
        
        {
            let mut store = self.cache_store.lock().await;
            store.cache_health_status(health_status).await?;
        }
        
        Ok(())
    }
    
    pub async fn store_truck(&self, truck: &Truck) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut store = self.document_store.lock().await;
            store.store_truck(truck).await?;
        }
        
        {
            let mut store = self.cache_store.lock().await;
            store.cache_truck(truck).await?;
        }
        
        Ok(())
    }
    
    pub async fn get_document_store(&self) -> Arc<Mutex<document::DocumentStore>> {
        self.document_store.clone()
    }
    
    pub async fn get_timeseries_store(&self) -> Arc<Mutex<timeseries::TimeseriesStore>> {
        self.timeseries_store.clone()
    }
    
    pub async fn get_blob_store(&self) -> Arc<Mutex<blob::BlobStore>> {
        self.blob_store.clone()
    }
    
    pub async fn get_cache_store(&self) -> Arc<Mutex<cache::CacheStore>> {
        self.cache_store.clone()
    }
}