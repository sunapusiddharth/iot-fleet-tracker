use crate::models::telemetry::{TelemetryData, TelemetrySummary, SensorData, CameraData, CameraFrameRef, FrameMetadata};
use crate::server::storage::StorageManager;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct TelemetryService {
    storage_manager: Arc<StorageManager>,
}

impl TelemetryService {
    pub fn new(storage_manager: Arc<StorageManager>) -> Self {
        Self {
            storage_manager,
        }
    }

    pub async fn store_telemetry(&self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_telemetry(telemetry).await?;

        // Update truck last seen and location
        self.update_truck_location(&telemetry.truck_id, &telemetry.location, telemetry.timestamp).await?;

        metrics::counter!("telemetry_events_stored_total").increment(1);
        tracing::info!(truck_id=%telemetry.truck_id, "✅ Telemetry stored");

        Ok(())
    }

    async fn update_truck_location(&self, truck_id: &Uuid, location: &geo::Point<f64>, timestamp: chrono::DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        if let Some(mut truck) = store.get_truck(truck_id).await? {
            truck.last_seen = timestamp;
            truck.location = *location;
            truck.updated_at = Utc::now();
            store.store_truck(&truck).await?;
        }

        Ok(())
    }

    pub async fn get_truck_telemetry(&self, truck_id: &Uuid, limit: i64, offset: i64) -> Result<Vec<TelemetryData>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let telemetry = store.get_truck_telemetry(truck_id, limit, offset).await?;
        
        metrics::counter!("telemetry_retrieved_total").increment(1);
        tracing::info!(truck_id=%truck_id, count=%telemetry.len(), "📤 Telemetry retrieved");

        Ok(telemetry)
    }

    pub async fn get_telemetry_summary(&self, truck_id: &Uuid) -> Result<Option<TelemetrySummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let recent_telemetry = store.get_recent_telemetry(truck_id, 1).await?;
        
        if recent_telemetry.is_empty() {
            return Ok(None);
        }
        
        let latest = &recent_telemetry[0];
        let sensors = &latest.sensors;

        let summary = TelemetrySummary {
            truck_id: latest.truck_id,
            last_timestamp: latest.timestamp,
            last_location: latest.location,
            last_speed_kmh: latest.speed_kmh,
            last_heading: latest.heading,
            last_rpm: sensors.obd.rpm,
            last_coolant_temp: sensors.obd.coolant_temp,
            last_fuel_level: sensors.obd.fuel_level,
            last_accel_x: sensors.imu.accel_x,
            last_accel_y: sensors.imu.accel_y,
            last_accel_z: sensors.imu.accel_z,
            tire_pressures: [
                sensors.tpms.front_left.pressure_psi,
                sensors.tpms.front_right.pressure_psi,
                sensors.tpms.rear_left.pressure_psi,
                sensors.tpms.rear_right.pressure_psi,
            ],
            tire_temperatures: [
                sensors.tpms.front_left.temperature_c,
                sensors.tpms.front_right.temperature_c,
                sensors.tpms.rear_left.temperature_c,
                sensors.tpms.rear_right.temperature_c,
            ],
        };

        metrics::counter!("telemetry_summaries_retrieved_total").increment(1);
        tracing::info!(truck_id=%truck_id, "📤 Telemetry summary retrieved");

        Ok(Some(summary))
    }

    pub async fn store_camera_frame(&self, truck_id: &Uuid, camera_type: &str, frame_data: &[u8], metadata: FrameMetadata) -> Result<CameraFrameRef, Box<dyn std::error::Error>> {
        let blob_store = self.storage_manager.get_blob_store().await;
        let mut store = blob_store.lock().await;
        
        // Generate frame ID
        let frame_id = Uuid::new_v4();
        
        // Store in blob storage
        let path = format!("trucks/{}/camera/{}/{}.jpg", truck_id, camera_type, frame_id);
        let url = store.store_blob(&path, frame_data).await?;
        
        // Create thumbnail (simplified - in production, generate actual thumbnail)
        let thumbnail_path = format!("trucks/{}/camera/{}/{}_thumb.jpg", truck_id, camera_type, frame_id);
        let thumbnail_url = store.store_blob(&thumbnail_path, frame_data).await?;

        let frame_ref = CameraFrameRef {
            frame_id,
            timestamp: Utc::now(),
            url,
            thumbnail_url: Some(thumbnail_url),
            width: 1280,
            height: 720,
            format: "jpeg".to_string(),
            size_bytes: frame_data.len() as u64,
            is_keyframe: true,
            meta metadata,
        };

        metrics::counter!("camera_frames_stored_total").increment(1);
        tracing::info!(truck_id=%truck_id, camera_type=%camera_type, "✅ Camera frame stored");

        Ok(frame_ref)
    }
}


