use crate::alert::types::{Alert, AlertType, AlertSeverity};
use crate::ml_edge::types::MLEvent;
use tracing::{info, warn};

pub struct MlTriggerEngine;

impl MlTriggerEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn trigger_from_ml(&self, ml_event: &MLEvent) -> Option<Alert> {
        match &ml_event.result {
            crate::ml_edge::types::InferenceResult::Drowsiness(d) => {
                if d.is_drowsy && ml_event.calibrated_confidence > 0.8 {
                    Some(Alert::new(
                        AlertType::DrowsyDriver,
                        AlertSeverity::Emergency,
                        "Driver is drowsy - immediate attention required",
                        &ml_event.meta.device_id,
                    ))
                } else {
                    None
                }
            }
            crate::ml_edge::types::InferenceResult::LaneDeparture(l) => {
                if l.is_departing && l.deviation_pixels > 50 && ml_event.calibrated_confidence > 0.7 {
                    Some(Alert::new(
                        AlertType::LaneDeparture,
                        AlertSeverity::Critical,
                        "Lane departure detected - correct steering",
                        &ml_event.meta.device_id,
                    ))
                } else {
                    None
                }
            }
            crate::ml_edge::types::InferenceResult::CargoTamper(c) => {
                if c.is_tampered && ml_event.calibrated_confidence > 0.8 {
                    Some(Alert::new(
                        AlertType::CargoTamper,
                        AlertSeverity::Critical,
                        "Cargo tampering detected - check cargo area",
                        &ml_event.meta.device_id,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}