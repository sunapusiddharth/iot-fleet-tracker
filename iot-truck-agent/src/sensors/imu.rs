use crate::sensors::types::{ImuData, SensorEvent, SensorType, SensorValues};
use chrono::Utc;
use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev;
use lis3dh::Lis3dh;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

pub async fn start_imu_reader(
    device_path: String,
    tx: broadcast::Sender<SensorEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let i2c_dev = I2cdev::new(device_path)
        .map_err(|e| format!("Failed to open I2C device {}: {}", device_path, e))?;

    let mut imu = Lis3dh::new(i2c_dev, lis3dh::Address::Primary)
        .map_err(|e| format!("Failed to initialize LIS3DH: {:?}", e))?;

    // Configure: 100Hz, ±2g
    imu.set_sample_rate(lis3dh::SampleRate::Hz100)?;
    imu.set_scale(lis3dh::Scale::G2)?;

    info!(device=%device_path, "🌀 IMU reader started");

    let sensor_id = device_path.clone();

    loop {
        match read_imu_data(&mut imu) {
            Ok(data) => {
                let event = SensorEvent {
                    sensor_id: sensor_id.clone(),
                    sensor_type: SensorType::Imu,
                    timestamp: Utc::now(),
                    values: SensorValues::Imu(data),
                    raw_payload: None,
                };

                if tx.send(event).is_err() {
                    warn!("Sensor channel receiver dropped");
                }
                metrics::counter!("sensor_events_total", "sensor" => "imu").increment(1);
            }
            Err(e) => {
                error!(error=%e, "IMU read error");
                metrics::counter!("sensor_errors_total", "sensor" => "imu").increment(1);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await; // 100 Hz
    }
}

fn read_imu_data(imu: &mut Lis3dh<I2cdev>) -> Result<ImuData, Box<dyn std::error::Error>> {
    let accel = imu.accel_raw()?;
    let g = 0.00098; // LSB to g (for ±2g scale)

    Ok(ImuData {
        accel_x: (accel.x as f32) * g,
        accel_y: (accel.y as f32) * g,
        accel_z: (accel.z as f32) * g,
        gyro_x: 0.0, // LIS3DH doesn't have gyro
        gyro_y: 0.0,
        gyro_z: 0.0,
    })
}
