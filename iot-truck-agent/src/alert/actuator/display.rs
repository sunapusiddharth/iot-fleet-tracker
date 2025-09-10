use crate::alert::types::{Alert, AlertAction, ActionType};
use crate::alert::error::Result;
use ssd1306::{prelude::*, I2CDisplayInterface, DisplaySize128x64};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use std::thread;
use std::time::Duration;
use tracing::{error, info};

pub struct DisplayActuator {
    interface: I2CDisplayInterface,
    size: DisplaySize128x64,
}

impl DisplayActuator {
    pub fn new(i2c_bus: u8, address: u16) -> Result<Self> {
        // This is simplified - in production, use proper I2C setup
        let interface = I2CDisplayInterface::new(/* i2c */);
        let size = DisplaySize128x64;

        Ok(Self {
            interface,
            size,
        })
    }
}

#[async_trait::async_trait]
impl crate::alert::actuator::Actuator for DisplayActuator {
    async fn trigger(&self, alert: &Alert, action: &AlertAction) -> Result<()> {
        let message = action.parameters.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or(&alert.message);

        let duration_ms = action.parameters.get("duration_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(5000) as u64;

        info!(alert_id=%alert.alert_id, message=%message, duration_ms, "🖥️  Showing alert on display");

        // In production, initialize and control the display
        // This is a placeholder for the actual display code

        // Simulate display update
        thread::sleep(Duration::from_millis(duration_ms as u64));

        metrics::counter!("alert_display_triggers_total").increment(1);
        Ok(())
    }

    fn get_type(&self) -> ActionType {
        ActionType::ShowOnDisplay
    }

    fn box_clone(&self) -> Box<dyn crate::alert::actuator::Actuator + Send + Sync> {
        // Display can't be easily cloned, so create new
        Box::new(Self {
            interface: self.interface.clone(), // This won't work - need proper clone
            size: self.size,
        })
    }
}