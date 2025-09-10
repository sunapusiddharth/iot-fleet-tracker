use crate::camera::types::{CameraFrame, TriggerEvent};
use ringbuf::{RingBuffer, Producer, Consumer};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tokio::sync::broadcast;
use tracing::{info, warn};

pub struct TriggerBuffer {
    camera_id: String,
    buffer: RingBuffer<CameraFrame>,
    producer: Producer<CameraFrame>,
    consumer: Consumer<CameraFrame>,
    max_duration_sec: u32,
    fps: u32,
}

impl TriggerBuffer {
    pub fn new(camera_id: &str, duration_sec: u32, fps: u32) -> Self {
        let capacity = (duration_sec * fps) as usize;
        let (prod, cons) = RingBuffer::new(capacity).split();

        Self {
            camera_id: camera_id.to_string(),
            buffer: RingBuffer::new(capacity),
            producer: prod,
            consumer: cons,
            max_duration_sec: duration_sec,
            fps,
        }
    }

    // Push frame into ring buffer (overwrites oldest)
    pub fn push_frame(&mut self, frame: CameraFrame) {
        if self.producer.push(frame).is_err() {
            warn!(camera=%self.camera_id, "Trigger buffer full — dropping oldest frame");
        }
    }

    // On trigger event, extract frames from buffer
    pub fn extract_on_trigger(&mut self, trigger: TriggerEvent) -> Vec<CameraFrame> {
        let mut frames = Vec::new();
        let target_count = (trigger.duration_sec * self.fps) as usize;

        // Drain all available frames (up to target)
        while frames.len() < target_count {
            if let Ok(frame) = self.consumer.pop() {
                let mut frame_with_trigger = frame;
                frame_with_trigger.trigger_event = Some(trigger.event_type.clone());
                frames.push(frame_with_trigger);
            } else {
                break;
            }
        }

        info!(
            camera=%self.camera_id,
            event=%trigger.event_type,
            frames_captured=frames.len(),
            "📸 Triggered capture"
        );

        frames
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

// Listen for sensor events that should trigger capture
pub async fn start_trigger_listener(
    mut sensor_rx: broadcast::Receiver<crate::sensors::types::SensorEvent>,
    trigger_tx: broadcast::Sender<TriggerEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("👂 Starting trigger event listener...");

    while let Ok(event) = sensor_rx.recv().await {
        // Example: harsh braking → g-force > 0.5
        if let crate::sensors::types::SensorValues::Imu(imu) = &event.values {
            let g_force = (imu.accel_x.powi(2) + imu.accel_y.powi(2) + imu.accel_z.powi(2)).sqrt();
            if g_force > 0.5 {
                let trigger = TriggerEvent {
                    event_type: "harsh_brake".to_string(),
                    severity: g_force,
                    duration_sec: 5, // capture 5 seconds
                };

                if trigger_tx.send(trigger).is_err() {
                    warn!("Trigger channel full — dropping event");
                }
            }
        }
    }

    Ok(())
}