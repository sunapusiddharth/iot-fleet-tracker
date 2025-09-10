use crate::camera::types::CameraFrame;
use crate::ml_edge::types::SensorContext;
use crate::sensors::types::{SensorEvent, SensorValues};
use chrono::Utc;
use std::collections::VecDeque;

pub struct SensorFusion {
    sensor_buffer: VecDeque<SensorEvent>,
    max_buffer_size: usize,
}

impl SensorFusion {
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            sensor_buffer: VecDeque::with_capacity(max_buffer_size),
            max_buffer_size,
        }
    }

    pub fn add_sensor_event(&mut self, event: SensorEvent) {
        if self.sensor_buffer.len() >= self.max_buffer_size {
            self.sensor_buffer.pop_front();
        }
        self.sensor_buffer.push_back(event);
    }

    pub fn get_context_for_frame(&self, frame: &CameraFrame) -> Option<SensorContext> {
        let frame_time = frame.timestamp;

        // Find sensor events closest to frame time
        let mut closest_gps = None;
        let mut closest_obd = None;
        let mut closest_imu = None;

        for event in &self.sensor_buffer {
            let event_time = event.timestamp;
            let time_diff = (event_time - frame_time).num_milliseconds().abs();

            if time_diff > 1000 {
                // 1 second max
                continue;
            }

            match &event.values {
                crate::sensors::types::SensorValues::Gps(gps) => {
                    if closest_gps.is_none()
                        || (event_time - frame_time).num_milliseconds().abs()
                            < (closest_gps.unwrap().0.timestamp - frame_time)
                                .num_milliseconds()
                                .abs()
                    {
                        closest_gps = Some((event.clone(), gps.clone()));
                    }
                }
                crate::sensors::types::SensorValues::Obd(obd) => {
                    if closest_obd.is_none()
                        || (event_time - frame_time).num_milliseconds().abs()
                            < (closest_obd.unwrap().0.timestamp - frame_time)
                                .num_milliseconds()
                                .abs()
                    {
                        closest_obd = Some((event.clone(), obd.clone()));
                    }
                }
                crate::sensors::types::SensorValues::Imu(imu) => {
                    if closest_imu.is_none()
                        || (event_time - frame_time).num_milliseconds().abs()
                            < (closest_imu.unwrap().0.timestamp - frame_time)
                                .num_milliseconds()
                                .abs()
                    {
                        closest_imu = Some((event.clone(), imu.clone()));
                    }
                }
                _ => {}
            }
        }

        let gps = closest_gps.map(|(_, g)| g);
        let obd = closest_obd.map(|(_, o)| o);
        let imu = closest_imu.map(|(_, i)| i);

        if gps.is_some() || obd.is_some() || imu.is_some() {
            let time_of_day = if frame.timestamp.hour() >= 6 && frame.timestamp.hour() < 18 {
                "day".to_string()
            } else if frame.timestamp.hour() >= 18 && frame.timestamp.hour() < 22 {
                "dusk".to_string()
            } else {
                "night".to_string()
            };

            Some(SensorContext {
                speed_kmh: obd.as_ref().map(|o| o.speed_kmh as f32).unwrap_or(0.0),
                acceleration: imu
                    .as_ref()
                    .map(|i| (i.accel_x.powi(2) + i.accel_y.powi(2) + i.accel_z.powi(2)).sqrt())
                    .unwrap_or(0.0),
                steering_angle: 0.0, // Not available yet
                gps_lat: gps.as_ref().map(|g| g.latitude).unwrap_or(0.0),
                gps_lon: gps.as_ref().map(|g| g.longitude).unwrap_or(0.0),
                time_of_day,
            })
        } else {
            None
        }
    }
}
