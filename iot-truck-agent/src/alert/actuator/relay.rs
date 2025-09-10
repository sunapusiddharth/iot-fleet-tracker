use crate::alert::types::{Alert, AlertAction, ActionType};
use crate::alert::error::Result;
use rppal::gpio::{Gpio, OutputPin};
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use tracing::{error, info};

pub struct RelayActuator {
    pin: Arc<Mutex<OutputPin>>,
    pin_number: u8,
    normally_open: bool,
}

impl RelayActuator {
    pub fn new(pin_number: u8, normally_open: bool) -> Result<Self> {
        let gpio = Gpio::new()
            .map_err(|e| AlertError::GpioError(format!("Failed to initialize GPIO: {}", e)))?;
        
        let pin = gpio.get(pin_number)
            .map_err(|e| AlertError::GpioError(format!("Failed to get GPIO pin {}: {}", pin_number, e)))?
            .into_output();

        // Set initial state based on normally_open
        {
            let mut p = pin.lock().unwrap();
            if normally_open {
                p.set_low(); // Off state for normally open relay
            } else {
                p.set_high(); // Off state for normally closed relay
            }
        }

        Ok(Self {
            pin,
            pin_number,
            normally_open,
        })
    }
}

#[async_trait::async_trait]
impl crate::alert::actuator::Actuator for RelayActuator {
    async fn trigger(&self, alert: &Alert, action: &AlertAction) -> Result<()> {
        let duration_ms = action.parameters.get("duration_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(1000) as u64;

        let activate = action.parameters.get("activate")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        info!(alert_id=%alert.alert_id, pin=%self.pin_number, duration_ms, activate, "🔌 Triggering relay");

        {
            let mut p = self.pin.lock().unwrap();
            if self.normally_open {
                if activate {
                    p.set_high(); // Activate normally open relay
                } else {
                    p.set_low(); // Deactivate
                }
            } else {
                if activate {
                    p.set_low(); // Activate normally closed relay
                } else {
                    p.set_high(); // Deactivate
                }
            }
        }

        if duration_ms > 0 {
            sleep(Duration::from_millis(duration_ms)).await;
            
            // Return to default state
            {
                let mut p = self.pin.lock().unwrap();
                if self.normally_open {
                    p.set_low();
                } else {
                    p.set_high();
                }
            }
        }

        metrics::counter!("alert_relay_triggers_total", "pin" => self.pin_number.to_string()).increment(1);
        Ok(())
    }

    fn get_type(&self) -> ActionType {
        ActionType::ActivateRelay
    }

    fn box_clone(&self) -> Box<dyn crate::alert::actuator::Actuator + Send + Sync> {
        Box::new(Self {
            pin: self.pin.clone(),
            pin_number: self.pin_number,
            normally_open: self.normally_open,
        })
    }
}