use crate::alert::types::{Alert, AlertSeverity, AlertType};
use crate::sensors::types::SensorEvent;
use tracing::{info, warn};

pub struct SensorTriggerEngine;

impl SensorTriggerEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn trigger_from_sensor(&self, sensor_event: &SensorEvent) -> Option<Alert> {
        match &sensor_event.values {
            crate::sensors::types::SensorValues::Imu(imu) => {
                let g_force =
                    (imu.accel_x.powi(2) + imu.accel_y.powi(2) + imu.accel_z.powi(2)).sqrt();
                if g_force > 0.8 {
                    // Harsh braking
                    Some(Alert::new(
                        AlertType::HarshBraking,
                        AlertSeverity::Warning,
                        "Harsh braking detected",
                        &sensor_event.sensor_id,
                    ))
                } else if g_force > 0.6 {
                    // Rapid acceleration
                    Some(Alert::new(
                        AlertType::RapidAcceleration,
                        AlertSeverity::Warning,
                        "Rapid acceleration detected",
                        &sensor_event.sensor_id,
                    ))
                } else {
                    None
                }
            }
            crate::sensors::types::SensorValues::Obd(obd) => {
                if obd.speed_kmh > 120 {
                    // Over-speeding (adjust based on road type)
                    Some(Alert::new(
                        AlertType::OverSpeeding,
                        AlertSeverity::Warning,
                        "Over-speeding detected",
                        &sensor_event.sensor_id,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
