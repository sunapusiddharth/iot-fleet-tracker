use crate::simulator::types::{OtaCommand, CommandType};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct OtaGenerator {
    truck_id: String,
    rng: rand::rngs::ThreadRng,
}

impl OtaGenerator {
    pub fn new(truck_id: &str) -> Self {
        Self {
            truck_id: truck_id.to_string(),
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_ota_command(&mut self, scenario: &str) -> Option<OtaCommand> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        
        // Generate commands based on scenario
        match scenario {
            "system_failure" => {
                // Send config update to reduce load
                if self.rng.gen_bool(0.3) {
                    return Some(OtaCommand {
                        command_id: format!("cmd-{}-config-{}", self.truck_id, timestamp),
                        truck_id: self.truck_id.clone(),
                        command_type: CommandType::UpdateConfig,
                        parameters: serde_json::json!({
                            "ml_edge": {
                                "enable_drowsiness": true,
                                "enable_lane_departure": true,
                                "enable_cargo_tamper": false, // Disable to reduce load
                            },
                            "camera": {
                                "fps": 10, // Reduce FPS
                            }
                        }),
                        issued_at: timestamp,
                    });
                }
            }
            "maintenance" => {
                // Send firmware update
                if self.rng.gen_bool(0.2) {
                    return Some(OtaCommand {
                        command_id: format!("cmd-{}-firmware-{}", self.truck_id, timestamp),
                        truck_id: self.truck_id.clone(),
                        command_type: CommandType::UpdateFirmware,
                        parameters: serde_json::json!({
                            "version": "2.1.0",
                            "url": "https://updates.example.com/firmware-2.1.0.bin",
                            "checksum": "sha256:deadbeef...",
                        }),
                        issued_at: timestamp,
                    });
                }
            }
            "normal_driving" => {
                // Occasionally send diagnostics request
                if self.rng.gen_bool(0.05) {
                    return Some(OtaCommand {
                        command_id: format!("cmd-{}-diagnostics-{}", self.truck_id, timestamp),
                        truck_id: self.truck_id.clone(),
                        command_type: CommandType::GetDiagnostics,
                        parameters: serde_json::json!({
                            "detail_level": "full",
                        }),
                        issued_at: timestamp,
                    });
                }
            }
            _ => {
                // Random occasional commands
                if self.rng.gen_bool(0.01) {
                    let commands = vec![
                        CommandType::UpdateConfig,
                        CommandType::Reboot,
                        CommandType::Shutdown,
                        CommandType::GetDiagnostics,
                    ];
                    let command_type = commands[self.rng.gen_range(0..commands.len())];
                    
                    return Some(OtaCommand {
                        command_id: format!("cmd-{}-{:?}-{}", self.truck_id, command_type, timestamp),
                        truck_id: self.truck_id.clone(),
                        command_type,
                        parameters: serde_json::json!({}),
                        issued_at: timestamp,
                    });
                }
            }
        }

        None
    }
}