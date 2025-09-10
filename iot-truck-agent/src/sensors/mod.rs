use crate::config::Config;
use crate::sensors::types::SensorEvent;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

pub mod gps;
pub mod imu;
pub mod obd;
pub mod tpms;
pub mod types;

// Metrics
metrics::describe_counter!("sensor_events_total", "Total sensor events emitted");
metrics::describe_gauge!("sensor_status", "Sensor connectivity status (1=up, 0=down)");
metrics::describe_counter!("sensor_errors_total", "Total sensor read errors");

pub async fn start_sensor_engine(
    config: &Config,
    tx: broadcast::Sender<SensorEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🚀 Starting Sensor Ingestion Engine...");

    // Start GPS if device path is configured
    if !config.sensors.gps_device.is_empty() {
        let gps_tx = tx.clone();
        let gps_device = config.sensors.gps_device.clone();
        tokio::spawn(async move {
            if let Err(e) = gps::start_gps_reader(gps_device, gps_tx).await {
                error!(sensor="gps", error=%e, "GPS reader failed");
            }
        });
    }

    // Start OBD if configured
    if !config.sensors.obd_device.is_empty() {
        let obd_tx = tx.clone();
        let obd_device = config.sensors.obd_device.clone();
        tokio::spawn(async move {
            if let Err(e) = obd::start_obd_reader(obd_device, obd_tx).await {
                error!(sensor="obd", error=%e, "OBD reader failed");
            }
        });
    }

    // Start IMU if configured
    if !config.sensors.imu_device.is_empty() {
        let imu_tx = tx.clone();
        let imu_device = config.sensors.imu_device.clone();
        tokio::spawn(async move {
            if let Err(e) = imu::start_imu_reader(imu_device, imu_tx).await {
                error!(sensor="imu", error=%e, "IMU reader failed");
            }
        });
    }

    // TPMS — future
    // if config has CAN or RF config → start_tpms_reader()

    info!(
        "✅ Sensor engine started — monitoring {} sensors",
        active_sensor_count(config)
    );
    Ok(())
}

fn active_sensor_count(config: &Config) -> usize {
    let mut count = 0;
    if !config.sensors.gps_device.is_empty() {
        count += 1;
    }
    if !config.sensors.obd_device.is_empty() {
        count += 1;
    }
    if !config.sensors.imu_device.is_empty() {
        count += 1;
    }
    count
}
//In each module, add heartbeats: TODO
// In sensor loop
// loop {
//     // ... your code
//     sensor_monitor.heartbeat(); // or task_monitor.heartbeat("sensor_engine");
//     tokio::time::sleep(Duration::from_millis(100)).await;
// }
