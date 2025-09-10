use crate::config::Config;
use crate::stream::types::StreamEvent;
use crate::health::types::{HealthEvent, HealthStatus};
use crate::health::system_monitor::SystemMonitor;
use crate::health::network_monitor::NetworkMonitor;
use crate::health::task_supervisor::TaskSupervisor;
use crate::health::thermal_manager::ThermalManager;
use crate::health::disk_pressure::DiskPressureManager;
use crate::health::adaptive_controller::AdaptiveController;
use crate::health::alert_manager::AlertManager;
use crate::health::snapshot::HealthSnapshotter;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tracing::{error, info, warn};

pub mod types;
pub mod error;
pub mod config;
pub mod system_monitor;
pub mod network_monitor;
pub mod task_supervisor;
pub mod thermal_manager;
pub mod disk_pressure;
pub mod adaptive_controller;
pub mod alert_manager;
pub mod snapshot;

// Metrics
metrics::describe_gauge!("health_cpu_percent", "CPU usage percent");
metrics::describe_gauge!("health_memory_percent", "Memory usage percent");
metrics::describe_gauge!("health_disk_percent", "Disk usage percent");
metrics::describe_gauge!("health_temperature_c", "System temperature in Celsius");
metrics::describe_gauge!("health_network_latency_ms", "Network latency in ms");
metrics::describe_counter!("health_alerts_total", "Total health alerts");
metrics::describe_gauge!("health_status", "Health status (0=Ok, 1=Warning, 2=Critical, 3=Degraded)");
metrics::describe_counter!("health_actions_taken_total", "Total adaptive actions taken");

pub struct HealthManager {
    system_monitor: SystemMonitor,
    network_monitor: NetworkMonitor,
    task_supervisor: TaskSupervisor,
    thermal_manager: ThermalManager,
    disk_pressure_manager: DiskPressureManager,
    adaptive_controller: AdaptiveController,
    alert_manager: AlertManager,
    snapshotter: HealthSnapshotter,
    tx: broadcast::Sender<StreamEvent>,
    config: Config,
    mqtt_client: Option<Arc<rumqttc::AsyncClient>>,
}

impl HealthManager {
    pub async fn new(
        config: Config,
        tx: broadcast::Sender<StreamEvent>,
        mqtt_client: Option<Arc<rumqttc::AsyncClient>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let health_config = config.health.clone();

        let system_monitor = SystemMonitor::new(health_config.clone());
        let network_monitor = NetworkMonitor::new(health_config.clone(), mqtt_client.clone());
        let task_supervisor = TaskSupervisor::new();
        let thermal_manager = ThermalManager::new(health_config.clone());
        let disk_pressure_manager = DiskPressureManager::new(health_config.clone());
        let adaptive_controller = AdaptiveController::new(health_config.clone());
        let alert_manager = AlertManager::new(health_config.clone())?;
        let snapshotter = HealthSnapshotter::new(&format!("{}/health_snapshots", config.storage.wal_path))?;

        info!("✅ Health Manager initialized with full adaptive control");

        Ok(Self {
            system_monitor,
            network_monitor,
            task_supervisor,
            thermal_manager,
            disk_pressure_manager,
            adaptive_controller,
            alert_manager,
            snapshotter,
            tx,
            config,
            mqtt_client,
        })
    }

    pub fn get_task_supervisor(&self) -> &TaskSupervisor {
        &self.task_supervisor
    }

    pub fn get_task_supervisor_mut(&mut self) -> &mut TaskSupervisor {
        &mut self.task_supervisor
    }

