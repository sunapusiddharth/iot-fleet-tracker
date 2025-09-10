use redis::{Client, AsyncCommands};
use crate::models::telemetry::TelemetryData;
use crate::models::alert::Alert;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;
use crate::models::truck::Truck;
use uuid::Uuid;
use serde_json;

pub struct CacheStore {
    client: Client,
}

impl CacheStore {
    pub fn new(client: Client) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client,
        })
    }
    
    pub async fn cache_telemetry(&mut self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("telemetry:{}", telemetry.truck_id);
        let value = serde_json::to_string(telemetry)?;
        let _: () = conn.set_ex(key, value, 300).await?; // 5 minutes
        Ok(())
    }
    
    pub async fn get_cached_telemetry(&mut self, truck_id: Uuid) -> Result<Option<TelemetryData>, Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("telemetry:{}", truck_id);
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                let telemetry: TelemetryData = serde_json::from_str(&v)?;
                Ok(Some(telemetry))
            }
            None => Ok(None),
        }
    }
    
    pub async fn cache_alert(&mut self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("alert:{}", alert.id);
        let value = serde_json::to_string(alert)?;
        let _: () = conn.set_ex(key, value, 3600).await?; // 1 hour
        Ok(())
    }
    
    pub async fn get_cached_alert(&mut self, id: Uuid) -> Result<Option<Alert>, Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("alert:{}", id);
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                let alert: Alert = serde_json::from_str(&v)?;
                Ok(Some(alert))
            }
            None => Ok(None),
        }
    }
    
    pub async fn cache_ml_event(&mut self, ml_event: &MlEvent) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("ml_event:{}", ml_event.id);
        let value = serde_json::to_string(ml_event)?;
        let _: () = conn.set_ex(key, value, 3600).await?; // 1 hour
        Ok(())
    }
    
    pub async fn get_cached_ml_event(&mut self, id: Uuid) -> Result<Option<MlEvent>, Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("ml_event:{}", id);
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                let ml_event: MlEvent = serde_json::from_str(&v)?;
                Ok(Some(ml_event))
            }
            None => Ok(None),
        }
    }
    
    pub async fn cache_health_status(&mut self, health_status: &HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("health_status:{}", health_status.truck_id);
        let value = serde_json::to_string(health_status)?;
        let _: () = conn.set_ex(key, value, 300).await?; // 5 minutes
        Ok(())
    }
    
    pub async fn get_cached_health_status(&mut self, truck_id: Uuid) -> Result<Option<HealthStatus>, Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("health_status:{}", truck_id);
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                let health_status: HealthStatus = serde_json::from_str(&v)?;
                Ok(Some(health_status))
            }
            None => Ok(None),
        }
    }
    
    pub async fn cache_truck(&mut self, truck: &Truck) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("truck:{}", truck.id);
        let value = serde_json::to_string(truck)?;
        let _: () = conn.set_ex(key, value, 3600).await?; // 1 hour
        Ok(())
    }
    
    pub async fn get_cached_truck(&mut self, id: Uuid) -> Result<Option<Truck>, Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("truck:{}", id);
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                let truck: Truck = serde_json::from_str(&v)?;
                Ok(Some(truck))
            }
            None => Ok(None),
        }
    }
    
    pub async fn invalidate_truck_cache(&mut self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.client.get_async_connection().await?;
        let key = format!("truck:{}", id);
        let _: () = conn.del(key).await?;
        Ok(())
    }
}