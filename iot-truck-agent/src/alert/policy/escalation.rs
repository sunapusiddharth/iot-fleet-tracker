use crate::alert::types::{Alert, AlertSeverity, AlertAction, ActionType};
use std::collections::VecDeque;
use tracing::{info, warn};

pub struct AlertEscalator {
    escalation_rules: Vec<EscalationRule>,
}

pub struct EscalationRule {
    pub alert_type: String,
    pub min_severity: AlertSeverity,
    pub actions: Vec<AlertActionTemplate>,
    pub repeat_interval: Option<u64>, // seconds
}

pub struct AlertActionTemplate {
    pub action_type: ActionType,
    pub target: String,
    pub parameters: serde_json::Value,
}

impl AlertEscalator {
    pub fn new() -> Self {
        let mut rules = Vec::new();

        // Drowsy driver escalation
        rules.push(EscalationRule {
            alert_type: "DrowsyDriver".to_string(),
            min_severity: AlertSeverity::Critical,
            actions: vec![
                AlertActionTemplate {
                    action_type: ActionType::TriggerBuzzer,
                    target: "buzzer_1".to_string(),
                    parameters: serde_json::json!({"duration_ms": 1000, "pattern": "pulse", "pulse_count": 5}),
                },
                AlertActionTemplate {
                    action_type: ActionType::FlashLed,
                    target: "led_red".to_string(),
                    parameters: serde_json::json!({"duration_ms": 5000, "pattern": "blink", "blink_count": 10}),
                },
                AlertActionTemplate {
                    action_type: ActionType::ShowOnDisplay,
                    target: "display_1".to_string(),
                    parameters: serde_json::json!({"message": "DROWSY DRIVER DETECTED", "duration_ms": 10000}),
                },
            ],
            repeat_interval: Some(30), // Repeat every 30 seconds if not acknowledged
        });

        // High temperature escalation
        rules.push(EscalationRule {
            alert_type: "HighTemperature".to_string(),
            min_severity: AlertSeverity::Critical,
            actions: vec![
                AlertActionTemplate {
                    action_type: ActionType::TriggerBuzzer,
                    target: "buzzer_1".to_string(),
                    parameters: serde_json::json!({"duration_ms": 500, "pattern": "solid"}),
                },
                AlertActionTemplate {
                    action_type: ActionType::ActivateRelay,
                    target: "relay_1".to_string(),
                    parameters: serde_json::json!({"activate": true, "duration_ms": 0}), // Stay on
                },
            ],
            repeat_interval: Some(60),
        });

        Self {
            escalation_rules,
        }
    }

    pub fn get_actions_for_alert(&self, alert: &Alert) -> Vec<AlertAction> {
        let mut actions = Vec::new();

        for rule in &self.escalation_rules {
            if format!("{:?}", alert.alert_type) == rule.alert_type && alert.severity >= rule.min_severity {
                for template in &rule.actions {
                    let action = AlertAction {
                        action_id: format!("action-{}-{}", template.action_type, chrono::Utc::now().timestamp_nanos()),
                        action_type: template.action_type.clone(),
                        target: template.target.clone(),
                        parameters: template.parameters.clone(),
                        executed_at: None,
                        success: false,
                        error: None,
                    };
                    actions.push(action);
                }
            }
        }

        actions
    }

    pub fn should_repeat_alert(&self, alert: &Alert, last_triggered: u64) -> bool {
        for rule in &self.escalation_rules {
            if format!("{:?}", alert.alert_type) == rule.alert_type {
                if let Some(interval) = rule.repeat_interval {
                    let elapsed = chrono::Utc::now().timestamp() as u64 - last_triggered;
                    return elapsed >= interval;
                }
            }
        }
        false
    }
}