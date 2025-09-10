use crate::simulator::types::{HealthStatus, NetworkQuality, HealthAlert, AlertSeverity};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct HealthGenerator {
    truck_id: String,
    rng: rand::rngs::ThreadRng,
    scenario_duration: std::time::Duration,
    scenario_start: std::time::Instant,
}

impl HealthGenerator {
    pub fn new(truck_id: &str) -> Self {
        Self {
            truck_id: truck_id.to_string(),
            rng: rand::thread_rng(),
            scenario_duration: std::time::Duration::from_secs(0),
            scenario_start: std::time::Instant::now(),
        }
    }

    pub fn set_scenario(&mut self, scenario: &str, duration_minutes: u32) {
        self.scenario_duration = std::time::Duration::from_secs(duration_minutes as u64 * 60);
        self.scenario_start = std::time::Instant::now();
    }

    pub fn generate_health_status(&mut self, scenario: &str) -> HealthStatus {
        let elapsed = self.scenario_start.elapsed();
        let progress = elapsed.as_secs_f32() / self.scenario_duration.as_secs_f32();
        
        // Base values
        let mut cpu_percent = 45.0 + self.rng.gen_range(-10.0..10.0);
        let mut memory_percent = 60.0 + self.rng.gen_range(-10.0..10.0);
        let mut disk_percent = 70.0 + self.rng.gen_range(-5.0..5.0);
        let mut temperature_c = 45.0 + self.rng.gen_range(-5.0..10.0);
        
        // Modify based on scenario and progress
        match scenario {
            "system_failure" => {
                if progress < 0.3 {
                    // Normal at start
                } else if progress < 0.6 {
                    // Gradually increase
                    cpu_percent = 70.0 + self.rng.gen_range(-5.0..10.0);
                    temperature_c = 65.0 + self.rng.gen_range(-5.0..10.0);
                } else {
                    // Critical at end
                    cpu_percent = 95.0 + self.rng.gen_range(-5.0..5.0);
                    temperature_c = 85.0 + self.rng.gen_range(-5.0..10.0);
                    memory_percent = 85.0 + self.rng.gen_range(-5.0..10.0);
                }
            }
            "emergency_braking" => {
                cpu_percent = 65.0 + self.rng.gen_range(-10.0..15.0); // Higher during braking
            }
            "rapid_acceleration" => {
                cpu_percent = 70.0 + self.rng.gen_range(-10.0..15.0); // Higher during acceleration
                temperature_c = 55.0 + self.rng.gen_range(-5.0..15.0);
            }
            _ => {}
        }

        let network_quality = NetworkQuality {
            latency_ms: 50.0 + self.rng.gen_range(-20.0..50.0),
            packet_loss_percent: 0.5 + self.rng.gen_range(-0.5..2.0),
            bandwidth_kbps: 1000.0 + self.rng.gen_range(-200.0..500.0),
        };

        let alerts = self.generate_alerts(scenario, progress);

        HealthStatus {
            cpu_percent: cpu_percent.clamp(0.0, 100.0),
            memory_percent: memory_percent.clamp(0.0, 100.0),
            disk_percent: disk_percent.clamp(0.0, 100.0),
            temperature_c: temperature_c.clamp(0.0, 100.0),
            network_quality,
            alerts,
        }
    }

    fn generate_alerts(&mut self, scenario: &str, progress: f32) -> Vec<HealthAlert> {
        let mut alerts = Vec::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;

        match scenario {
            "system_failure" => {
                if progress > 0.5 && self.rng.gen_bool(0.3) {
                    alerts.push(HealthAlert {
                        alert_type: "high_cpu_usage".to_string(),
                        severity: AlertSeverity::Warning,
                        message: "CPU usage is high".to_string(),
                        timestamp,
                    });
                }
                
                if progress > 0.7 && self.rng.gen_bool(0.5) {
                    alerts.push(HealthAlert {
                        alert_type: "high_temperature".to_string(),
                        severity: AlertSeverity::Critical,
                        message: "System temperature critical".to_string(),
                        timestamp,
                    });
                }
            }
            "emergency_braking" => {
                if self.rng.gen_bool(0.1) {
                    alerts.push(HealthAlert {
                        alert_type: "harsh_braking".to_string(),
                        severity: AlertSeverity::Warning,
                        message: "Harsh braking detected".to_string(),
                        timestamp,
                    });
                }
            }
            "rapid_acceleration" => {
                if self.rng.gen_bool(0.1) {
                    alerts.push(HealthAlert {
                        alert_type: "rapid_acceleration".to_string(),
                        severity: AlertSeverity::Warning,
                        message: "Rapid acceleration detected".to_string(),
                        timestamp,
                    });
                }
            }
            _ => {
                // Random occasional alerts
                if self.rng.gen_bool(0.01) {
                    alerts.push(HealthAlert {
                        alert_type: "low_disk_space".to_string(),
                        severity: AlertSeverity::Warning,
                        message: "Disk space low".to_string(),
                        timestamp,
                    });
                }
                
                if self.rng.gen_bool(0.005) {
                    alerts.push(HealthAlert {
                        alert_type: "network_latency".to_string(),
                        severity: AlertSeverity::Warning,
                        message: "Network latency high".to_string(),
                        timestamp,
                    });
                }
            }
        }

        alerts
    }
}