use crate::sensors::types::{SensorEvent, SensorType, SensorValues, ObdData};
use chrono::Utc;
use tokio::sync::broadcast;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tracing::{error, info, warn};

pub async fn start_obd_reader(
    device_path: String,
    tx: broadcast::Sender<SensorEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut port = tokio_serial::new(&device_path, 38400) // ELM327 default
        .open_native_async()
        .map_err(|e| format!("Failed to open OBD device {}: {}", device_path, e))?;

    info!(device=%device_path, "🔌 OBD reader started");

    // Initialize ELM327
    initialize_elm327(&mut port).await?;

    let sensor_id = device_path.clone();
    let sample_rate_ms = 1000 / 10; // 10 Hz from config

    loop {
        let obd_data = read_obd_frame(&mut port).await;

        match obd_data {
            Ok(Some(data)) => {
                let event = SensorEvent {
                    sensor_id: sensor_id.clone(),
                    sensor_type: SensorType::Obd,
                    timestamp: Utc::now(),
                    values: SensorValues::Obd(data),
                    raw_payload: None,
                };

                if tx.send(event).is_err() {
                    warn!("Sensor channel receiver dropped");
                }
                metrics::counter!("sensor_events_total", "sensor" => "obd").increment(1);
            }
            Ok(None) => {
                // No data — not an error
            }
            Err(e) => {
                error!(error=%e, "OBD read error");
                metrics::counter!("sensor_errors_total", "sensor" => "obd").increment(1);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(sample_rate_ms)).await;
    }
}

async fn initialize_elm327(port: &mut SerialStream) -> Result<(), Box<dyn std::error::Error>> {
    // Reset
    write_command(port, "ATZ\r").await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Echo off
    write_command(port, "ATE0\r").await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Set headers auto
    write_command(port, "ATH1\r").await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Set protocol auto
    write_command(port, "ATSP0\r").await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    info!("✅ ELM327 initialized");
    Ok(())
}

async fn write_command(port: &mut SerialStream, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    port.write_all(cmd.as_bytes()).await?;
    port.flush().await?;
    Ok(())
}

async fn read_obd_frame(port: &mut SerialStream) -> Result<Option<ObdData>, Box<dyn std::error::Error>> {
    // Request RPM
    write_command(port, "010C\r").await?;
    let rpm = read_pid_response(port, "41 0C").await?;

    // Request Speed
    write_command(port, "010D\r").await?;
    let speed = read_pid_response(port, "41 0D").await?;

    // Request Coolant Temp
    write_command(port, "0105\r").await?;
    let temp = read_pid_response(port, "41 05").await?;

    // Request Fuel Level
    write_command(port, "012F\r").await?;
    let fuel = read_pid_response(port, "41 2F").await?;

    // Request Engine Load
    write_command(port, "0104\r").await?;
    let load = read_pid_response(port, "41 04").await?;

    // Request Throttle Position
    write_command(port, "0111\r").await?;
    let throttle = read_pid_response(port, "41 11").await?;

    Ok(Some(ObdData {
        rpm: rpm.unwrap_or(0),
        speed_kmh: speed.unwrap_or(0) as u8,
        coolant_temp: (temp.unwrap_or(0) as i32 - 40) as i8,
        fuel_level: fuel.unwrap_or(0),
        engine_load: load.unwrap_or(0),
        throttle_pos: throttle.unwrap_or(0),
    }))
}

async fn read_pid_response(
    port: &mut SerialStream,
    expected_prefix: &str,
) -> Result<Option<u16>, Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; 256];
    let timeout = tokio::time::sleep(tokio::time::Duration::from_millis(200));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            _ = &mut timeout => {
                return Ok(None); // Timeout
            }
            result = port.read(&mut buf) => {
                let n = result?;
                let response = String::from_utf8_lossy(&buf[..n]);
                if response.contains(expected_prefix) {
                    let hex_str = response.split_whitespace().collect::<Vec<_>>();
                    if hex_str.len() >= 4 {
                        if let (Ok(a), Ok(b)) = (u8::from_str_radix(hex_str[2], 16), u8::from_str_radix(hex_str[3], 16)) {
                            return Ok(Some(((a as u16) << 8) | (b as u16)));
                        }
                    }
                }
            }
        }
    }
}