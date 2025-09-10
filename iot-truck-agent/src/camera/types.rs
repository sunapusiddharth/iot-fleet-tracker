use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use bytes::Bytes;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum CameraId {
    Front,
    Driver,
    Cargo,
    Rear,
    Custom(String),
}

impl fmt::Display for CameraId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CameraId::Front => write!(f, "front"),
            CameraId::Driver => write!(f, "driver"),
            CameraId::Cargo => write!(f, "cargo"),
            CameraId::Rear => write!(f, "rear"),
            CameraId::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CameraFrame {
    pub camera_id: CameraId,
    pub timestamp: DateTime<Utc>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub data: Bytes,           // JPEG bytes or raw (configurable)
    pub is_keyframe: bool,     // For video streams
    pub trigger_event: Option<String>, // e.g., "harsh_brake"
    pub metadata: FrameMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    Jpeg,
    H264,
    RawRgb,
    RawYuv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    pub exposure_us: Option<u32>,
    pub gain_db: Option<f32>,
    pub temperature_c: Option<f32>,
    pub gps_lat: Option<f64>,
    pub gps_lon: Option<f64>,
    pub speed_kmh: Option<f32>,
}

impl CameraFrame {
    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Event for triggering buffered capture
#[derive(Debug, Clone)]
pub struct TriggerEvent {
    pub event_type: String,    // "harsh_brake", "drowsy_driver", etc.
    pub severity: f32,         // 0.0 to 1.0
    pub duration_sec: u32,     // Capture N seconds around event
}

// Config from Module 1
#[derive(Debug, Clone)]
pub struct CameraConfig {
    pub device_path: String,   // "/dev/video0" or "rtsp://..."
    pub camera_id: CameraId,
    pub resolution: (u32, u32), // (1280, 720)
    pub fps: u32,
    pub encode_quality: u8,    // 1-100
    pub format: ImageFormat,
    pub enable_trigger_buffer: bool,
    pub trigger_buffer_sec: u32, // Pre-record N seconds
}