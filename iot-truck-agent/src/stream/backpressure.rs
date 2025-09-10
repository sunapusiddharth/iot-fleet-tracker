use crate::wal::WalManager;
use std::sync::Arc;
use tokio::sync::RwLock;

static WAL_MANAGER: once_cell::sync::Lazy<Arc<RwLock<Option<WalManager>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(None)));

pub async fn init_wal_manager(wal_manager: WalManager) {
    let mut wal = WAL_MANAGER.write().await;
    *wal = Some(wal_manager);
}

pub async fn notify_wal_ack(event_id: &str) {
    let wal = WAL_MANAGER.read().await;
    if let Some(wal_manager) = wal.as_ref() {
        // In production, call wal_manager.mark_acked(event_id)
        tracing::trace!(event_id, "🗑️  WAL entry acknowledged");
    }
}

pub async fn buffer_to_wal(event: crate::stream::types::StreamEvent) -> Result<(), Box<dyn std::error::Error>> {
    let wal = WAL_MANAGER.read().await;
    if let Some(wal_manager) = wal.as_ref() {
        // Convert to WAL entry and write
        match event.event_type {
            crate::stream::types::EventType::Sensor => {
                if let crate::stream::types::EventPayload::Sensor(sensor_event) = event.payload {
                    wal_manager.write_sensor(sensor_event).await?;
                }
            }
            crate::stream::types::EventType::CameraMeta => {
                if let crate::stream::types::EventPayload::CameraMeta(meta) = event.payload {
                    // Convert to CameraFrame and write
                    let frame = crate::camera::types::CameraFrame {
                        camera_id: crate::camera::types::CameraId::Custom("stream".to_string()),
                        timestamp: chrono::Utc::now(),
                        width: meta.width,
                        height: meta.height,
                        format: crate::camera::types::ImageFormat::Jpeg,
                         bytes::Bytes::new(),
                        is_keyframe: meta.is_keyframe,
                        trigger_event: meta.trigger_event,
                        meta crate::camera::types::FrameMetadata {
                            exposure_us: None,
                            gain_db: None,
                            temperature_c: None,
                            gps_lat: None,
                            gps_lon: None,
                            speed_kmh: None,
                        },
                    };
                    wal_manager.write_camera_frame(frame).await?;
                }
            }
            _ => {
                // Write as generic WAL entry
                let wal_entry = crate::wal::types::WalEntry::Heartbeat(crate::wal::types::HeartbeatMarker {
                    timestamp: chrono::Utc::now().timestamp_nanos() as u64,
                    uptime_sec: 0,
                    memory_used_bytes: 0,
                });
                // Need to get WAL writer to write this
            }
        }
    }
    Ok(())
}