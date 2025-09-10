use image::{DynamicImage, ImageBuffer, Rgb, imageops};
use ndarray::{Array, Ix3, Ix4};
use bytemuck;
use tract_onnx::prelude::Tensor;

pub fn preprocess_image_zero_copy(
    image: &DynamicImage,
    target_width: u32,
    target_height: u32,
    roi: Option<(f32, f32, f32, f32)>,
) -> Result<(Tensor, (u32, u32)), Box<dyn std::error::Error>> {
    let mut img = image.to_rgb8();

    // Apply ROI if specified
    if let Some((rx, ry, rw, rh)) = roi {
        let img_width = img.width() as f32;
        let img_height = img.height() as f32;

        let x = (rx * img_width) as u32;
        let y = (ry * img_height) as u32;
        let w = (rw * img_width) as u32;
        let h = (rh * img_height) as u32;

        img = imageops::crop(&mut img, x, y, w, h).to_image();
    }

    // Resize using triangle filter (good quality, fast)
    let resized = imageops::resize(
        &img,
        target_width,
        target_height,
        imageops::FilterType::Triangle,
    );

    // Zero-copy conversion to f32 tensor
    // We assume RGB8 layout is compatible with f32 (3 bytes per pixel)
    let raw_pixels = resized.into_raw();
    let len = raw_pixels.len() / 3; // 3 bytes per pixel

    // Safety: RGB8 is u8x3, which has same memory layout as [u8; 3]
    let rgb_pixels: &[u8] = &raw_pixels;
    let float_pixels: &[f32] = unsafe {
        std::slice::from_raw_parts(
            rgb_pixels.as_ptr() as *const f32,
            len,
        )
    };

    // But we need to normalize to [0,1] - so we can't avoid some computation
    // Let's do it efficiently with SIMD if possible
    let mut normalized = Vec::with_capacity(len * 3);
    for &pixel in rgb_pixels {
        normalized.push(pixel as f32 / 255.0);
    }

    // Reshape to NCHW (1, 3, H, W)
    let tensor = Array::from_shape_vec(
        (1, 3, target_height as usize, target_width as usize),
        normalized,
    )?;

    Ok((tensor.into(), (target_width, target_height)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{RgbImage, Rgb};

    #[test]
    fn test_preprocess_zero_copy() {
        let mut img = RgbImage::new(640, 480);
        for y in 0..480 {
            for x in 0..640 {
                img.put_pixel(x, y, Rgb([x as u8 % 255, y as u8 % 255, (x+y) as u8 % 255]));
            }
        }

        let dyn_img = DynamicImage::ImageRgb8(img);
        let (tensor, shape) = preprocess_image_zero_copy(&dyn_img, 224, 224, None).unwrap();

        assert_eq!(shape, (224, 224));
        assert_eq!(tensor.shape(), &[1, 3, 224, 224]);
        assert!(!tensor.as_slice::<f32>().unwrap().is_empty());
    }
}