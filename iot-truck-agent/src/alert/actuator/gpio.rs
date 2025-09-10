use crate::alert::types::{Alert, AlertAction, ActionType};
use crate::alert::error::Result;
use rppal::gpio::{Gpio, OutputPin};
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use tracing::{error, info};

pub struct GpioActuator {
    pin: Arc<Mutex<OutputPin>>,
    pin_number: u8,
    actuator_type: ActionType,
}

impl GpioActuator {
    pub fn new(pin_number: u8, actuator_type: ActionType) -> Result<Self> {
        let gpio = Gpio::new()
            .map_err(|e| AlertError::GpioError(format!("Failed to initialize GPIO: {}", e)))?;
        
        let pin = gpio.get(pin_number)
            .map_err(|e| AlertError::GpioError(format!("Failed to get GPIO pin {}: {}", pin_number, e)))?
            .into_output();

        Ok(Self {
            pin: Arc::new(Mutex::new(pin)),
            pin_number,
            actuator_type,
        })
    }
}

#[async_trait::async_trait]
impl crate::alert::actuator::Actuator for GpioActuator {
    async fn trigger(&self, alert: &Alert, action: &AlertAction) -> Result<()> {
        let duration_ms = action.parameters.get("duration_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(1000) as u64;

        let pattern = action.parameters.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("solid");

        info!(alert_id=%alert.alert_id, pin=%self.pin_number, duration_ms, pattern, "🔊 Triggering GPIO actuator");

        match pattern {
            "solid" => {
                {
                    let mut p = self.pin.lock().unwrap();
                    p.set_high();
                }
                sleep(Duration::from_millis(duration_ms)).await;
                {
                    let mut p = self.pin.lock().unwrap();
                    p.set_low();
                }
            }
            "blink" => {
                let blink_count = action.parameters.get("blink_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(3) as u64;
                
                let blink_interval_ms = action.parameters.get("blink_interval_ms")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(500) as u64;

                for _ in 0..blink_count {
                    {
                        let mut p = self.pin.lock().unwrap();
                        p.set_high();
                    }
                    sleep(Duration::from_millis(blink_interval_ms)).await;
                    {
                        let mut p = self.pin.lock().unwrap();
                        p.set_low();
                    }
                    if _ < blink_count - 1 {
                        sleep(Duration::from_millis(blink_interval_ms)).await;
                    }
                }
            }
            "pulse" => {
                let pulse_count = action.parameters.get("pulse_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(5) as u64;
                
                let pulse_interval_ms = action.parameters.get("pulse_interval_ms")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(100) as u64;

                for _ in 0..pulse_count {
                    {
                        let mut p = self.pin.lock().unwrap();
                        p.set_high();
                    }
                    sleep(Duration::from_millis(50)).await;
                    {
                        let mut p = self.pin.lock().unwrap();
                        p.set_low();
                    }
                    if _ < pulse_count - 1 {
                        sleep(Duration::from_millis(pulse_interval_ms)).await;
                    }
                }
            }
            _ => {
                return Err(AlertError::GpioError(format!("Unknown pattern: {}", pattern)));
            }
        }

        metrics::counter!("alert_gpio_triggers_total", "pin" => self.pin_number.to_string(), "type" => format!("{:?}", self.actuator_type)).increment(1);
        Ok(())
    }

    fn get_type(&self) -> ActionType {
        self.actuator_type.clone()
    }

    fn box_clone(&self) -> Box<dyn crate::alert::actuator::Actuator + Send + Sync> {
        Box::new(Self {
            pin: self.pin.clone(),
            pin_number: self.pin_number,
            actuator_type: self.actuator_type.clone(),
        })
    }
}