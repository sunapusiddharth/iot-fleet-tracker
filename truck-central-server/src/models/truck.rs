use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use geo::Point;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Truck {
    pub id: Uuid,
    pub truck_id: String,
    #[validate(length(min = 1, max = 100))]
    pub model: String,
    #[validate(length(min = 1, max = 100))]
    pub make: String,
    #[validate(length(min = 1, max = 50))]
    pub year: String,
    #[validate(length(min = 1, max = 50))]
    pub license_plate: String,
    pub vin: String,
    pub fleet_id: Option<Uuid>,
    pub driver_id: Option<Uuid>,
    pub status: TruckStatus,
    pub last_seen: DateTime<Utc>,
    pub location: Point<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TruckStatus {
    Online,
    Offline,
    Maintenance,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTruckRequest {
    #[validate(length(min = 1, max = 100))]
    pub model: String,
    #[validate(length(min = 1, max = 100))]
    pub make: String,
    #[validate(length(min = 1, max = 50))]
    pub year: String,
    #[validate(length(min = 1, max = 50))]
    pub license_plate: String,
    #[validate(length(min = 1, max = 100))]
    pub vin: String,
    pub fleet_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTruckRequest {
    #[validate(length(min = 1, max = 100))]
    pub model: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub make: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub year: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub license_plate: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub vin: Option<String>,
    pub fleet_id: Option<Uuid>,
    pub driver_id: Option<Uuid>,
    pub status: Option<TruckStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruckSummary {
    pub id: Uuid,
    pub truck_id: String,
    pub model: String,
    pub make: String,
    pub license_plate: String,
    pub status: TruckStatus,
    pub last_seen: DateTime<Utc>,
    pub location: Point<f64>,
    pub speed_kmh: Option<f32>,
    pub heading: Option<f32>,
    pub active_alerts: i32,
    pub health_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruckDetail {
    pub summary: TruckSummary,
    pub sensors: Option<crate::models::telemetry::SensorData>,
    pub cameras: Option<crate::models::telemetry::CameraData>,
    pub ml_events: Vec<crate::models::ml::MlEvent>,
    pub health_status: Option<crate::models::health::HealthStatus>,
    pub recent_trips: Vec<TripSummary>,
    pub maintenance_history: Vec<MaintenanceRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripSummary {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub start_location: Point<f64>,
    pub end_location: Point<f64>,
    pub distance_km: f32,
    pub duration_minutes: i32,
    pub average_speed_kmh: f32,
    pub max_speed_kmh: f32,
    pub fuel_consumed_liters: f32,
    pub events_count: i32,
    pub alerts_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceRecord {
    pub id: Uuid,
    pub maintenance_type: String,
    pub description: String,
    pub performed_at: DateTime<Utc>,
    pub next_due_date: Option<DateTime<Utc>>,
    pub cost: f32,
    pub performed_by: String,
    pub mileage: i32,
}