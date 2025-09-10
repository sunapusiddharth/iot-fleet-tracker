use crate::simulator::types::{CameraFrame, FrameMetadata};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CameraGenerator {
    truck_id: String,
    rng: rand::rngs::ThreadRng,
}

impl CameraGenerator {
    pub fn new(truck_id: &str) -> Self {
        Self {
            truck_id: truck_id.to_string(),
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_driver_camera_frame(&mut self, scenario: &str) -> CameraFrame {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        
        // Generate a simple image - in production, use real images or more complex generation
        let width = 640;
        let height = 480;
        let mut img = RgbImage::new(width, height);
        
        // Fill with different colors based on scenario
        let color = match scenario {
            "drowsy_driver" => Rgb([255, 0, 0]), // Red for drowsy
            "distracted_driver" => Rgb([255, 255, 0]), // Yellow for distracted
            "normal_driving" => Rgb([0, 255, 0]), // Green for normal
            _ => Rgb([128, 128, 128]), // Gray for unknown
        };
        
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, color);
            }
        }
        
        // Add some noise
        for _ in 0..1000 {
            let x = self.rng.gen_range(0..width);
            let y = self.rng.gen_range(0..height);
            let noise_color = Rgb([
                self.rng.gen_range(0..255),
                self.rng.gen_range(0..255),
                self.rng.gen_range(0..255),
            ]);
            img.put_pixel(x, y, noise_color);
        }
        
        // Convert to bytes
         img.into_raw();
        
        CameraFrame {
            timestamp,
            width,
            height,
            format: "rgb".to_string(),
            data,
            is_keyframe: true,
            meta FrameMetadata {
                exposure_us: Some(10000),
                gain_db: Some(0.0),
                temperature_c: Some(25.0),
                gps_lat: None,
                gps_lon: None,
                speed_kmh: None,
            },
        }
    }
    
    pub fn generate_front_camera_frame(&mut self, scenario: &str) -> CameraFrame {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        
        let width = 1280;
        let height = 720;
        let mut img = RgbImage::new(width, height);
        
        // Generate road scene
        let road_color = Rgb([100, 100, 100]);
        let lane_color = Rgb([255, 255, 255]);
        let sky_color = Rgb([135, 206, 235]);
        
        // Fill sky (top 1/3)
        for y in 0..height/3 {
            for x in 0..width {
                img.put_pixel(x, y, sky_color);
            }
        }
        
        // Fill road (bottom 2/3)
        for y in height/3..height {
            for x in 0..width {
                img.put_pixel(x, y, road_color);
            }
        }
        
        // Add lane markers
        for y in (height/3..height).step_by(50) {
            for x in (width/2 - 5)..=(width/2 + 5) {
                if x < width {
                    img.put_pixel(x, y, lane_color);
                }
            }
        }
        
        // Modify based on scenario
        match scenario {
            "lane_departure" => {
                // Shift lane markers to simulate departure
                for y in (height/3..height).step_by(50) {
                    for x in (width/2 - 5 + 100)..=(width/2 + 5 + 100) {
                        if x < width {
                            img.put_pixel(x, y, lane_color);
                        }
                    }
                }
            }
            "cargo_tamper" => {
                // Add "cargo" in the distance
                for y in (height/2..height/2 + 100) {
                    for x in (width/3..width/3 + 200) {
                        if x < width && y < height {
                            img.put_pixel(x, y, Rgb([200, 150, 100]));
                        }
                    }
                }
            }
            _ => {}
        }
        
        // Convert to bytes
         img.into_raw();
        
        CameraFrame {
            timestamp,
            width,
            height,
            format: "rgb".to_string(),
            data,
            is_keyframe: true,
            meta FrameMetadata {
                exposure_us: Some(5000),
                gain_db: Some(0.0),
                temperature_c: Some(25.0),
                gps_lat: None,
                gps_lon: None,
                speed_kmh: None,
            },
        }
    }
    
    pub fn generate_cargo_camera_frame(&mut self, scenario: &str) -> CameraFrame {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        
        let width = 800;
        let height = 600;
        let mut img = RgbImage::new(width, height);
        
        // Generate cargo area
        let cargo_color = Rgb([200, 150, 100]);
        let background_color = Rgb([50, 50, 50]);
        
        // Fill background
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, background_color);
            }
        }
        
        // Add cargo
        for y in (height/4..3*height/4) {
            for x in (width/4..3*width/4) {
                img.put_pixel(x, y, cargo_color);
            }
        }
        
        // Modify based on scenario
        match scenario {
            "cargo_tamper" => {
                // Add "tampering" - move some cargo
                for y in (height/4..height/2) {
                    for x in (width/4..width/2) {
                        img.put_pixel(x, y, Rgb([255, 0, 0])); // Red for tampering
                    }
                }
            }
            _ => {}
        }
        
        // Convert to bytes
         img.into_raw();
        
        CameraFrame {
            timestamp,
            width,
            height,
            format: "rgb".to_string(),
            data,
            is_keyframe: true,
            meta FrameMetadata {
                exposure_us: Some(8000),
                gain_db: Some(0.0),
                temperature_c: Some(25.0),
                gps_lat: None,
                gps_lon: None,
                speed_kmh: None,
            },
        }
    }
}