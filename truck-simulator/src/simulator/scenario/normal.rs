use crate::simulator::types::TruckState;
use crate::simulator::truck::TruckSimulator;
use crate::config::ScenarioConfig;

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

impl super::Scenario for NormalDrivingScenario {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }
    
    fn update_truck(&self, truck: &mut TruckSimulator, _elapsed_minutes: f32) -> TruckState {
        truck.set_scenario("normal_driving", self.duration_minutes());
        truck.update()
    }
    
    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}