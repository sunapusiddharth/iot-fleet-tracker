use crate::models::truck::{Truck, CreateTruckRequest, UpdateTruckRequest, TruckSummary, TruckDetail, TripSummary, MaintenanceRecord};
use crate::server::storage::StorageManager;
use uuid::Uuid;
use chrono::Utc;
use geo::Point;
use validator::Validate;
use std::sync::Arc;

pub struct TruckService {
    storage_manager: Arc<StorageManager>,
}

impl TruckService {
    pub fn new(storage_manager: Arc<StorageManager>) -> Self {
        Self {
            storage_manager,
        }
    }

    pub async fn create_truck(&self, request: CreateTruckRequest) -> Result<Truck, Box<dyn std::error::Error>> {
        // Validate request
        request.validate()?;

        let truck = Truck {
            id: Uuid::new_v4(),
            truck_id: format!("TRK-{}", Uuid::new_v4().to_string()[..8].to_uppercase()),
            model: request.model,
            make: request.make,
            year: request.year,
            license_plate: request.license_plate,
            vin: request.vin,
            fleet_id: request.fleet_id,
            driver_id: None,
            status: crate::models::truck::TruckStatus::Offline,
            last_seen: Utc::now(),
            location: Point::from((0.0, 0.0)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.storage_manager.store_truck(&truck).await?;

        metrics::counter!("trucks_created_total").increment(1);
        tracing::info!(truck_id=%truck.id, "✅ Truck created successfully");

        Ok(truck)
    }

    pub async fn get_truck(&self, id: &Uuid) -> Result<Option<Truck>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let truck = store.get_truck(id).await?;
        
        if let Some(ref t) = truck {
            metrics::counter!("trucks_retrieved_total").increment(1);
            tracing::info!(truck_id=%id, "📤 Truck retrieved");
        }

        Ok(truck)
    }

    pub async fn update_truck(&self, id: &Uuid, request: UpdateTruckRequest) -> Result<Truck, Box<dyn std::error::Error>> {
        let existing_truck = self.get_truck(id).await?
            .ok_or_else(|| "Truck not found".to_string())?;

        let updated_truck = Truck {
            id: existing_truck.id,
            truck_id: existing_truck.truck_id,
            model: request.model.unwrap_or(existing_truck.model),
            make: request.make.unwrap_or(existing_truck.make),
            year: request.year.unwrap_or(existing_truck.year),
            license_plate: request.license_plate.unwrap_or(existing_truck.license_plate),
            vin: request.vin.unwrap_or(existing_truck.vin),
            fleet_id: request.fleet_id.or(existing_truck.fleet_id),
            driver_id: request.driver_id.or(existing_truck.driver_id),
            status: request.status.unwrap_or(existing_truck.status),
            last_seen: existing_truck.last_seen,
            location: existing_truck.location,
            created_at: existing_truck.created_at,
            updated_at: Utc::now(),
        };

        self.storage_manager.store_truck(&updated_truck).await?;

        metrics::counter!("trucks_updated_total").increment(1);
        tracing::info!(truck_id=%id, "✅ Truck updated successfully");

        Ok(updated_truck)
    }

    pub async fn delete_truck(&self, id: &Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let result = store.delete_truck(id).await?;
        
        if result {
            metrics::counter!("trucks_deleted_total").increment(1);
            tracing::info!(truck_id=%id, "🗑️  Truck deleted");
        }

        Ok(result)
    }

    pub async fn list_trucks(&self, limit: i64, offset: i64) -> Result<Vec<TruckSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let trucks = store.list_trucks(limit, offset).await?;
        
        metrics::counter!("trucks_listed_total").increment(1);
        tracing::info!(count=%trucks.len(), "📤 Trucks listed");

        Ok(trucks)
    }

    pub async fn get_truck_detail(&self, id: &Uuid) -> Result<Option<TruckDetail>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let truck = store.get_truck(id).await?;
        if truck.is_none() {
            return Ok(None);
        }
        let truck = truck.unwrap();

        // Get recent telemetry
        let sensors = store.get_recent_telemetry(&truck.id, 1).await?.first().cloned();
        let cameras = store.get_recent_camera_data(&truck.id, 1).await?.first().cloned();

