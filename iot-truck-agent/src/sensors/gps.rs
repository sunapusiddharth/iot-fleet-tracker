use crate::sensors::types::{SensorEvent, SensorType, SensorValues, GpsData};
use chrono::Utc;
use nmea::{parse, SentenceType};
use tokio::sync::broadcast;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tracing::{error, info, warn};

pub async fn start_gps_reader(
    device_path: String,
    tx: broadcast::Sender<SensorEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut port = tokio_serial::new(&device_path, 9600)
        .open_native_async()
        .map_err(|e| format!("Failed to open GPS device {}: {}", device_path, e))?;

    info!(device=%device_path, "📡 GPS reader started");

    let mut buf = String::new();
    let sensor_id = device_path.clone();

    loop {
        buf.clear();
        match tokio::io::read_line(&mut port, &mut buf).await {
            Ok(0) => {
                // EOF — device disconnected
                warn!(device=%device_path, "GPS device disconnected");
                metrics::gauge!("sensor_status", "sensor" => "gps").set(0.0);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                // Try to reopen
                match tokio_serial::new(&device_path, 9600).open_native_async() {
                    Ok(new_port) => {
                        port = new_port;
                        info!(device=%device_path, "✅ GPS device reconnected");
                        metrics::gauge!("sensor_status", "sensor" => "gps").set(1.0);
                    }
                    Err(e) => {
                        error!(device=%device_path, error=%e, "Failed to reconnect GPS");
                        continue;
                    }
                }
            }
            Ok(_) => {
                // Parse NMEA
                if let Some(sentence) = buf.strip_suffix('\n').or_else(|| buf.strip_suffix('\r')) {
                    if let Ok(parsed) = parse(sentence) {
                        match parsed.sentence_type {
                            SentenceType::GGA | SentenceType::RMC => {
                                if let Some(gps_data) = extract_gps_data(&parsed) {
                                    let event = SensorEvent {
                                        sensor_id: sensor_id.clone(),
                                        sensor_type: SensorType::Gps,
                                        timestamp: Utc::now(),
                                        values: SensorValues::Gps(gps_data),
                                        raw_payload: Some(sentence.to_string()),
                                    };

                                    if tx.send(event).is_err() {
                                        warn!("Sensor channel receiver dropped — no consumers");
                                    }
                                    metrics::counter!("sensor_events_total", "sensor" => "gps").increment(1);
                                }
                            }
                            _ => {} // Ignore other sentences
                        }
                    } else {
                        // Log malformed but don't crash
                        metrics::counter!("sensor_errors_total", "sensor" => "gps").increment(1);
                        warn!(sentence=%sentence, "Malformed NMEA sentence");
                    }
                }
            }
            Err(e) => {
                error!(error=%e, "GPS read error");
                metrics::counter!("sensor_errors_total", "sensor" => "gps").increment(1);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
}

fn extract_gps_data(sentence: &nmea::Sentence) -> Option<GpsData> {
    match sentence {
        nmea::Sentence::GGA(gga) => {
            Some(GpsData {
                latitude: gga.latitude.unwrap_or(0.0),
                longitude: gga.longitude.unwrap_or(00.0),
                altitude: gga.altitude.unwrap_or(0.0) as f32,
                speed_kmh: 0.0, // Not in GGA
                heading: 0.0,   // Not in GGA
                satellites: gga.satellites.unwrap_or(0) as u8,
                fix_quality: gga.fix_quality as u8,
            })
        }
        nmea::Sentence::RMC(rmc) => {
            Some(GpsData {
                latitude: rmc.latitude.unwrap_or(0.0),
                longitude: rmc.longitude.unwrap_or(0.0),
                altitude: 0.0,
                speed_kmh: rmc.speed_over_ground.unwrap_or(0.0) as f32 * 1.852, // knots to km/h
                heading: rmc.course_over_ground.unwrap_or(0.0) as f32,
                satellites: 0,
                fix_quality: if rmc.is_valid { 1 } else { 0 },
            })
        }
        _ => None,
    }
}