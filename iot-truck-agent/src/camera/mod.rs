use crate::camera::types::{CameraConfig, CameraFrame, CameraId};
use crate::config::Config;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

pub mod encoder;
pub mod rtsp;
pub mod trigger;
pub mod types;
pub mod v4l2; // stub for now

// Metrics
metrics::describe_counter!("camera_frames_captured_total", "Total frames captured");
metrics::describe_counter!("camera_errors_total", "Total camera errors");
metrics::describe_gauge!("camera_status", "Camera status (1=up, 0=down)");
metrics::describe_gauge!("camera_buffer_usage", "Trigger buffer usage 0.0-1.0");

pub async fn start_camera_engine(
    config: &Config,
    frame_tx: broadcast::Sender<CameraFrame>,
    sensor_rx: broadcast::Receiver<crate::sensors::types::SensorEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🎥 Starting Camera Engine...");

    // Start trigger listener
    let (trigger_tx, _) = broadcast::channel(100);
    let sensor_rx_clone = sensor_rx;
    tokio::spawn(async move {
        if let Err(e) = trigger::start_trigger_listener(sensor_rx_clone, trigger_tx.clone()).await {
            error!(error=%e, "Trigger listener failed");
        }
    });

    // Start each configured camera
    for device_path in &config.camera.devices {
        // Infer camera_id from device path or index
        let camera_id = if device_path.contains("front") {
            CameraId::Front
        } else if device_path.contains("driver") {
            CameraId::Driver
        } else if device_path.contains("cargo") {
            CameraId::Cargo
        } else {
            CameraId::Custom(device_path.clone())
        };

        // Parse resolution
        let resolution: (u32, u32) = config
            .camera
            .resolution
            .split('x')
            .filter_map(|s| s.parse().ok())
            .take(2)
            .collect::<Vec<u32>>()
            .try_into()
            .map_err(|_| "Invalid resolution format")?;

        let cam_config = CameraConfig {
            device_path: device_path.clone(),
            camera_id: camera_id.clone(),
            resolution,
            fps: config.camera.fps,
            encode_quality: config.camera.encode_quality,
            format: crate::camera::types::ImageFormat::Jpeg,
            enable_trigger_buffer: true,
            trigger_buffer_sec: 10, // configurable later
        };

        let frame_tx_clone = frame_tx.clone();
        let trigger_rx = trigger_tx.subscribe();

        // Spawn camera task
        tokio::spawn(async move {
            if device_path.starts_with("/dev/video") {
                if let Err(e) =
                    v4l2::start_v4l2_camera(cam_config, frame_tx_clone, trigger_rx).await
                {
                    error!(camera=%camera_id, error=%e, "V4L2 camera failed");
                    metrics::gauge!("camera_status", "camera" => camera_id.to_string()).set(0.0);
                }
            } else if device_path.starts_with("rtsp://") {
                // Future: rtsp::start_rtsp_camera()
                warn!(camera=%camera_id, "RTSP not implemented yet");
            } else {
                error!(camera=%camera_id, path=%device_path, "Unknown camera protocol");
            }
        });
    }

    info!(
        "✅ Camera engine started with {} cameras",
        config.camera.devices.len()
    );
    Ok(())
}
