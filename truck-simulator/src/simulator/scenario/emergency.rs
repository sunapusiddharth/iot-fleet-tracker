use crate::simulator::types::TruckState;
use crate::simulator::truck::TruckSimulator;
use crate::config::ScenarioConfig;

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

impl super::Scenario for EmergencyBrakingScenario {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }
    
    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
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

impl super::Scenario for RapidAccelerationScenario {
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

impl super::Scenario for SharpTurnScenario {
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