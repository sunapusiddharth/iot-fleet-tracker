use influxdb2::Client;
use influxdb2::models::DataPoint;
use crate::models::telemetry::TelemetryData;
use crate::models::health::HealthStatus;
use uuid::Uuid;
use chrono::Utc;

pub struct TimeseriesStore {
    client: Client,
    org: String,
    bucket: String,
}

impl TimeseriesStore {
    pub fn new(client: Client, org: &str, bucket: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client,
            org: org.to_string(),
            bucket: bucket.to_string(),
        })
    }
    
    pub async fn store_telemetry(&mut self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        let point = DataPoint::builder("telemetry")
            .tag("truck_id", telemetry.truck_id.to_string())
            .tag("scenario", telemetry.scenario.as_deref().unwrap_or("unknown"))
            .field("speed_kmh", telemetry.speed_kmh)
            .field("heading", telemetry.heading)
            .field("gps_latitude", telemetry.location.y())
            .field("gps_longitude", telemetry.location.x())
            .field("gps_altitude", telemetry.sensors.gps.altitude)
            .field("gps_satellites", telemetry.sensors.gps.satellites as i64)
            .field("obd_rpm", telemetry.sensors.obd.rpm as i64)
            .field("obd_coolant_temp", telemetry.sensors.obd.coolant_temp as i64)
            .field("obd_fuel_level", telemetry.sensors.obd.fuel_level as i64)
            .field("imu_accel_x", telemetry.sensors.imu.accel_x)
            .field("imu_accel_y", telemetry.sensors.imu.accel_y)
            .field("imu_accel_z", telemetry.sensors.imu.accel_z)
            .field("tpms_front_left_pressure", telemetry.sensors.tpms.front_left.pressure_psi)
            .field("tpms_front_left_temperature", telemetry.sensors.tpms.front_left.temperature_c)
            .field("tpms_front_right_pressure", telemetry.sensors.tpms.front_right.pressure_psi)
            .field("tpms_front_right_temperature", telemetry.sensors.tpms.front_right.temperature_c)
            .field("tpms_rear_left_pressure", telemetry.sensors.tpms.rear_left.pressure_psi)
            .field("tpms_rear_left_temperature", telemetry.sensors.tpms.rear_left.temperature_c)
            .field("tpms_rear_right_pressure", telemetry.sensors.tpms.rear_right.pressure_psi)
            .field("tpms_rear_right_temperature", telemetry.sensors.tpms.rear_right.temperature_c)
            .timestamp(telemetry.timestamp.timestamp_nanos())
            .build()?;
        
        self.client.write(&self.org, &self.bucket, stream::iter(vec![point])).await?;
        Ok(())
    }
    
    pub async fn store_health_status(&mut self, health_status: &HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
        let point = DataPoint::builder("health_status")
            .tag("truck_id", health_status.truck_id.to_string())
            .tag("status", format!("{:?}", health_status.status))
            .field("cpu_percent", health_status.resources.cpu_percent)
            .field("memory_percent", health_status.resources.memory_percent)
            .field("disk_percent", health_status.resources.disk_percent)
            .field("temperature_c", health_status.resources.temperature_c)
            .field("uptime_sec", health_status.resources.uptime_sec as i64)
            .field("load_average_1m", health_status.resources.load_average.0)
            .field("load_average_5m", health_status.resources.load_average.1)
            .field("load_average_15m", health_status.resources.load_average.2)
            .field("active_alerts", health_status.alerts.len() as i64)
            .timestamp(health_status.timestamp.timestamp_nanos())
            .build()?;
        
        self.client.write(&self.org, &self.bucket, stream::iter(vec![point])).await?;
        Ok(())
    }
    
    pub async fn get_telemetry_statistics(&mut self, truck_id: Uuid, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>) -> Result<crate::services::telemetry_service::TelemetryStatistics, Box<dyn std::error::Error>> {
        // In production, use InfluxDB Flux queries to calculate statistics
        // For now, return dummy data
        Ok(crate::services::telemetry_service::TelemetryStatistics {
            truck_id,
            start_time,
            end_time,
            avg_speed_kmh: 60.0,
            max_speed_kmh: 80.0,
            min_speed_kmh: 40.0,
            avg_rpm: 2000.0,
            max_rpm: 3000,
            min_rpm: 1000,
            avg_coolant_temp: 85.0,
            max_coolant_temp: 95,
            min_coolant_temp: 75,
            avg_fuel_level: 75.0,
            min_fuel_level: 50,
            total_distance_km: 100.0,
            total_fuel_consumed_liters: 20.0,
            harsh_braking_events: 5,
            rapid_acceleration_events: 3,
            overspeeding_events: 2,
        })
    }
}