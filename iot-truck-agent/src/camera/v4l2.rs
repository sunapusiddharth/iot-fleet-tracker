use crate::camera::types::{CameraFrame, CameraId, ImageFormat, FrameMetadata, CameraConfig};
use crate::camera::encoder::FrameEncoder;
use v4l::prelude::*;
use v4l::fourcc::*;
use v4l::Device;
use bytes::Bytes;
use chrono::Utc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

pub async fn start_v4l2_camera(
    config: CameraConfig,
    frame_tx: broadcast::Sender<CameraFrame>,
    trigger_rx: broadcast::Receiver<crate::camera::types::TriggerEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let device_path = &config.device_path;
    let mut device = Device::new(device_path)
        .map_err(|e| format!("Failed to open V4L2 device {}: {}", device_path, e))?;

    info!(device=%device_path, camera=%config.camera_id, "📹 Starting V4L2 camera");

    // Configure format
    let mut format = Format::new(
        config.resolution.0,
        config.resolution.1,
        FourCC::new(b'MJPG'), // Try MJPEG first
    );

    device.set_format(&mut format)?;

    // Framerate
    let interval = Fraction::new(1, config.fps as i32);
    device.set_frame_interval(&interval)?;

    // Create stream
    let mut stream = device.stream().map_err(|e| format!("Failed to create stream: {}", e))?;

    // Start streaming
    stream.start()?;

    info!(
        device=%device_path,
        resolution=?config.resolution,
        fps=config.fps,
        "✅ V4L2 camera configured and streaming"
    );

    // Setup trigger buffer if enabled
    let mut trigger_buffer = if config.enable_trigger_buffer {
        Some(crate::camera::trigger::TriggerBuffer::new(
            &config.camera_id.to_string(),
            config.trigger_buffer_sec,
            config.fps,
        ))
    } else {
        None
    };

    // Capture loop
    let mut frame_count: u64 = 0;

    loop {
        match stream.next() {
            Some(Ok(buffer)) => {
                frame_count += 1;

                let timestamp = Utc::now();
                let (width, height) = (config.resolution.0, config.resolution.1);

                let encoded_data = match config.format {
                    ImageFormat::Jpeg => {
                        if buffer.fourcc() == FourCC::new(b'MJPG') {
                            // Already JPEG — pass through
                            buffer.to_vec()
                        } else if buffer.fourcc() == FourCC::new(b'RGB3') {
                            // Convert RGB to JPEG
                            FrameEncoder::encode_rgb_to_jpeg(
                                buffer.as_bytes(),
                                width,
                                height,
                                config.encode_quality,
                            )?
                        } else {
                            warn!(
                                fourcc=?buffer.fourcc(),
                                "Unsupported format — dropping frame"
                            );
                            continue;
                        }
                    }
                    ImageFormat::RawRgb => buffer.to_vec(),
                    _ => {
                        warn!("Unsupported output format — dropping frame");
                        continue;
                    }
                };

                let frame = CameraFrame {
                    camera_id: config.camera_id.clone(),
                    timestamp,
                    width,
                    height,
                    format: config.format.clone(),
                    data: Bytes::from(encoded_data),
                    is_keyframe: true,
                    trigger_event: None,
                    meta FrameMetadata {
                        exposure_us: None, // Can be read from V4L2 controls
                        gain_db: None,
                        temperature_c: None,
                        gps_lat: None,
                        gps_lon: None,
                        speed_kmh: None,
                    },
                };

                // Push to trigger buffer if enabled
                if let Some(ref mut buf) = trigger_buffer {
                    buf.push_frame(frame.clone());
                }

                // Send to main channel
                if frame_tx.send(frame).is_err() {
                    warn!("Camera frame channel full — dropping frame");
                }

                metrics::counter!("camera_frames_captured_total", "camera" => config.camera_id.to_string()).increment(1);
                metrics::gauge!("camera_buffer_usage", "camera" => config.camera_id.to_string()).set(trigger_buffer.as_ref().map_or(0.0, |b| b.len() as f64 / b.capacity() as f64));

                // Respect FPS
                let sleep_ms = 1000 / config.fps;
                tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms as u64)).await;
            }
            Some(Err(e)) => {
                error!(error=%e, "V4L2 buffer error");
                metrics::counter!("camera_errors_total", "camera" => config.camera_id.to_string()).increment(1);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            None => {
                warn!("V4L2 stream ended — attempting restart");
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                // Try to reinitialize
                match Device::new(device_path) {
                    Ok(new_device) => {
                        device = new_device;
                        // Reconfigure and restart stream (simplified)
                        let mut format = Format::new(width, height, FourCC::new(b'MJPG'));
                        device.set_format(&mut format)?;
                        stream = device.stream()?;
                        stream.start()?;
                        info!("✅ V4L2 camera reconnected");
                    }
                    Err(e) => {
                        error!(error=%e, "Failed to reconnect V4L2 camera");
                    }
                }
            }
        }
    }
}