        // Get recent ML events
        let ml_events = store.get_recent_ml_events(&truck.id, 10).await?;

        // Get health status
        let health_status = store.get_recent_health_status(&truck.id, 1).await?.first().cloned();

        // Get recent trips (last 30 days)
        let recent_trips = self.get_recent_trips(&truck.id, 30).await?;

        // Get maintenance history
        let maintenance_history = self.get_maintenance_history(&truck.id).await?;

        let detail = TruckDetail {
            summary: TruckSummary {
                id: truck.id,
                truck_id: truck.truck_id,
                model: truck.model,
                make: truck.make,
                license_plate: truck.license_plate,
                status: truck.status,
                last_seen: truck.last_seen,
                location: truck.location,
                speed_kmh: sensors.as_ref().map(|s| s.sensors.obd.speed_kmh as f32),
                heading: sensors.as_ref().map(|s| s.sensors.gps.heading),
                active_alerts: store.get_active_alerts_count(&truck.id).await?,
                health_score: self.calculate_health_score(&health_status).await,
            },
            sensors,
            cameras,
            ml_events,
            health_status,
            recent_trips,
            maintenance_history,
        };

        metrics::counter!("trucks_details_retrieved_total").increment(1);
        tracing::info!(truck_id=%id, "📤 Truck detail retrieved");

        Ok(Some(detail))
    }

    async fn get_recent_trips(&self, truck_id: &Uuid, days: i32) -> Result<Vec<TripSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let trips = store.get_recent_trips(truck_id, days).await?;
        Ok(trips)
    }

    async fn get_maintenance_history(&self, truck_id: &Uuid) -> Result<Vec<MaintenanceRecord>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let maintenance = store.get_maintenance_history(truck_id).await?;
        Ok(maintenance)
    }

    async fn calculate_health_score(&self, health_status: &Option<crate::models::health::HealthStatus>) -> f32 {
        if let Some(status) = health_status {
            let cpu_score = (100.0 - status.resources.cpu_percent) / 100.0;
            let memory_score = (100.0 - status.resources.memory_percent) / 100.0;
            let disk_score = (100.0 - status.resources.disk_percent) / 100.0;
            let temp_score = if status.resources.temperature_c > 75.0 {
                0.5
            } else if status.resources.temperature_c > 65.0 {
                0.75
            } else {
                1.0
            };

            let avg_score = (cpu_score + memory_score + disk_score + temp_score) / 4.0;
            (avg_score * 100.0).max(0.0).min(100.0)
        } else {
            85.0 // Default score if no health data
        }
    }
}



impl TruckService {
    pub fn new(storage_manager: Arc<StorageManager>) -> Self {
        Self {
            storage_manager,
        }
    }

    pub async fn create_truck(&self, request: CreateTruckRequest) -> Result<Truck, Box<dyn std::error::Error>> {
        // Validate request
        request.validate()?;

        let truck_id = Uuid::new_v4();
        let now = Utc::now();
        
        let truck = Truck {
            id: truck_id,
            truck_id: format!("TRK-{}", truck_id.as_simple()),
            model: request.model,
            make: request.make,
            year: request.year,
            license_plate: request.license_plate,
            vin: request.vin,
            fleet_id: request.fleet_id,
            driver_id: None,
            status: crate::models::truck::TruckStatus::Offline,
            last_seen: now,
            location: Point::from([0.0, 0.0]),
            created_at: now,
            updated_at: now,
        };

        self.storage_manager.store_truck(&truck).await?;

        Ok(truck)
    }

    pub async fn get_truck(&self, id: Uuid) -> Result<Truck, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let truck = store.get_truck(id).await?;
        
