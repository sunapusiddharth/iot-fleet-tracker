use crate::simulator::types::{SensorData, GpsData, ObdData, ImuData, TpmsData};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SensorGenerator {
    truck_id: String,
    rng: rand::rngs::ThreadRng,
}

impl SensorGenerator {
    pub fn new(truck_id: &str) -> Self {
        Self {
            truck_id: truck_id.to_string(),
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_sensor_data(&mut self, latitude: f64, longitude: f64, speed_kmh: f32, scenario: &str) -> SensorData {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;

        // GPS data
        let gps = GpsData {
            latitude,
            longitude,
            altitude: 100.0 + self.rng.gen_range(-10.0..10.0),
            speed_kmh,
            heading: self.rng.gen_range(0.0..360.0),
            satellites: self.rng.gen_range(8..12),
            fix_quality: 1,
        };

        // OBD data - varies by scenario
        let (rpm, coolant_temp, fuel_level, engine_load, throttle_pos) = match scenario {
            "emergency_braking" => (2000, 95, 75, 80, 20), // High RPM, high temp during braking
            "rapid_acceleration" => (4000, 90, 75, 90, 90), // High everything during acceleration
            "system_failure" => (1000, 110, 75, 30, 10), // Overheating
            _ => {
                let base_rpm = if speed_kmh > 80.0 { 2500 } else { 1500 };
                (base_rpm + self.rng.gen_range(-500..500) as u16,
                 85 + self.rng.gen_range(-5..5) as i8,
                 75 + self.rng.gen_range(-5..5) as u8,
                 50 + self.rng.gen_range(-20..20) as u8,
                 30 + self.rng.gen_range(-20..20) as u8)
            }
        };

        let obd = ObdData {
            rpm,
            speed_kmh: speed_kmh as u8,
            coolant_temp,
            fuel_level,
            engine_load,
            throttle_pos,
        };

        // IMU data - accelerometer and gyroscope
        let (accel_x, accel_y, accel_z) = match scenario {
            "emergency_braking" => (-0.8, 0.1, 0.98), // Strong negative acceleration
            "rapid_acceleration" => (0.6, 0.1, 0.98), // Strong positive acceleration
            "sharp_turn" => (0.1, 0.5, 0.98), // Lateral acceleration
            _ => (0.05 + self.rng.gen_range(-0.1..0.1), 
                  0.05 + self.rng.gen_range(-0.1..0.1), 
                  0.98 + self.rng.gen_range(-0.05..0.05)),
        };

        let imu = ImuData {
            accel_x,
            accel_y,
            accel_z,
            gyro_x: self.rng.gen_range(-1.0..1.0),
            gyro_y: self.rng.gen_range(-1.0..1.0),
            gyro_z: self.rng.gen_range(-1.0..1.0),
        };

        // TPMS data - tire pressure monitoring
        let tpms = TpmsData {
            front_left: TireSensor {
                pressure_psi: 32.0 + self.rng.gen_range(-2.0..2.0),
                temperature_c: 25.0 + self.rng.gen_range(-5.0..15.0),
                battery_percent: 90 + self.rng.gen_range(-10..10) as u8,
                alert: false,
            },
            front_right: TireSensor {
                pressure_psi: 32.0 + self.rng.gen_range(-2.0..2.0),
                temperature_c: 25.0 + self.rng.gen_range(-5.0..15.0),
                battery_percent: 90 + self.rng.gen_range(-10..10) as u8,
                alert: false,
            },
            rear_left: TireSensor {
                pressure_psi: 32.0 + self.rng.gen_range(-2.0..2.0),
                temperature_c: 25.0 + self.rng.gen_range(-5.0..15.0),
                battery_percent: 90 + self.rng.gen_range(-10..10) as u8,
                alert: false,
            },
            rear_right: TireSensor {
                pressure_psi: 32.0 + self.rng.gen_range(-2.0..2.0),
                temperature_c: 25.0 + self.rng.gen_range(-5.0..15.0),
                battery_percent: 90 + self.rng.gen_range(-10..10) as u8,
                alert: false,
            },
        };

        SensorData {
            gps,
            obd,
            imu,
            tpms,
        }
    }
}