    pub async fn start_monitoring(&self) -> Result<(), Box<dyn std::error::Error>> {
        let interval = Duration::from_millis(self.config.health.interval_ms);
        info!(interval=?interval, "⏱️  Starting comprehensive health monitoring loop");

        loop {
            sleep(interval).await;

            match self.collect_and_act().await {
                Ok(health_event) => {
                    // Save snapshot
                    if let Err(e) = self.snapshotter.save_snapshot(&health_event) {
                        error!(error=%e, "Failed to save health snapshot");
                    }

                    // Send to streamer
                    let stream_event = StreamEvent::new_health_event(health_event.clone(), &self.config.device_id);
                    if self.tx.send(stream_event).is_err() {
                        warn!("Health event channel full — dropping event");
                    }

                    // Update metrics
                    metrics::gauge!("health_cpu_percent").set(health_event.resources.cpu_percent as f64);
                    metrics::gauge!("health_memory_percent").set(health_event.resources.memory_percent as f64);
                    metrics::gauge!("health_disk_percent").set(health_event.resources.disk_percent as f64);
                    metrics::gauge!("health_temperature_c").set(health_event.resources.temperature_c as f64);
                    metrics::gauge!("health_network_latency_ms").set(health_event.network.latency_ms as f64);
                    metrics::gauge!("health_status").set(match health_event.status {
                        HealthStatus::Ok => 0.0,
                        HealthStatus::Warning => 1.0,
                        HealthStatus::Critical => 2.0,
                        HealthStatus::Degraded => 3.0,
                        HealthStatus::ShutdownPending => 4.0,
                    });
                    metrics::counter!("health_alerts_total").increment(health_event.alerts.len() as u64);
                    metrics::counter!("health_actions_taken_total").increment(health_event.actions_taken.len() as u64);
                }
                Err(e) => {
                    error!(error=%e, "Failed to collect health event");
                }
            }
        }
    }

    async fn collect_and_act(&self) -> Result<HealthEvent, Box<dyn std::error::Error>> {
        // Collect system health
        let (resources, mut system_alerts) = self.system_monitor.collect()?;

        // Collect network health
        let (network, mut network_alerts) = self.network_monitor.collect().await?;

        // Check tasks
        let (tasks, mut task_alerts) = self.task_supervisor.check_tasks();

        // Thermal management
        let (mut thermal_alerts, mut thermal_actions) = self.thermal_manager.check_thermal(resources.temperature_c);

        // Disk pressure management
        let (mut disk_alerts, mut disk_actions) = self.disk_pressure_manager.check_disk_pressure(resources.disk_percent);

        // Adaptive control
        let adaptive_actions = self.adaptive_controller.evaluate_system_health(
            resources.cpu_percent,
            resources.memory_percent,
            resources.disk_percent,
            network.latency_ms,
        );

        // Combine all alerts
        let mut all_alerts = Vec::new();
        all_alerts.extend(system_alerts);
        all_alerts.extend(network_alerts);
        all_alerts.extend(task_alerts);
        all_alerts.extend(thermal_alerts);
        all_alerts.extend(disk_alerts);

        // Process alerts (debounce, log, trigger local)
        let processed_alerts = self.alert_manager.process_alerts(all_alerts);

        // Determine overall status
        let mut status = HealthStatus::Ok;
        for alert in &processed_alerts {
            match alert.severity {
                AlertSeverity::Critical => {
                    status = HealthStatus::Critical;
                    break;
                }
                AlertSeverity::Warning => {
                    if status == HealthStatus::Ok {
                        status = HealthStatus::Warning;
                    }
                }
                _ => {}
            }
        }

        // If we took thermal shutdown action, set status to ShutdownPending
        for action in &thermal_actions {
            if action.action_type == ActionType::RebootSystem {
                status = HealthStatus::ShutdownPending;
            }
        }

        // Combine all actions
        let mut all_actions = Vec::new();
        all_actions.extend(thermal_actions);
        all_actions.extend(disk_actions);
        all_actions.extend(adaptive_actions);

        // Create health event
        let mut event = HealthEvent::new(status, resources, &self.config.device_id);
        event.network = network;
        event.tasks = tasks;
        event.alerts = processed_alerts;
        event.actions_taken = all_actions;
        event.meta.location = None; // Would be filled from GPS

        Ok(event)
    }
}

// Add to StreamEvent
impl StreamEvent {
    pub fn new_health_event(health_event: HealthEvent, device_id: &str) -> Self {
        Self {
            event_id: health_event.event_id.clone(),
            event_type: crate::stream::types::EventType::Health,
            payload: crate::stream::types::EventPayload::Health(health_event),
            timestamp: health_event.timestamp,
            meta crate::stream::types::EventMetadata {
                device_id: device_id.to_string(),
                truck_id: device_id.to_string(),
                sequence_number: 0,
                retry_count: 0,
                source_module: "health".to_string(),
            },
        }
    }
}