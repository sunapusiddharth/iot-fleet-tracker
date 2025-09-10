use iot_truck_agent::telemetry;
use std::sync::Arc;
use tokio;
use tracing::{Level, info};

mod alert;
mod camera;
mod config;
mod health;
mod ml_edge;
mod ota;
mod sensors;
mod stream;
mod supervisor;
mod wal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging and metrics
    telemetry::init_tracing();
    telemetry::init_metrics("0.0.0.0:9090".parse().unwrap())?;

    info!("🚛 Starting Truck IoT Agent — COMPLETE SYSTEM");

    // Load configuration
    let config_path = "config/agent.toml";
    let config = config::Config::load_from_file(config_path).map_err(|e| {
        tracing::error!(error = %e, "❌ Failed to load config — CRASHING");
        e
    })?;
    config::Config::set_global(config.clone())?;

    // Initialize system state
    supervisor::lifecycle::state::initialize_system_state(&config.device_id);

    // Create channels for inter-module communication
    let (sensor_tx, sensor_rx) = tokio::sync::broadcast::channel(1000);
    let (camera_tx, camera_rx) = tokio::sync::broadcast::channel(1000);
    let (ml_tx, ml_rx) = tokio::sync::broadcast::channel(1000);
    let (health_tx, health_rx) = tokio::sync::broadcast::channel(1000);
    let (stream_tx, stream_rx) = tokio::sync::broadcast::channel(1000);
    let (alert_tx, alert_rx) = tokio::sync::broadcast::channel(1000);
    let (ota_command_tx, ota_command_rx) = tokio::sync::mpsc::channel(100);
    let (ota_response_tx, _ota_response_rx) = tokio::sync::mpsc::channel(100);

    // Create shared resource usage for ML and health modules
    let resource_usage = Arc::new(tokio::sync::RwLock::new(health::types::ResourceUsage {
        cpu_percent: 0.0,
        memory_percent: 0.0,
        memory_used_mb: 0,
        memory_total_mb: 0,
        memory_available_mb: 0,
        swap_percent: 0.0,
        disk_percent: 0.0,
        disk_used_gb: 0,
        disk_total_gb: 0,
        disk_available_gb: 0,
        temperature_c: 0.0,
        thermal_throttling: false,
        uptime_sec: 0,
        load_average: (0.0, 0.0, 0.0),
    }));

    // Initialize WAL Manager
    info!("📂 Initializing WAL Manager");
    let wal_manager = wal::WalManager::new(&config, resource_usage.clone()).await?;
    let wal_manager_clone = wal_manager.clone();

    // Initialize Stream Manager
    info!("📡 Initializing Stream Manager");
    let mut stream_manager = stream::StreamManager::new(&config, wal_manager.clone()).await?;
    let stream_manager_clone = stream_manager.clone();

    // Initialize ML Edge Manager
    info!("🧠 Initializing ML Edge Manager");
    let ml_manager =
        ml_edge::MLEdgeManager::new(&config, ml_tx.clone(), resource_usage.clone()).await?;
    let ml_manager_clone = ml_manager.clone();

    // Initialize Health Manager
    info!("🏥 Initializing Health Manager");
    let health_manager = health::HealthManager::new(config.clone(), health_tx.clone()).await?;
    let health_manager_clone = health_manager.clone();
    let task_supervisor = health_manager.get_task_supervisor_mut();

    // Initialize Alert Manager
    info!("🚨 Initializing Alert Manager");
    let alert_manager = alert::AlertManager::new(&config, alert_tx.clone()).await?;
    let alert_manager_clone = alert_manager.clone();

    // Initialize OTA Manager
    info!("🔄 Initializing OTA Manager");
    let network_health = Arc::new(tokio::sync::RwLock::new(stream::types::NetworkHealth {
        mqtt_connected: true,
        http_connected: true,
        latency_ms: 0.0,
        last_heartbeat_ack: 0,
        packets_lost: 0,
        bandwidth_kbps: 1000.0,
    }));
    let ota_manager = ota::OtaManager::new(
        config.clone(),
        network_health.clone(),
        ota_command_rx,
        ota_response_tx,
    )
    .await?;
    let ota_manager_clone = ota_manager.clone();

    // Initialize Supervisor
    info!("🛡️  Initializing Supervisor");
    let supervisor_manager = supervisor::Supervisor::new(config.clone(), stream_tx.clone()).await?;
    let mut supervisor_health_monitor = supervisor_manager.get_health_monitor();

    // Register modules with supervisor health monitor
    let sensor_monitor = supervisor_health_monitor.register_module("sensor_engine");
    let camera_monitor = supervisor_health_monitor.register_module("camera_engine");
    let wal_monitor = supervisor_health_monitor.register_module("wal_engine");
    let stream_monitor = supervisor_health_monitor.register_module("stream_engine");
    let ml_monitor = supervisor_health_monitor.register_module("ml_engine");
    let health_monitor = supervisor_health_monitor.register_module("health_engine");
    let alert_monitor = supervisor_health_monitor.register_module("alert_engine");
    let ota_monitor = supervisor_health_monitor.register_module("ota_engine");

    // Start Sensor Engine
    info!("📡 Starting Sensor Engine");
    let config_clone1 = config.clone();
    let sensor_tx_clone1 = sensor_tx.clone();
    let wal_manager_clone1 = wal_manager.clone();
    let stream_manager_clone1 = stream_manager.clone();
    let ml_manager_clone1 = ml_manager.clone();
    let alert_manager_clone1 = alert_manager.clone();
    let sensor_monitor_clone = sensor_monitor.clone();

    tokio::spawn(async move {
        if let Err(e) = sensors::start_sensor_engine(&config_clone1, sensor_tx_clone1).await {
            tracing::error!(error = %e, "Sensor engine crashed");
            supervisor_manager.emergency_shutdown().await;
        }
    });

    // Start Camera Engine
    info!("📹 Starting Camera Engine");
    let config_clone2 = config.clone();
    let camera_tx_clone2 = camera_tx.clone();
    let sensor_rx_clone2 = sensor_rx.subscribe();
    let wal_manager_clone2 = wal_manager.clone();
    let stream_manager_clone2 = stream_manager.clone();
    let ml_manager_clone2 = ml_manager.clone();
    let alert_manager_clone2 = alert_manager.clone();
    let camera_monitor_clone = camera_monitor.clone();

    tokio::spawn(async move {
        if let Err(e) =
            camera::start_camera_engine(&config_clone2, camera_tx_clone2, sensor_rx_clone2).await
        {
            tracing::error!(error = %e, "Camera engine crashed");
            supervisor_manager.emergency_shutdown().await;
        }
    });

    // Start ML Edge Manager processing
    info!("🧠 Starting ML Edge Processing");
    let ml_manager_clone3 = ml_manager.clone();
    let camera_rx_clone3 = camera_rx.subscribe();
    let sensor_rx_clone3 = sensor_rx.subscribe();
    let stream_manager_clone3 = stream_manager.clone();
    let alert_manager_clone3 = alert_manager.clone();
    let ml_monitor_clone = ml_monitor.clone();

    tokio::spawn(async move {
        let mut camera_rx = camera_rx_clone3;
        let mut sensor_rx = sensor_rx_clone3;

        loop {
            tokio::select! {
                Ok(frame) = camera_rx.recv() => {
                    if let Err(e) = ml_manager_clone3.process_frame(&frame).await {
                        tracing::error!(error = %e, "ML processing failed for frame");
                    }
                    ml_monitor_clone.heartbeat();
                }
                Ok(event) = sensor_rx.recv() => {
                    ml_manager_clone3.add_sensor_event(&event).await;
                    ml_monitor_clone.heartbeat();
                }
            }
        }
    });

    // Start Health Manager monitoring
    info!("🏥 Starting Health Monitoring");
    let health_manager_clone4 = health_manager.clone();
    let resource_usage_clone4 = resource_usage.clone();
    let health_monitor_clone = health_monitor.clone();

    tokio::spawn(async move {
        // Update resource usage from health events
        let mut health_rx = health_manager_clone4.get_health_receiver().await;
        loop {
            if let Ok(health_event) = health_rx.recv().await {
                let mut usage = resource_usage_clone4.write().await;
                *usage = health_event.resources.clone();

                // Update supervisor module state
                supervisor::lifecycle::state::update_module_state(
                    "health_engine",
                    supervisor::types::ModuleStatus::Running,
                    health_event.resources.cpu_percent,
                    health_event.resources.memory_used_mb,
                );

                health_monitor_clone.heartbeat();
            }
        }
    });

    // Start Health Manager main loop
    let health_manager_clone5 = health_manager.clone();
    tokio::spawn(async move {
        if let Err(e) = health_manager_clone5.start_monitoring().await {
            tracing::error!(error = %e, "Health monitoring crashed");
            supervisor_manager.emergency_shutdown().await;
        }
    });

    // Start Alert Manager processing
    info!("🚨 Starting Alert Processing");
    let alert_manager_clone6 = alert_manager.clone();
    let ml_rx_clone6 = ml_rx.subscribe();
    let health_rx_clone6 = health_rx.subscribe();
    let sensor_rx_clone6 = sensor_rx.subscribe();
    let alert_monitor_clone = alert_monitor.clone();

    tokio::spawn(async move {
        let mut ml_rx = ml_rx_clone6;
        let mut health_rx = health_rx_clone6;
        let mut sensor_rx = sensor_rx_clone6;

        loop {
            tokio::select! {
                Ok(ml_event) = ml_rx.recv() => {
                    if let Err(e) = alert_manager_clone6.process_ml_event(&ml_event).await {
                        tracing::error!(error = %e, "Failed to process ML alert");
                    }
                    alert_monitor_clone.heartbeat();
                }
                Ok(health_event) = health_rx.recv() => {
                    if let Err(e) = alert_manager_clone6.process_health_event(&health_event).await {
                        tracing::error!(error = %e, "Failed to process health alert");
                    }
                    alert_monitor_clone.heartbeat();
                }
                Ok(sensor_event) = sensor_rx.recv() => {
                    if let Err(e) = alert_manager_clone6.process_sensor_event(&sensor_event).await {
                        tracing::error!(error = %e, "Failed to process sensor alert");
                    }
                    alert_monitor_clone.heartbeat();
                }
            }
        }
    });

    // Start OTA Manager
    info!("🔄 Starting OTA Manager");
    let ota_manager_clone7 = ota_manager.clone();
    let ota_monitor_clone = ota_monitor.clone();

    tokio::spawn(async move {
        if let Err(e) = ota_manager_clone7.start().await {
            tracing::error!(error = %e, "OTA manager crashed");
            supervisor_manager.emergency_shutdown().await;
        }
        ota_monitor_clone.heartbeat();
    });

    // Start Stream Manager main loop
    info!("📡 Starting Stream Manager");
    let stream_manager_clone8 = stream_manager.clone();
    let stream_monitor_clone = stream_monitor.clone();

    tokio::spawn(async move {
        if let Err(e) = stream_manager_clone8.start_streaming_loop().await {
            tracing::error!(error = %e, "Streaming loop crashed");
            supervisor_manager.emergency_shutdown().await;
        }
        stream_monitor_clone.heartbeat();
    });

    // Start Supervisor monitoring
    info!("🛡️  Starting Supervisor Monitoring");
    let supervisor_manager_clone9 = supervisor_manager.clone();
    let wal_monitor_clone = wal_monitor.clone();
    let stream_monitor_clone9 = stream_monitor.clone();

    tokio::spawn(async move {
        if let Err(e) = supervisor_manager_clone9.start_monitoring().await {
            tracing::error!(error = %e, "Supervisor monitoring crashed");
            supervisor_manager_clone9.emergency_shutdown().await;
        }
        wal_monitor_clone.heartbeat();
        stream_monitor_clone9.heartbeat();
    });

    // Start WAL Manager background tasks
    info!("📂 Starting WAL Manager Background Tasks");
    let wal_manager_clone10 = wal_manager.clone();
    let wal_monitor_clone10 = wal_monitor.clone();

    tokio::spawn(async move {
        // In production, start WAL compaction and retention tasks
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
            wal_monitor_clone10.heartbeat();
        }
    });

    // Handle sensor events
    tokio::spawn(async move {
        let mut rx = sensor_rx;
        let wal_manager = wal_manager_clone1;
        let stream_manager = stream_manager_clone1;
        let ml_manager = ml_manager_clone1;
        let alert_manager = alert_manager_clone1;
        let sensor_monitor = sensor_monitor_clone;
        let sequence_number = std::sync::atomic::AtomicU64::new(1);

        while let Ok(event) = rx.recv().await {
            // Write to WAL
            if let Err(e) = wal_manager.write_sensor(event.clone()).await {
                tracing::error!(error = %e, "Failed to write sensor to WAL");
            }

            // Send to streamer
            let seq = sequence_number.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if let Err(e) = stream_manager
                .send_event(stream::types::StreamEvent::new_sensor(
                    event.clone(),
                    &config.device_id,
                    seq,
                ))
                .await
            {
                tracing::error!(error = %e, "Failed to queue sensor for streaming");
            }

            // Send to ML manager for fusion
            if let Err(e) = ml_manager.add_sensor_event(&event).await {
                tracing::error!(error = %e, "Failed to add sensor event to ML manager");
            }

            sensor_monitor.heartbeat();
        }
    });

    // Handle camera events
    tokio::spawn(async move {
        let mut rx = camera_rx;
        let wal_manager = wal_manager_clone2;
        let stream_manager = stream_manager_clone2;
        let ml_manager = ml_manager_clone2;
        let alert_manager = alert_manager_clone2;
        let camera_monitor = camera_monitor_clone;
        let sequence_number = std::sync::atomic::AtomicU64::new(1);

        while let Ok(frame) = rx.recv().await {
            // Write to WAL
            if let Err(e) = wal_manager.write_camera_frame(frame.clone()).await {
                tracing::error!(error = %e, "Failed to write camera frame to WAL");
            }

            // Send to streamer
            let seq = sequence_number.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if let Err(e) = stream_manager
                .send_event(stream::types::StreamEvent::new_camera_frame(
                    frame.clone(),
                    &config.device_id,
                    seq,
                ))
                .await
            {
                tracing::error!(error = %e, "Failed to queue camera frame for streaming");
            }

            camera_monitor.heartbeat();
        }
    });

    // Handle ML events
    tokio::spawn(async move {
        let mut rx = ml_rx;
        let stream_manager = stream_manager_clone3;
        let alert_manager = alert_manager_clone3;
        let ml_monitor = ml_monitor_clone;
        let sequence_number = std::sync::atomic::AtomicU64::new(1);

        while let Ok(event) = rx.recv().await {
            // Send to streamer
            let seq = sequence_number.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if let Err(e) = stream_manager
                .send_event(stream::types::StreamEvent::new_ml_event(
                    event.clone(),
                    &config.device_id,
                    seq,
                ))
                .await
            {
                tracing::error!(error = %e, "Failed to queue ML event for streaming");
            }

            ml_monitor.heartbeat();
        }
    });

    // Handle health events
    tokio::spawn(async move {
        let mut rx = health_rx;
        let stream_manager = stream_manager_clone;
        let health_monitor = health_monitor_clone;
        let sequence_number = std::sync::atomic::AtomicU64::new(1);

        while let Ok(event) = rx.recv().await {
            // Send to streamer
            let seq = sequence_number.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if let Err(e) = stream_manager
                .send_event(stream::types::StreamEvent::new_health_event(
                    event.clone(),
                    &config.device_id,
                    seq,
                ))
                .await
            {
                tracing::error!(error = %e, "Failed to queue health event for streaming");
            }

            health_monitor.heartbeat();
        }
    });

    // Handle alert events
    tokio::spawn(async move {
        let mut rx = alert_rx;
        let stream_manager = stream_manager_clone;
        let alert_monitor = alert_monitor_clone;
        let sequence_number = std::sync::atomic::AtomicU64::new(1);

        while let Ok(event) = rx.recv().await {
            // Send to streamer
            let seq = sequence_number.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if let Err(e) = stream_manager
                .send_event(stream::types::StreamEvent::new_alert_event(
                    event.clone(),
                    &config.device_id,
                    seq,
                ))
                .await
            {
                tracing::error!(error = %e, "Failed to queue alert event for streaming");
            }

            alert_monitor.heartbeat();
        }
    });

    // Handle shutdown signals
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        info!("👋 Ctrl+C received - initiating graceful shutdown");
        supervisor_manager
            .shutdown(supervisor::types::ShutdownReason::Normal)
            .await
            .expect("Shutdown failed");
        std::process::exit(0);
    });

    info!("✅ ALL MODULES STARTED SUCCESSFULLY - SYSTEM RUNNING");

    // Keep main thread alive
    tokio::signal::ctrl_c().await?;
    info!("👋 Shutdown signal received - exiting");
    Ok(())
}
