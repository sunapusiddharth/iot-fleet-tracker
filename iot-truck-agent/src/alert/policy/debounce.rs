use crate::alert::types::Alert;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use tracing::{info, warn};

pub struct AlertDebouncer {
    last_alerts: HashMap<String, Instant>,
    cooldown_periods: HashMap<String, Duration>,
}

impl AlertDebouncer {
    pub fn new() -> Self {
        let mut cooldown_periods = HashMap::new();
        cooldown_periods.insert("DrowsyDriver".to_string(), Duration::from_secs(30));
        cooldown_periods.insert("LaneDeparture".to_string(), Duration::from_secs(10));
        cooldown_periods.insert("HarshBraking".to_string(), Duration::from_secs(5));
        cooldown_periods.insert("HighTemperature".to_string(), Duration::from_secs(60));

        Self {
            last_alerts: HashMap::new(),
            cooldown_periods,
        }
    }

    pub fn should_suppress(&mut self, alert: &Alert) -> bool {
        let alert_key = format!("{:?}", alert.alert_type);
        let now = Instant::now();

        if let Some(last) = self.last_alerts.get(&alert_key) {
            let cooldown = self.cooldown_periods.get(&alert_key)
                .copied()
                .unwrap_or(Duration::from_secs(5));

            if now.duration_since(*last) < cooldown {
                warn!(alert_type=%alert_key, "⏳ Alert suppressed due to cooldown");
                metrics::counter!("alert_suppressed_total", "type" => alert_key).increment(1);
                return true;
            }
        }

        self.last_alerts.insert(alert_key, now);
        false
    }

    pub fn reset_cooldown(&mut self, alert_type: &str) {
        self.last_alerts.remove(alert_type);
        info!(alert_type=%alert_type, "🔄 Cooldown reset");
    }
}