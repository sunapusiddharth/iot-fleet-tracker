use crate::models::telemetry::TelemetryData;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;
use crate::models::alert::{Alert, AlertType, AlertSeverity, AlertStatus, AlertAction, ActionType};
use uuid::Uuid;
use chrono::Utc;

pub struct AlertProcessor;

impl AlertProcessor {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn process_telemetry(&self, telemetry: &TelemetryData) -> Result<Vec<Alert>, Box<dyn std::error::Error>> {
        let mut alerts = Vec::new();
        
        // Check for harsh braking
        let g_force = (telemetry.sensors.imu.accel_x.powi(2) + 
                      telemetry.sensors.imu.accel_y.powi(2) + 
                      telemetry.sensors.imu.accel_z.powi(2)).sqrt();
        if g_force > 0.8 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-harsh_braking-{}", telemetry.truck_id, Utc::now().timestamp_nanos()),
                truck_id: telemetry.truck_id,
                alert_type: AlertType::HarshBraking,
                severity: AlertSeverity::Warning,
                message: "Harsh braking detected".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "telemetry_processor".to_string(),
                context: serde_json::json!({
                    "g_force": g_force,
                    "speed_kmh": telemetry.speed_kmh,
                    "location": {
                        "lat": telemetry.location.y(),
                        "lon": telemetry.location.x()
                    }
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-buzzer", Uuid::new_v4()),
                        action_type: ActionType::TriggerBuzzer,
                        target: "buzzer_1".to_string(),
                        parameters: serde_json::json!({
                            "duration_ms": 1000,
                            "pattern": "pulse"
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    }
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }
        
        // Check for rapid acceleration
        if telemetry.sensors.imu.accel_x > 0.6 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-rapid_acceleration-{}", telemetry.truck_id, Utc::now().timestamp_nanos()),
                truck_id: telemetry.truck_id,
                alert_type: AlertType::RapidAcceleration,
                severity: AlertSeverity::Warning,
                message: "Rapid acceleration detected".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "telemetry_processor".to_string(),
                context: serde_json::json!({
                    "acceleration": telemetry.sensors.imu.accel_x,
                    "speed_kmh": telemetry.speed_kmh,
                    "location": {
                        "lat": telemetry.location.y(),
                        "lon": telemetry.location.x()
                    }
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-led", Uuid::new_v4()),
                        action_type: ActionType::FlashLed,
                        target: "led_yellow".to_string(),
                        parameters: serde_json::json!({
                            "duration_ms": 2000,
                            "pattern": "blink"
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    }
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }
        
        // Check for overspeeding
        if telemetry.speed_kmh > 100.0 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-overspeeding-{}", telemetry.truck_id, Utc::now().timestamp_nanos()),
                truck_id: telemetry.truck_id,
                alert_type: AlertType::OverSpeeding,
                severity: AlertSeverity::Warning,
                message: "Overspeeding detected".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "telemetry_processor".to_string(),
                context: serde_json::json!({
                    "speed_kmh": telemetry.speed_kmh,
                    "location": {
                        "lat": telemetry.location.y(),
                        "lon": telemetry.location.x()
                    }
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-display", Uuid::new_v4()),
                        action_type: ActionType::ShowOnDisplay,
                        target: "display_1".to_string(),
                        parameters: serde_json::json!({
                            "message": "SLOW DOWN",
                            "duration_ms": 5000
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    }
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }
        
        Ok(alerts)
    }
    
    pub async fn process_ml_event(&self, ml_event: &MlEvent) -> Result<Vec<Alert>, Box<dyn std::error::Error>> {
        let mut alerts = Vec::new();
        
        // Check for drowsy driver
        if let crate::models::ml::MlResult::Drowsiness { is_drowsy, eye_closure_ratio } = &ml_event.result {
            if *is_drowsy && ml_event.confidence > 0.8 {
                alerts.push(Alert {
                    id: Uuid::new_v4(),
                    alert_id: format!("alert-{}-drowsy_driver-{}", ml_event.truck_id, Utc::now().timestamp_nanos()),
                    truck_id: ml_event.truck_id,
                    alert_type: AlertType::DrowsyDriver,
                    severity: AlertSeverity::Emergency,
                    message: "Drowsy driver detected - immediate attention required".to_string(),
                    triggered_at: Utc::now(),
                    acknowledged_at: None,
                    resolved_at: None,
                    source: "ml_processor".to_string(),
                    context: serde_json::json!({
                        "eye_closure_ratio": eye_closure_ratio,
                        "confidence": ml_event.confidence,
                        "location": ml_event.meta.location
                    }),
                    actions: vec![
                        AlertAction {
                            action_id: format!("action-{}-buzzer", Uuid::new_v4()),
                            action_type: ActionType::TriggerBuzzer,
                            target: "buzzer_1".to_string(),
                            parameters: serde_json::json!({
                                "duration_ms": 2000,
                                "pattern": "pulse",
                                "pulse_count": 10
                            }),
                            executed_at: None,
                            success: false,
                            error: None,
                        },
                        AlertAction {
                            action_id: format!("action-{}-led", Uuid::new_v4()),
                            action_type: ActionType::FlashLed,
                            target: "led_red".to_string(),
                            parameters: serde_json::json!({
                                "duration_ms": 10000,
                                "pattern": "blink",
                                "blink_count": 20
                            }),
                            executed_at: None,
                            success: false,
                            error: None,
                        },
                        AlertAction {
                            action_id: format!("action-{}-display", Uuid::new_v4()),
                            action_type: ActionType::ShowOnDisplay,
                            target: "display_1".to_string(),
                            parameters: serde_json::json!({
                                "message": "DROWSY DRIVER - PULL OVER",
                                "duration_ms": 10000
                            }),
                            executed_at: None,
                            success: false,
                            error: None,
                        },
                    ],
                    status: AlertStatus::Triggered,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                });
            }
        }
        
        // Check for lane departure
        if let crate::models::ml::MlResult::LaneDeparture { is_departing, deviation_pixels } = &ml_event.result {
            if *is_departing && *deviation_pixels > 50 && ml_event.confidence > 0.7 {
                alerts.push(Alert {
                    id: Uuid::new_v4(),
                    alert_id: format!("alert-{}-lane_departure-{}", ml_event.truck_id, Utc::now().timestamp_nanos()),
                    truck_id: ml_event.truck_id,
                    alert_type: AlertType::LaneDeparture,
                    severity: AlertSeverity::Critical,
                    message: "Lane departure detected - correct steering".to_string(),
                    triggered_at: Utc::now(),
                    acknowledged_at: None,
                    resolved_at: None,
                    source: "ml_processor".to_string(),
                    context: serde_json::json!({
                        "deviation_pixels": deviation_pixels,
                        "confidence": ml_event.confidence,
                        "location": ml_event.meta.location
                    }),
                    actions: vec![
                        AlertAction {
                            action_id: format!("action-{}-buzzer", Uuid::new_v4()),
                            action_type: ActionType::TriggerBuzzer,
                            target: "buzzer_1".to_string(),
                            parameters: serde_json::json!({
                                "duration_ms": 500,
                                "pattern": "solid"
                            }),
                            executed_at: None,
                            success: false,
                            error: None,
                        },
                        AlertAction {
                            action_id: format!("action-{}-display", Uuid::new_v4()),
                            action_type: ActionType::ShowOnDisplay,
                            target: "display_1".to_string(),
                            parameters: serde_json::json!({
                                "message": "LANE DEPARTURE",
                                "duration_ms": 3000
                            }),
                            executed_at: None,
                            success: false,
                            error: None,
                        },
                    ],
                    status: AlertStatus::Triggered,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                });
            }
        }
        
        // Check for cargo tampering
        if let crate::models::ml::MlResult::CargoTamper { is_tampered, motion_score } = &ml_event.result {
            if *is_tampered && ml_event.confidence > 0.8 {
                alerts.push(Alert {
                    id: Uuid::new_v4(),
                    alert_id: format!("alert-{}-cargo_tamper-{}", ml_event.truck_id, Utc::now().timestamp_nanos()),
                    truck_id: ml_event.truck_id,
                    alert_type: AlertType::CargoTamper,
                    severity: AlertSeverity::Critical,
                    message: "Cargo tampering detected - check cargo area".to_string(),
                    triggered_at: Utc::now(),
                    acknowledged_at: None,
                    resolved_at: None,
                    source: "ml_processor".to_string(),
                    context: serde_json::json!({
                        "motion_score": motion_score,
                        "confidence": ml_event.confidence,
                        "location": ml_event.meta.location
                    }),
                    actions: vec![
                        AlertAction {
                            action_id: format!("action-{}-buzzer", Uuid::new_v4()),
                            action_type: ActionType::TriggerBuzzer,
                            target: "buzzer_2".to_string(),
                            parameters: serde_json::json!({
                                "duration_ms": 1000,
                                "pattern": "pulse"
                            }),
                            executed_at: None,
                            success: false,
                            error: None,
                        },
                        AlertAction {
                            action_id: format!("action-{}-led", Uuid::new_v4()),
                            action_type: ActionType::FlashLed,
                            target: "led_orange".to_string(),
                            parameters: serde_json::json!({
                                "duration_ms": 5000,
                                "pattern": "blink"
                            }),
                            executed_at: None,
                            success: false,
                            error: None,
                        },
                    ],
                    status: AlertStatus::Triggered,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                });
            }
        }
        
        Ok(alerts)
    }
    
    pub async fn process_health_status(&self, health_status: &HealthStatus) -> Result<Vec<Alert>, Box<dyn std::error::Error>> {
        let mut alerts = Vec::new();
        
        // Check for high temperature
        if health_status.resources.temperature_c > 80.0 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-high_temperature-{}", health_status.truck_id, Utc::now().timestamp_nanos()),
                truck_id: health_status.truck_id,
                alert_type: AlertType::HighTemperature,
                severity: AlertSeverity::Emergency,
                message: "System temperature critical - shutdown imminent".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "health_processor".to_string(),
                context: serde_json::json!({
                    "temperature_c": health_status.resources.temperature_c,
                    "thermal_throttling": health_status.resources.thermal_throttling,
                    "uptime_sec": health_status.resources.uptime_sec
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-relay", Uuid::new_v4()),
                        action_type: ActionType::ActivateRelay,
                        target: "relay_1".to_string(),
                        parameters: serde_json::json!({
                            "activate": true,
                            "duration_ms": 0 // Stay on
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    },
                    AlertAction {
                        action_id: format!("action-{}-display", Uuid::new_v4()),
                        action_type: ActionType::ShowOnDisplay,
                        target: "display_1".to_string(),
                        parameters: serde_json::json!({
                            "message": "TEMP CRITICAL - SHUTDOWN",
                            "duration_ms": 10000
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    },
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        } else if health_status.resources.temperature_c > 70.0 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-high_temperature-{}", health_status.truck_id, Utc::now().timestamp_nanos()),
                truck_id: health_status.truck_id,
                alert_type: AlertType::HighTemperature,
                severity: AlertSeverity::Critical,
                message: "System temperature high - reduce load".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "health_processor".to_string(),
                context: serde_json::json!({
                    "temperature_c": health_status.resources.temperature_c,
                    "thermal_throttling": health_status.resources.thermal_throttling,
                    "uptime_sec": health_status.resources.uptime_sec
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-display", Uuid::new_v4()),
                        action_type: ActionType::ShowOnDisplay,
                        target: "display_1".to_string(),
                        parameters: serde_json::json!({
                            "message": "HIGH TEMP - REDUCE LOAD",
                            "duration_ms": 5000
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    },
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }
        
        // Check for low disk space
        if health_status.resources.disk_percent > 95.0 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-low_disk_space-{}", health_status.truck_id, Utc::now().timestamp_nanos()),
                truck_id: health_status.truck_id,
                alert_type: AlertType::LowDiskSpace,
                severity: AlertSeverity::Critical,
                message: "Disk space critical - deleting old data".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "health_processor".to_string(),
                context: serde_json::json!({
                    "disk_percent": health_status.resources.disk_percent,
                    "disk_used_gb": health_status.resources.disk_used_gb,
                    "disk_total_gb": health_status.resources.disk_total_gb
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-log", Uuid::new_v4()),
                        action_type: ActionType::LogToServer,
                        target: "server".to_string(),
                        parameters: serde_json::json!({
                            "message": "Disk space critical - triggering cleanup"
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    },
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        } else if health_status.resources.disk_percent > 90.0 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-low_disk_space-{}", health_status.truck_id, Utc::now().timestamp_nanos()),
                truck_id: health_status.truck_id,
                alert_type: AlertType::LowDiskSpace,
                severity: AlertSeverity::Warning,
                message: "Disk space low - consider cleanup".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "health_processor".to_string(),
                context: serde_json::json!({
                    "disk_percent": health_status.resources.disk_percent,
                    "disk_used_gb": health_status.resources.disk_used_gb,
                    "disk_total_gb": health_status.resources.disk_total_gb
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-log", Uuid::new_v4()),
                        action_type: ActionType::LogToServer,
                        target: "server".to_string(),
                        parameters: serde_json::json!({
                            "message": "Disk space low - monitoring"
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    },
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }
        
        // Check for high CPU usage
        if health_status.resources.cpu_percent > 95.0 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-high_cpu_usage-{}", health_status.truck_id, Utc::now().timestamp_nanos()),
                truck_id: health_status.truck_id,
                alert_type: AlertType::HighCpuUsage,
                severity: AlertSeverity::Critical,
                message: "CPU usage critical - system may become unresponsive".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "health_processor".to_string(),
                context: serde_json::json!({
                    "cpu_percent": health_status.resources.cpu_percent,
                    "load_average": {
                        "1m": health_status.resources.load_average.0,
                        "5m": health_status.resources.load_average.1,
                        "15m": health_status.resources.load_average.2
                    }
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-log", Uuid::new_v4()),
                        action_type: ActionType::LogToServer,
                        target: "server".to_string(),
                        parameters: serde_json::json!({
                            "message": "CPU usage critical - triggering degradation"
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    },
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        } else if health_status.resources.cpu_percent > 85.0 {
            alerts.push(Alert {
                id: Uuid::new_v4(),
                alert_id: format!("alert-{}-high_cpu_usage-{}", health_status.truck_id, Utc::now().timestamp_nanos()),
                truck_id: health_status.truck_id,
                alert_type: AlertType::HighCpuUsage,
                severity: AlertSeverity::Warning,
                message: "CPU usage high - consider reducing load".to_string(),
                triggered_at: Utc::now(),
                acknowledged_at: None,
                resolved_at: None,
                source: "health_processor".to_string(),
                context: serde_json::json!({
                    "cpu_percent": health_status.resources.cpu_percent,
                    "load_average": {
                        "1m": health_status.resources.load_average.0,
                        "5m": health_status.resources.load_average.1,
                        "15m": health_status.resources.load_average.2
                    }
                }),
                actions: vec![
                    AlertAction {
                        action_id: format!("action-{}-log", Uuid::new_v4()),
                        action_type: ActionType::LogToServer,
                        target: "server".to_string(),
                        parameters: serde_json::json!({
                            "message": "CPU usage high - monitoring"
                        }),
                        executed_at: None,
                        success: false,
                        error: None,
                    },
                ],
                status: AlertStatus::Triggered,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        }
        
        Ok(alerts)
    }
    
    pub async fn process_alert(&self, alert: Alert) -> Result<Alert, Box<dyn std::error::Error>> {
        // In production, add additional processing logic here
        // For now, just return the alert as is
        Ok(alert)
    }
}