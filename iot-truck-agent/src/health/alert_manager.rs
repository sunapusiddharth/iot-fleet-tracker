use crate::health::config::HealthConfig;
use crate::health::types::{AlertInfo, AlertSeverity, HealthEvent};
use rppal::gpio::{Gpio, OutputPin};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{error, info, warn};

pub struct AlertManager {
    config: HealthConfig,
    pin: Option<Arc<Mutex<OutputPin>>>,
    last_alerts: HashMap<String, Instant>,
    alert_history: Vec<AlertInfo>,
    max_history: usize,
}

impl AlertManager {
    pub fn new(config: HealthConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let pin = if config.enable_alerts {
            match Gpio::new() {
                Ok(gpio) => match gpio.get(config.alert_pin) {
                    Ok(pin) => {
                        let output = pin.into_output();
                        Some(Arc::new(Mutex::new(output)))
                    }
                    Err(e) => {
                        error!(error=%e, "Failed to get GPIO pin {}", config.alert_pin);
                        None
                    }
                },
                Err(e) => {
                    error!(error=%e, "Failed to initialize GPIO");
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            config,
            pin,
            last_alerts: HashMap::new(),
            alert_history: Vec::new(),
            max_history: 1000,
        })
    }

    pub fn process_alerts(&mut self, alerts: Vec<AlertInfo>) -> Vec<AlertInfo> {
        let mut processed_alerts = Vec::new();

        for alert in alerts {
            // Debounce
            if let Some(last) = self.last_alerts.get(&alert.alert_type) {
                if last.elapsed() < Duration::from_millis(self.config.debounce_ms) {
                    continue; // Skip this alert
                }
            }

            // Record
            self.last_alerts
                .insert(alert.alert_type.clone(), Instant::now());
            self.alert_history.push(alert.clone());
            if self.alert_history.len() > self.max_history {
                self.alert_history.remove(0);
            }

            // Trigger local alert
            if let Err(e) = self.trigger_local_alert(&alert) {
                error!(error=%e, "Failed to trigger local alert");
            }

            processed_alerts.push(alert);
        }

        processed_alerts
    }

    fn trigger_local_alert(&self, alert: &AlertInfo) -> Result<(), Box<dyn std::error::Error>> {
        if alert.severity == AlertSeverity::Info {
            return Ok(()); // No local alert for info
        }

        if let Some(ref pin) = self.pin {
            match alert.severity {
                AlertSeverity::Warning => {
                    // Blink slowly (500ms on, 500ms off) x2
                    self.blink(pin.clone(), 500, 2)?;
                }
                AlertSeverity::Critical => {
                    // Blink rapidly (100ms on, 100ms off) x5
                    self.blink(pin.clone(), 100, 5)?;
                }
                _ => {}
            }
            info!(alert_type=%alert.alert_type, severity=%alert.severity, "🚨 Local alert triggered: {}", alert.message);
        } else {
            // Log if no GPIO
            info!(alert_type=%alert.alert_type, severity=%alert.severity, "🔔 Local alert: {}", alert.message);
        }

        Ok(())
    }

    fn blink(
        &self,
        pin: Arc<Mutex<OutputPin>>,
        interval_ms: u64,
        times: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn get_alert_history(&self) -> Vec<AlertInfo> {
        self.alert_history.clone()
    }

    pub fn clear_alert_history(&mut self) {
        self.alert_history.clear();
    }
}
