use crate::health::types::{AlertInfo, AlertSeverity};
use rppal::gpio::{Gpio, OutputPin};
use std::sync::{Arc, Mutex};
use tracing::{error, info, warn};

pub struct LocalAlerter {
    pin: Option<Arc<Mutex<OutputPin>>>,
    last_alert: std::time::Instant,
    debounce_ms: u64,
}

impl LocalAlerter {
    pub fn new(alert_pin: u8, debounce_ms: u64) -> Result<Self, HealthError> {
        let pin = match Gpio::new() {
            Ok(gpio) => match gpio.get(alert_pin) {
                Ok(pin) => {
                    let output = pin.into_output();
                    Some(Arc::new(Mutex::new(output)))
                }
                Err(e) => {
                    error!(error=%e, "Failed to get GPIO pin {}", alert_pin);
                    None
                }
            },
            Err(e) => {
                error!(error=%e, "Failed to initialize GPIO");
                None
            }
        };

        Ok(Self {
            pin,
            last_alert: std::time::Instant::now(),
            debounce_ms,
        })
    }

    pub fn trigger_alert(&mut self, alert: &AlertInfo) -> Result<(), HealthError> {
        // Debounce
        let elapsed = self.last_alert.elapsed().as_millis();
        if elapsed < self.debounce_ms {
            return Ok(());
        }
        self.last_alert = std::time::Instant::now();

        // Only trigger for Warning and Critical
        if alert.severity == AlertSeverity::Info {
            return Ok(());
        }

        if let Some(ref pin) = self.pin {
            match alert.severity {
                AlertSeverity::Warning => {
                    // Blink slowly
                    self.blink(pin.clone(), 500, 2)?;
                }
                AlertSeverity::Critical => {
                    // Blink rapidly
                    self.blink(pin.clone(), 100, 5)?;
                }
                _ => {}
            }
            info!(alert_type=%alert.alert_type, severity=%alert.severity, "🚨 Local alert triggered");
        } else {
            // Log if no GPIO
            info!(alert_type=%alert.alert_type, severity=%alert.severity, "🔔 Local alert (no GPIO): {}", alert.message);
        }

        Ok(())
    }

    fn blink(
        &self,
        pin: Arc<Mutex<OutputPin>>,
        interval_ms: u64,
        times: u32,
    ) -> Result<(), HealthError> {
        for _ in 0..times {
            {
                let mut p = pin.lock().unwrap();
                p.set_high();
            }
            std::thread::sleep(std::time::Duration::from_millis(interval_ms));
            {
                let mut p = pin.lock().unwrap();
                p.set_low();
            }
            std::thread::sleep(std::time::Duration::from_millis(interval_ms));
        }
        Ok(())
    }
}
