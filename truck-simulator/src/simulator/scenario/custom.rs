use crate::config::ScenarioConfig;
use crate::simulator::truck::TruckSimulator;
use crate::simulator::types::TruckState;

pub struct DrowsyDriverScenario {
    config: ScenarioConfig,
}

impl DrowsyDriverScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl super::Scenario for DrowsyDriverScenario {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }

    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
        if elapsed_minutes < 0.1 {
            truck.set_scenario("drowsy_driver", self.duration_minutes());
        }
        truck.update()
    }

    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}

pub struct LaneDepartureScenario {
    config: ScenarioConfig,
}

impl LaneDepartureScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl super::Scenario for LaneDepartureScenario {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }

    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
        if elapsed_minutes < 0.1 {
            truck.set_scenario("lane_departure", self.duration_minutes());
        }
        truck.update()
    }

    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}

pub struct CargoTamperScenario {
    config: ScenarioConfig,
}

impl CargoTamperScenario {
    pub fn new(config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl super::Scenario for CargoTamperScenario {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn duration_minutes(&self) -> u32 {
        self.config.duration_minutes
    }

    fn update_truck(&self, truck: &mut TruckSimulator, elapsed_minutes: f32) -> TruckState {
        if elapsed_minutes < 0.1 {
            truck.set_scenario("cargo_tamper", self.duration_minutes());
        }
        truck.update()
    }

    fn parameters(&self) -> &serde_json::Value {
        &self.config.parameters
    }
}
