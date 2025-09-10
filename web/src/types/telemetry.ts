import { v4 as uuidv4 } from 'uuid';

export interface TelemetryData {
  id: string;
  truck_id: string;
  timestamp: string;
  location: [number, number]; // [longitude, latitude]
  speed_kmh: number;
  heading: number;
  sensors: SensorData;
  cameras: CameraData | null;
  scenario: string | null;
  created_at: string;
}

export interface SensorData {
  gps: GpsData;
  obd: ObdData;
  imu: ImuData;
  tpms: TpmsData;
}

export interface GpsData {
  latitude: number;
  longitude: number;
  altitude: number;
  speed_kmh: number;
  heading: number;
  satellites: number;
  fix_quality: number;
}

export interface ObdData {
  rpm: number;
  speed_kmh: number;
  coolant_temp: number;
  fuel_level: number;
  engine_load: number;
  throttle_pos: number;
}

export interface ImuData {
  accel_x: number;
  accel_y: number;
  accel_z: number;
  gyro_x: number;
  gyro_y: number;
  gyro_z: number;
}

export interface TpmsData {
  front_left: TireSensor;
  front_right: TireSensor;
  rear_left: TireSensor;
  rear_right: TireSensor;
}

export interface TireSensor {
  pressure_psi: number;
  temperature_c: number;
  battery_percent: number;
  alert: boolean;
}

export interface CameraData {
  front_camera: CameraFrameRef | null;
  driver_camera: CameraFrameRef | null;
  cargo_camera: CameraFrameRef | null;
}

export interface CameraFrameRef {
  frame_id: string;
  timestamp: string;
  url: string;
  thumbnail_url: string | null;
  width: number;
  height: number;
  format: string;
  size_bytes: number;
  is_keyframe: boolean;
  meta :FrameMetadata;
}

export interface FrameMetadata {
  exposure_us: number | null;
  gain_db: number | null;
  temperature_c: number | null;
  gps_lat: number | null;
  gps_lon: number | null;
  speed_kmh: number | null;
}

export interface TelemetrySummary {
  truck_id: string;
  last_timestamp: string;
  last_location: [number, number];
  last_speed_kmh: number;
  last_heading: number;
  last_rpm: number;
  last_coolant_temp: number;
  last_fuel_level: number;
  last_accel_x: number;
  last_accel_y: number;
  last_accel_z: number;
  tire_pressures: [number, number, number, number];
  tire_temperatures: [number, number, number, number];
}

export interface TelemetryStatistics {
  truck_id: string;
  start_time: string;
  end_time: string;
  avg_speed_kmh: number;
  max_speed_kmh: number;
  min_speed_kmh: number;
  avg_rpm: number;
  max_rpm: number;
  min_rpm: number;
  avg_coolant_temp: number;
  max_coolant_temp: number;
  min_coolant_temp: number;
  avg_fuel_level: number;
  min_fuel_level: number;
  total_distance_km: number;
  total_fuel_consumed_liters: number;
  harsh_braking_events: number;
  rapid_acceleration_events: number;
  overspeeding_events: number;
}