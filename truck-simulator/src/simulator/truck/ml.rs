use crate::simulator::types::{MlEvent, MlResult};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MlGenerator {
    truck_id: String,
    rng: rand::rngs::ThreadRng,
}

impl MlGenerator {
    pub fn new(truck_id: &str) -> Self {
        Self {
            truck_id: truck_id.to_string(),
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_ml_events(&mut self, scenario: &str) -> Vec<MlEvent> {
        let mut events = Vec::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;

        // Generate events based on scenario
        match scenario {
            "drowsy_driver" => {
                events.push(MlEvent {
                    event_id: format!("ml-{}-drowsiness-{}", self.truck_id, timestamp),
                    model_name: "drowsiness".to_string(),
                    timestamp,
                    result: MlResult::Drowsiness {
                        is_drowsy: true,
                        eye_closure_ratio: 0.8 + self.rng.gen_range(-0.1..0.1),
                    },
                    confidence: 0.9 + self.rng.gen_range(-0.05..0.05),
                    latency_ms: 50.0 + self.rng.gen_range(-10.0..20.0),
                });
            }
            "lane_departure" => {
                events.push(MlEvent {
                    event_id: format!("ml-{}-lane_departure-{}", self.truck_id, timestamp),
                    model_name: "lane_departure".to_string(),
                    timestamp,
                    result: MlResult::LaneDeparture {
                        is_departing: true,
                        deviation_pixels: 60 + self.rng.gen_range(-10..20),
                    },
                    confidence: 0.85 + self.rng.gen_range(-0.05..0.05),
                    latency_ms: 45.0 + self.rng.gen_range(-5.0..15.0),
                });
            }
            "cargo_tamper" => {
                events.push(MlEvent {
                    event_id: format!("ml-{}-cargo_tamper-{}", self.truck_id, timestamp),
                    model_name: "cargo_tamper".to_string(),
                    timestamp,
                    result: MlResult::CargoTamper {
                        is_tampered: true,
                        motion_score: 0.75 + self.rng.gen_range(-0.1..0.1),
                    },
                    confidence: 0.92 + self.rng.gen_range(-0.05..0.05),
                    latency_ms: 60.0 + self.rng.gen_range(-10.0..20.0),
                });
            }
            "license_plate" => {
                events.push(MlEvent {
                    event_id: format!("ml-{}-license_plate-{}", self.truck_id, timestamp),
                    model_name: "license_plate".to_string(),
                    timestamp,
                    result: MlResult::LicensePlate {
                        plate_text: format!("TRK-{}", self.rng.gen_range(1000..9999)),
                        bounding_box: (0.1, 0.1, 0.2, 0.1),
                    },
                    confidence: 0.88 + self.rng.gen_range(-0.05..0.05),
                    latency_ms: 70.0 + self.rng.gen_range(-10.0..20.0),
                });
            }
            "normal_driving" => {
                // Occasionally generate normal events
                if self.rng.gen_bool(0.1) {
                    events.push(MlEvent {
                        event_id: format!("ml-{}-drowsiness-{}", self.truck_id, timestamp),
                        model_name: "drowsiness".to_string(),
                        timestamp,
                        result: MlResult::Drowsiness {
                            is_drowsy: false,
                            eye_closure_ratio: 0.2 + self.rng.gen_range(-0.1..0.1),
                        },
                        confidence: 0.95 + self.rng.gen_range(-0.05..0.05),
                        latency_ms: 48.0 + self.rng.gen_range(-5.0..10.0),
                    });
                }
                
                if self.rng.gen_bool(0.05) {
                    events.push(MlEvent {
                        event_id: format!("ml-{}-lane_departure-{}", self.truck_id, timestamp),
                        model_name: "lane_departure".to_string(),
                        timestamp,
                        result: MlResult::LaneDeparture {
                            is_departing: false,
                            deviation_pixels: 10 + self.rng.gen_range(-5..15),
                        },
                        confidence: 0.90 + self.rng.gen_range(-0.05..0.05),
                        latency_ms: 42.0 + self.rng.gen_range(-5.0..10.0),
                    });
                }
            }
            _ => {}
        }

        events
    }
}