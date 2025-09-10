use crate::camera::types::{CameraFrame, ImageFormat, FrameMetadata};
use image::{ImageBuffer, Rgb, RgbImage};
use jpeg_encoder::ColorType;
use tracing::{error, warn};

pub struct FrameEncoder;

impl FrameEncoder {
    // Encode raw RGB buffer to JPEG
    pub fn encode_rgb_to_jpeg(
        rgb_data: &[u8],
        width: u32,
        height: u32,
        quality: u8,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let mut encoder = jpeg_encoder::Encoder::new(&mut buffer, quality);

        // Validate size
        if rgb_data.len() != (width * height * 3) as usize {
            return Err("RGB data size mismatch".into());
        }

        encoder.encode(
            rgb_data,
            width as u16,
            height as u16,
            ColorType::Rgb,
        )?;

        Ok(buffer)
    }

    // Downscale + encode (to save bandwidth)
    pub fn resize_and_encode(
        rgb_ &[u8],
        src_width: u32,
        src_height: u32,
        dst_width: u32,
        dst_height: u32,
        quality: u8,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if rgb_data.len() != (src_width * src_height * 3) as usize {
            return Err("RGB data size mismatch".into());
        }

        let img: RgbImage = ImageBuffer::from_raw(src_width, src_height, rgb_data.to_vec())
            .ok_or("Failed to create image buffer")?;

        let resized_img = image::imageops::resize(
            &img,
            dst_width,
            dst_height,
            image::imageops::FilterType::Triangle,
        );

        let mut buffer = Vec::new();
        let mut encoder = jpeg_encoder::Encoder::new(&mut buffer, quality);
        encoder.encode(
            resized_img.as_raw(),
            dst_width as u16,
            dst_height as u16,
            ColorType::Rgb,
        )?;

        Ok(buffer)
    }

    // Future: H.264 via ffmpeg or v4l2 m2m
    pub fn encode_to_h264(
        _frames: Vec<&[u8]>,
        _width: u32,
        _height: u32,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Err("H.264 encoding not implemented yet".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jpeg_encode() {
        let width = 640;
        let height = 480;
        let size = (width * height * 3) as usize;
        let rgb_data = vec![128u8; size]; // gray image

        let jpeg = FrameEncoder::encode_rgb_to_jpeg(&rgb_data, width, height, 85).unwrap();
        assert!(!jpeg.is_empty());
        assert!(jpeg.len() < size); // should be compressed
    }

    #[test]
    fn test_resize_encode() {
        let src_width = 1280;
        let src_height = 720;
        let dst_width = 640;
        let dst_height = 360;
        let size = (src_width * src_height * 3) as usize;
        let rgb_data = vec![128u8; size];

        let jpeg = FrameEncoder::resize_and_encode(
            &rgb_data,
            src_width,
            src_height,
            dst_width,
            dst_height,
            85,
        )
        .unwrap();

        assert!(!jpeg.is_empty());
        assert!(jpeg.len() < size / 4); // should be much smaller
    }
}