use crate::models::telemetry::{TelemetryData, TelemetrySummary};
use crate::server::storage::StorageManager;
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::sync::Arc;

pub struct TelemetryService {
    storage_manager: Arc<StorageManager>,
}

impl TelemetryService {
    pub fn new(storage_manager: Arc<StorageManager>) -> Self {
        Self {
            storage_manager,
        }
    }

    pub async fn store_telemetry(&self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_telemetry(telemetry).await?;
        Ok(())
    }

    pub async fn get_telemetry(&self, id: Uuid) -> Result<TelemetryData, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let telemetry = store.get_telemetry(id).await?;
        
        Ok(telemetry)
    }

    pub async fn get_truck_telemetry(&self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<TelemetryData>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let telemetry = store.get_truck_telemetry(truck_id, limit, offset).await?;
        
        Ok(telemetry)
    }

    pub async fn get_latest_telemetry(&self, truck_id: Uuid) -> Result<TelemetryData, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let telemetry = store.get_latest_telemetry(truck_id).await?;
        
        Ok(telemetry)
    }

    pub async fn get_telemetry_summary(&self, truck_id: Uuid) -> Result<TelemetrySummary, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let telemetry = store.get_latest_telemetry(truck_id).await?;
        
        let tire_pressures = [
            telemetry.sensors.tpms.front_left.pressure_psi,
            telemetry.sensors.tpms.front_right.pressure_psi,
            telemetry.sensors.tpms.rear_left.pressure_psi,
            telemetry.sensors.tpms.rear_right.pressure_psi,
        ];
        
        let tire_temperatures = [
            telemetry.sensors.tpms.front_left.temperature_c,
            telemetry.sensors.tpms.front_right.temperature_c,
            telemetry.sensors.tpms.rear_left.temperature_c,
            telemetry.sensors.tpms.rear_right.temperature_c,
        ];
        
        Ok(TelemetrySummary {
            truck_id: telemetry.truck_id,
            last_timestamp: telemetry.timestamp,
            last_location: telemetry.location,
            last_speed_kmh: telemetry.speed_kmh,
            last_heading: telemetry.heading,
            last_rpm: telemetry.sensors.obd.rpm,
            last_coolant_temp: telemetry.sensors.obd.coolant_temp,
            last_fuel_level: telemetry.sensors.obd.fuel_level,
            last_accel_x: telemetry.sensors.imu.accel_x,
            last_accel_y: telemetry.sensors.imu.accel_y,
            last_accel_z: telemetry.sensors.imu.accel_z,
            tire_pressures,
            tire_temperatures,
        })
    }

    pub async fn get_telemetry_by_time_range(&self, truck_id: Uuid, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<TelemetryData>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let telemetry = store.get_telemetry_by_time_range(truck_id, start_time, end_time, limit, offset).await?;
        
        Ok(telemetry)
    }

    pub async fn get_telemetry_statistics(&self, truck_id: Uuid, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>) -> Result<TelemetryStatistics, Box<dyn std::error::Error>> {
        let timeseries_store = self.storage_manager.get_timeseries_store().await;
        let mut store = timeseries_store.lock().await;
        
        let stats = store.get_telemetry_statistics(truck_id, start_time, end_time).await?;
        
        Ok(stats)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TelemetryStatistics {
    pub truck_id: Uuid,
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
    pub avg_speed_kmh: f32,
    pub max_speed_kmh: f32,
    pub min_speed_kmh: f32,
    pub avg_rpm: f32,
    pub max_rpm: u16,
    pub min_rpm: u16,
    pub avg_coolant_temp: f32,
    pub max_coolant_temp: i8,
    pub min_coolant_temp: i8,
    pub avg_fuel_level: f32,
    pub min_fuel_level: u8,
    pub total_distance_km: f32,
    pub total_fuel_consumed_liters: f32,
    pub harsh_braking_events: i32,
    pub rapid_acceleration_events: i32,
    pub overspeeding_events: i32,
}