        Ok(truck)
    }

    pub async fn update_truck(&self, id: Uuid, request: UpdateTruckRequest) -> Result<Truck, Box<dyn std::error::Error>> {
        // Validate request
        request.validate()?;

        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let mut truck = store.get_truck(id).await?;
        
        if let Some(model) = request.model {
            truck.model = model;
        }
        if let Some(make) = request.make {
            truck.make = make;
        }
        if let Some(year) = request.year {
            truck.year = year;
        }
        if let Some(license_plate) = request.license_plate {
            truck.license_plate = license_plate;
        }
        if let Some(vin) = request.vin {
            truck.vin = vin;
        }
        if let Some(fleet_id) = request.fleet_id {
            truck.fleet_id = Some(fleet_id);
        }
        if let Some(driver_id) = request.driver_id {
            truck.driver_id = Some(driver_id);
        }
        if let Some(status) = request.status {
            truck.status = status;
        }
        
        truck.updated_at = Utc::now();
        
        store.update_truck(&truck).await?;
        
        Ok(truck)
    }

    pub async fn delete_truck(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        store.delete_truck(id).await?;
        
        Ok(())
    }

    pub async fn list_trucks(&self, limit: i64, offset: i64) -> Result<Vec<TruckSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let trucks = store.list_trucks(limit, offset).await?;
        
        Ok(trucks)
    }

    pub async fn get_truck_detail(&self, id: Uuid) -> Result<TruckDetail, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let truck = store.get_truck(id).await?;
        
        // Get latest telemetry
        let telemetry = store.get_latest_telemetry(id).await.ok();
        
        // Get latest ML events
        let ml_events = store.get_recent_ml_events(id, 10).await?;
        
        // Get latest health status
        let health_status = store.get_latest_health_status(id).await.ok();
        
        // Get recent trips
        let recent_trips = store.get_recent_trips(id, 5).await?;
        
        // Get maintenance history
        let maintenance_history = store.get_maintenance_history(id, 10).await?;
        
        let summary = TruckSummary {
            id: truck.id,
            truck_id: truck.truck_id,
            model: truck.model.clone(),
            make: truck.make.clone(),
            license_plate: truck.license_plate.clone(),
            status: truck.status.clone(),
            last_seen: truck.last_seen,
            location: truck.location,
            speed_kmh: telemetry.as_ref().map(|t| t.speed_kmh),
            heading: telemetry.as_ref().map(|t| t.heading),
            active_alerts: store.get_active_alerts_count(id).await?,
            health_score: health_status.as_ref().map(|h| calculate_health_score(h)).unwrap_or(100.0),
        };
        
        Ok(TruckDetail {
            summary,
            sensors: telemetry.map(|t| t.sensors),
            cameras: telemetry.map(|t| t.cameras),
            ml_events,
            health_status,
            recent_trips,
            maintenance_history,
        })
    }

    pub async fn get_truck_trips(&self, id: Uuid, limit: i64, offset: i64) -> Result<Vec<TripSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let trips = store.get_trips(id, limit, offset).await?;
        
        Ok(trips)
    }
}

fn calculate_health_score(health_status: &crate::models::health::HealthStatus) -> f32 {
    let mut score = 100.0;
    
    // Deduct points for high CPU usage
    if health_status.resources.cpu_percent > 80.0 {
        score -= (health_status.resources.cpu_percent - 80.0) * 2.0;
    }
    
    // Deduct points for high memory usage
    if health_status.resources.memory_percent > 85.0 {
        score -= (health_status.resources.memory_percent - 85.0) * 1.5;
    }
    
    // Deduct points for high disk usage
    if health_status.resources.disk_percent > 90.0 {
        score -= (health_status.resources.disk_percent - 90.0) * 3.0;
    }
    
    // Deduct points for high temperature
    if health_status.resources.temperature_c > 70.0 {
        score -= (health_status.resources.temperature_c - 70.0) * 1.0;
    }
    
    // Deduct points for critical alerts
    let critical_alerts = health_status.alerts.iter()
        .filter(|a| a.severity == crate::models::health::AlertSeverity::Critical)
        .count();
    score -= critical_alerts as f32 * 10.0;
    
    // Deduct points for warning alerts
    let warning_alerts = health_status.alerts.iter()
        .filter(|a| a.severity == crate::models::health::AlertSeverity::Warning)
        .count();
    score -= warning_alerts as f32 * 5.0;
    
    score.max(0.0).min(100.0)
}