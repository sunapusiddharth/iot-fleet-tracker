use crate::stream::types::{StreamEvent, CompressionType};

pub struct AdaptiveCompressor;

impl AdaptiveCompressor {
    pub fn compress_event(event: &mut StreamEvent, network_quality: &crate::stream::types::NetworkQuality) -> Result<(), Box<dyn std::error::Error>> {
        match &mut event.payload {
            crate::stream::types::EventPayload::Sensor(sensor_event) => {
                // Use delta encoding for sensor data
                let compressed = Self::delta_encode_sensor(sensor_event)?;
                event.payload = crate::stream::types::EventPayload::Sensor(compressed);
                event.metadata.encryption = None; // Delta encoding not encrypted
            }
            crate::stream::types::EventPayload::CameraBlob { data, compression_type, .. } => {
                // Choose compression based on network quality
                if network_quality.bandwidth_kbps < 500.0 {
                    // Use H.264 if not already compressed
                    if *compression_type == CompressionType::None {
                        let h264_data = Self::compress_to_h264(data)?;
                        *data = h264_data;
                        *compression_type = CompressionType::H264;
                    }
                } else if network_quality.bandwidth_kbps < 1000.0 {
                    // Use Zstd for moderate bandwidth
                    if *compression_type != CompressionType::Zstd {
                        let zstd_data = zstd::encode_all(data, 3)?;
                        *data = zstd_data;
                        *compression_type = CompressionType::Zstd;
                    }
                }
                // Otherwise leave as is
            }
            crate::stream::types::EventPayload::Ml(ml_event) => {
                // Compress ML events with Zstd
                let json = serde_json::to_vec(ml_event)?;
                let compressed = zstd::encode_all(&json[..], 3)?;
                event.payload = crate::stream::types::EventPayload::Ml(serde_json::from_slice(&compressed)?);
            }
            _ => {
                // Use Zstd for other event types
                let json = serde_json::to_vec(&event.payload)?;
                let compressed = zstd::encode_all(&json[..], 3)?;
                // Can't easily replace payload, so skip for now
            }
        }

        Ok(())
    }

    fn delta_encode_sensor(sensor_event: &crate::sensors::types::SensorEvent) -> Result<crate::sensors::types::SensorEvent, Box<dyn std::error::Error>> {
        // In production, implement delta encoding against previous values
        // For now, return as is
        Ok(sensor_event.clone())
    }

    fn compress_to_h264( &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // In production, use ffmpeg or v4l2 m2m
        // For now, return as is
        Ok(data.to_vec())
    }
}