use crate::simulator::types::TruckState;
use crate::config::TruckConfig;
use geo::{Point, Coord};
use rand::Rng;
use std::time::Duration;

pub mod sensor;
pub mod camera;
pub mod ml;
pub mod health;
pub mod ota;

pub struct TruckSimulator {
    config: TruckConfig,
    sensor_generator: sensor::SensorGenerator,
    camera_generator: camera::CameraGenerator,
    ml_generator: ml::MlGenerator,
    health_generator: health::HealthGenerator,
    ota_generator: ota::OtaGenerator,
    current_location: Point<f64>,
    current_speed: f32,
    current_heading: f32,
    current_scenario: String,
    scenario_duration: Duration,
    scenario_start: std::time::Instant,
    update_interval: Duration,
    last_update: std::time::Instant,
}

impl TruckSimulator {
    pub fn new(config: TruckConfig, update_interval_ms: u64) -> Self {
        let current_location = Point::from(Coord {
            x: config.initial_longitude,
            y: config.initial_latitude,
        });
        
        Self {
            config,
            sensor_generator: sensor::SensorGenerator::new(&config.id),
            camera_generator: camera::CameraGenerator::new(&config.id),
            ml_generator: ml::MlGenerator::new(&config.id),
            health_generator: health::HealthGenerator::new(&config.id),
            ota_generator: ota::OtaGenerator::new(&config.id),
            current_location,
            current_speed: config.initial_speed,
            current_heading: 0.0,
            current_scenario: "normal_driving".to_string(),
            scenario_duration: Duration::from_secs(10 * 60), // 10 minutes default
            scenario_start: std::time::Instant::now(),
            update_interval: Duration::from_millis(update_interval_ms),
            last_update: std::time::Instant::now(),
        }
    }

    pub fn set_scenario(&mut self, scenario: &str, duration_minutes: u32) {
        self.current_scenario = scenario.to_string();
        self.scenario_duration = Duration::from_secs(duration_minutes as u64 * 60);
        self.scenario_start = std::time::Instant::now();
        self.health_generator.set_scenario(scenario, duration_minutes);
    }

    pub fn update(&mut self) -> TruckState {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_update);
        
        if elapsed < self.update_interval {
            // Return current state without updating
            return self.get_current_state();
        }
        
        self.last_update = now;
        
        // Update position based on speed and heading
        self.update_position();
        
        // Generate sensor data
        let sensors = self.sensor_generator.generate_sensor_data(
            self.current_location.y(),
            self.current_location.x(),
            self.current_speed,
            &self.current_scenario,
        );
        
        // Generate camera data
        let front_camera = if self.config.camera_config.front_camera {
            Some(self.camera_generator.generate_front_camera_frame(&self.current_scenario))
        } else {
            None
        };
        
        let driver_camera = if self.config.camera_config.driver_camera {
            Some(self.camera_generator.generate_driver_camera_frame(&self.current_scenario))
        } else {
            None
        };
        
        let cargo_camera = if self.config.camera_config.cargo_camera {
            Some(self.camera_generator.generate_cargo_camera_frame(&self.current_scenario))
        } else {
            None
        };
        
        let cameras = crate::simulator::types::CameraData {
            front_camera,
            driver_camera,
            cargo_camera,
        };
        
        // Generate ML events
        let ml_events = self.ml_generator.generate_ml_events(&self.current_scenario);
        
        // Generate health status
        let health_status = self.health_generator.generate_health_status(&self.current_scenario);
        
        // Generate OTA command (occasionally)
        if let Some(ota_command) = self.ota_generator.generate_ota_command(&self.current_scenario) {
            // In production, send this to the truck
            tracing::info!(truck_id=%self.config.id, command_id=%ota_command.command_id, "📤 Generated OTA command");
        }
        
        TruckState {
            truck_id: self.config.id.clone(),
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64,
            location: (self.current_location.y(), self.current_location.x()),
            speed_kmh: self.current_speed,
            heading: self.current_heading,
            scenario: self.current_scenario.clone(),
            sensors,
            cameras,
            ml_events,
            health_status,
        }
    }
    
    fn update_position(&mut self) {
        // Simple position update based on speed and heading
        let speed_mps = self.current_speed * 1000.0 / 3600.0; // km/h to m/s
        let time_sec = self.update_interval.as_secs_f32();
        let distance_m = speed_mps * time_sec;
        
        // Convert heading to radians
        let heading_rad = self.current_heading.to_radians();
        
        // Calculate new position (simplified - doesn't account for Earth curvature)
        let lat_change = (distance_m * heading_rad.cos()) / 111320.0; // meters to degrees latitude
        let lon_change = (distance_m * heading_rad.sin()) / (111320.0 * self.current_location.y().to_radians().cos()); // meters to degrees longitude
        
        let new_lat = self.current_location.y() + lat_change;
        let new_lon = self.current_location.x() + lon_change;
        
        self.current_location = Point::from(Coord {
            x: new_lon,
            y: new_lat,
        });
        
        // Occasionally change heading
        if rand::thread_rng().gen_bool(0.1) {
            self.current_heading += rand::thread_rng().gen_range(-30.0..30.0);
            self.current_heading = ((self.current_heading % 360.0) + 360.0) % 360.0;
        }
        
        // Modify speed based on scenario
        match self.current_scenario.as_str() {
            "emergency_braking" => {
                self.current_speed = (self.current_speed - 5.0).max(0.0);
            }
            "rapid_acceleration" => {
                self.current_speed = (self.current_speed + 3.0).min(120.0);
            }
            "sharp_turn" => {
                self.current_speed = (self.current_speed - 2.0).max(20.0);
            }
            _ => {
                // Normal driving - maintain speed with small variations
                if rand::thread_rng().gen_bool(0.2) {
                    self.current_speed += rand::thread_rng().gen_range(-2.0..2.0);
                    self.current_speed = self.current_speed.max(0.0).min(120.0);
                }
            }
        }
    }
    
    fn get_current_state(&self) -> TruckState {
        // Return current state without updating
        TruckState {
            truck_id: self.config.id.clone(),
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64,
            location: (self.current_location.y(), self.current_location.x()),
            speed_kmh: self.current_speed,
            heading: self.current_heading,
            scenario: self.current_scenario.clone(),
            sensors: self.sensor_generator.generate_sensor_data(
                self.current_location.y(),
                self.current_location.x(),
                self.current_speed,
                &self.current_scenario,
            ),
            cameras: crate::simulator::types::CameraData {
                front_camera: None,
                driver_camera: None,
                cargo_camera: None,
            },
            ml_events: Vec::new(),
            health_status: self.health_generator.generate_health_status(&self.current_scenario),
        }
    }
}