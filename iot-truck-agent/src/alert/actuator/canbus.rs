use crate::alert::types::{Alert, AlertAction, ActionType};
use crate::alert::error::Result;
use socketcan::{CanSocket, EmbeddedFrame};
use tracing::{error, info};

pub struct CanBusActuator {
    socket: CanSocket,
    interface: String,
}

impl CanBusActuator {
    pub fn new(interface: &str) -> Result<Self> {
        let socket = CanSocket::open(interface)
            .map_err(|e| AlertError::CanError(format!("Failed to open CAN interface {}: {}", interface, e)))?;

        Ok(Self {
            socket,
            interface: interface.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl crate::alert::actuator::Actuator for CanBusActuator {
    async fn trigger(&self, alert: &Alert, action: &AlertAction) -> Result<()> {
        let can_id = action.parameters.get("can_id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| AlertError::CanError("CAN ID not specified".to_string()))? as u32;

        let data = action.parameters.get("data")
            .and_then(|v| v.as_array())
            .ok_or_else(|| AlertError::CanError("CAN data not specified".to_string()))?
            .iter()
            .map(|v| v.as_u64().unwrap_or(0) as u8)
            .collect::<Vec<u8>>();

        if data.len() > 8 {
            return Err(AlertError::CanError("CAN data too long (max 8 bytes)".to_string()));
        }

        let frame = EmbeddedFrame::new(can_id, &data)
            .map_err(|e| AlertError::CanError(format!("Failed to create CAN frame: {}", e)))?;

        self.socket.write_frame(&frame)
            .map_err(|e| AlertError::CanError(format!("Failed to send CAN frame: {}", e)))?;

        info!(alert_id=%alert.alert_id, can_id=%can_id, data=%hex::encode(&data), "🚗 Sent CAN bus message");

        metrics::counter!("alert_canbus_triggers_total", "interface" => self.interface.clone()).increment(1);
        Ok(())
    }

    fn get_type(&self) -> ActionType {
        ActionType::SendCanMessage
    }

    fn box_clone(&self) -> Box<dyn crate::alert::actuator::Actuator + Send + Sync> {
        // CAN sockets can't be cloned, so we need to reopen
        Box::new(Self::new(&self.interface).unwrap())
    }
}