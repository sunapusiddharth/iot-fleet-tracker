use crate::config::SimulatorConfig;
use crate::simulator::truck::TruckSimulator;
use crate::simulator::scenario::ScenarioManager;
use crate::simulator::protocol::ProtocolHandler;
use tokio::sync::broadcast;
use tracing::{info, error};

pub mod types;
pub mod error;
pub mod truck;
pub mod scenario;
pub mod protocol;

// Metrics
metrics::describe_gauge!("simulator_trucks_active", "Number of active trucks");
metrics::describe_counter!("simulator_events_generated", "Total events generated");
metrics::describe_gauge!("simulator_scenario_progress", "Current scenario progress 0-1");

pub struct Simulator {
    config: SimulatorConfig,
    trucks: Vec<TruckSimulator>,
    scenario_manager: ScenarioManager,
    protocol_handler: ProtocolHandler,
    tx: broadcast::Sender<crate::simulator::types::TruckState>,
}

impl Simulator {
    pub async fn new(config: SimulatorConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut trucks = Vec::new();
        
        // Create trucks based on config
        for truck_config in &config.trucks {
            let truck = TruckSimulator::new(truck_config.clone(), config.simulation.update_interval_ms);
            trucks.push(truck);
        }
        
        // If no trucks specified, create default trucks
        if trucks.is_empty() {
            for i in 0..config.simulation.num_trucks {
                let truck_config = crate::config::TruckConfig {
                    id: format!("TRK-{:04}", i + 1),
                    model: "Model-X".to_string(),
                    initial_latitude: config.simulation.base_latitude + (i as f64) * 0.001,
                    initial_longitude: config.simulation.base_longitude + (i as f64) * 0.001,
                    initial_speed: 60.0,
                    sensor_config: crate::config::SensorConfig {
                        gps_update_rate_hz: 10,
                        obd_update_rate_hz: 10,
                        imu_update_rate_hz: 100,
                        tpms_update_rate_hz: 1,
                    },
                    camera_config: crate::config::CameraConfig {
                        front_camera: true,
                        driver_camera: true,
                        cargo_camera: true,
                        resolution: "1280x720".to_string(),
                        fps: 15,
                    },
                    ml_config: crate::config::MlConfig {
                        drowsiness_detection: true,
                        lane_departure_detection: true,
                        cargo_tamper_detection: true,
                        license_plate_detection: false,
                    },
                };
                
                let truck = TruckSimulator::new(truck_config, config.simulation.update_interval_ms);
                trucks.push(truck);
            }
        }
        
        let scenario_manager = ScenarioManager::new(&config);
        let protocol_handler = ProtocolHandler::new(&config.server).await?;
        
        let (tx, _) = broadcast::channel(1000);
        
        info!("✅ Simulator initialized with {} trucks", trucks.len());
        
        Ok(Self {
            config,
            trucks,
            scenario_manager,
            protocol_handler,
            tx,
        })
    }
    
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Starting simulator");
        
        // Start protocol handler
        let tx_clone = self.tx.clone();
        tokio::spawn(async move {
            if let Err(e) = self.protocol_handler.start(tx_clone).await {
                error!("Protocol handler failed: {}", e);
            }
        });
        
        // Start simulation loop
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(self.config.simulation.update_interval_ms));
        
        loop {
            interval.tick().await;
            
            // Select scenario for each truck
            for truck in &mut self.trucks {
                if let Some(scenario) = self.scenario_manager.select_scenario() {
                    // Apply scenario to truck
                    let state = scenario.update_truck(truck, 0.0);
                    
                    // Send state to protocols
                    if let Err(e) = self.tx.send(state) {
                        error!("Failed to send truck state: {}", e);
                    }
                    
                    metrics::gauge!("simulator_trucks_active").set(self.trucks.len() as f64);
                    metrics::counter!("simulator_events_generated").increment(1);
                }
            }
        }
    }
}