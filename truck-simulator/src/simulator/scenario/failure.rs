use crate::simulator::types::TruckState;
use crate::simulator::truck::TruckSimulator;
use crate::config::ScenarioConfig;

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

impl super::Scenario for SystemFailureScenario {
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

impl super::Scenario for MaintenanceScenario {
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