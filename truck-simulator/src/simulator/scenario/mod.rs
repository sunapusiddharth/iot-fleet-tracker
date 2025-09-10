use crate::simulator::types::TruckState;
use crate::simulator::truck::TruckSimulator;
use crate::config::{SimulatorConfig, ScenarioConfig};
use rand::Rng;
use std::collections::HashMap;

pub mod normal;
pub mod emergency;
pub mod failure;
pub mod custom;

pub struct ScenarioManager {
    scenarios: HashMap<String, Box<dyn Scenario>>,
    rng: rand::rngs::ThreadRng,
}

pub trait Scenario: Send + Sync {
    fn name(&self) -> &str;
    fn duration_minutes(&self) -> u32;
    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState;
    fn parameters(&self) -> &serde_json::Value;
}

impl ScenarioManager {
    pub fn new(config: &SimulatorConfig) -> Self {
        let mut scenarios: HashMap<String, Box<dyn Scenario>> = HashMap::new();
        let rng = rand::thread_rng();
        
        for scenario_config in &config.scenarios {
            let scenario: Box<dyn Scenario> = match scenario_config.name.as_str() {
                "normal_driving" => Box::new(normal::NormalDrivingScenario::new(scenario_config)),
                "emergency_braking" => Box::new(emergency::EmergencyBrakingScenario::new(scenario_config)),
                "rapid_acceleration" => Box::new(emergency::RapidAccelerationScenario::new(scenario_config)),
                "sharp_turn" => Box::new(emergency::SharpTurnScenario::new(scenario_config)),
                "system_failure" => Box::new(failure::SystemFailureScenario::new(scenario_config)),
                "maintenance" => Box::new(failure::MaintenanceScenario::new(scenario_config)),
                "drowsy_driver" => Box::new(custom::DrowsyDriverScenario::new(scenario_config)),
                "lane_departure" => Box::new(custom::LaneDepartureScenario::new(scenario_config)),
                "cargo_tamper" => Box::new(custom::CargoTamperScenario::new(scenario_config)),
                _ => Box::new(normal::NormalDrivingScenario::new(scenario_config)),
            };
            scenarios.insert(scenario_config.name.clone(), scenario);
        }
        
        Self {
            scenarios,
            rng,
        }
    }
    
    pub fn select_scenario(&self) -> Option<&Box<dyn Scenario>> {
        if self.scenarios.is_empty() {
            return None;
        }
        
        // Weighted random selection based on probability
        let total_probability: f32 = self.scenarios.values().map(|s| {
            let config = self.get_scenario_config(s.name());
            config.map(|c| c.probability).unwrap_or(0.0)
        }).sum();
        
        if total_probability <= 0.0 {
            return Some(self.scenarios.values().next().unwrap());
        }
        
        let rand_val = self.rng.gen_range(0.0..total_probability);
        let mut cumulative = 0.0;
        
        for scenario in self.scenarios.values() {
            let config = self.get_scenario_config(scenario.name());
            let prob = config.map(|c| c.probability).unwrap_or(0.0);
            cumulative += prob;
            if rand_val <= cumulative {
                return Some(scenario);
            }
        }
        
        Some(self.scenarios.values().next().unwrap())
    }
    
    pub fn get_scenario(&self, name: &str) -> Option<&Box<dyn Scenario>> {
        self.scenarios.get(name)
    }
    
    fn get_scenario_config(&self, name: &str) -> Option<&ScenarioConfig> {
        // This would need to be implemented based on how you store configs
        None
    }
}

// Implement for each scenario type
pub struct NormalDrivingScenario {
    config: ScenarioConfig,
}

impl NormalDrivingScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl Scenario for NormalDrivingScenario {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }
    
    fn update_truck(&self, truck: &mut TruckSimulator, _elapsed_minutes: f32) -> TruckState {
        truck.update()
    }
    
    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}

pub struct EmergencyBrakingScenario {
    config: ScenarioConfig,
}

impl EmergencyBrakingScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl Scenario for EmergencyBrakingScenario {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }
    
    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
        // Set scenario for truck
        if elapsed_minutes < 0.1 { // First 6 seconds
            truck.set_scenario("emergency_braking", self.duration_minutes());
        }
        truck.update()
    }
    
    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}

pub struct RapidAccelerationScenario {
    config: ScenarioConfig,
}

impl RapidAccelerationScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl Scenario for RapidAccelerationScenario {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }
    
    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
        if elapsed_minutes < 0.1 {
            truck.set_scenario("rapid_acceleration", self.duration_minutes());
        }
        truck.update()
    }
    
    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}

pub struct SharpTurnScenario {
    config: ScenarioConfig,
}

impl SharpTurnScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl Scenario for SharpTurnScenario {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }
    
    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
        if elapsed_minutes < 0.1 {
            truck.set_scenario("sharp_turn", self.duration_minutes());
        }
        truck.update()
    }
    
    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}

pub struct SystemFailureScenario {
    config: ScenarioConfig,
}

impl SystemFailureScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl Scenario for SystemFailureScenario {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }
    
    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
        if elapsed_minutes < 0.1 {
            truck.set_scenario("system_failure", self.duration_minutes());
        }
        truck.update()
    }
    
    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}

pub struct MaintenanceScenario {
    config: ScenarioConfig,
}

impl MaintenanceScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl Scenario for MaintenanceScenario {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }
    
    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
        if elapsed_minutes < 0.1 {
            truck.set_scenario("maintenance", self.duration_minutes());
        }
        truck.update()
    }
    
    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}