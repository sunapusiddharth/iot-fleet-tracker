import { v4 as uuidv4 } from 'uuid';

export enum HardwareType {
  Cpu = 'Cpu',
  Cuda = 'Cuda',
  OpenVino = 'OpenVino',
  Fallback = 'Fallback',
}

export enum WeatherType {
  Clear = 'Clear',
  Rain = 'Rain',
  Fog = 'Fog',
  Snow = 'Snow',
  Night = 'Night',
}

export interface MlEvent {
  id: string;
  event_id: string;
  truck_id: string;
  model_name: string;
  model_version: string;
  timestamp: string;
  result: MlResult;
  confidence: number;
  calibrated_confidence: number;
  latency_ms: number;
  hardware_used: HardwareType;
  meta :MlEventMetadata;
  created_at: string;
}

export type MlResult = 
  | { type: 'Drowsiness', is_drowsy: boolean, eye_closure_ratio: number }
  | { type: 'LaneDeparture', is_departing: boolean, deviation_pixels: number }
  | { type: 'CargoTamper', is_tampered: boolean, motion_score: number }
  | { type: 'LicensePlate', plate_text: string, bounding_box: [number, number, number, number] }
  | { type: 'Weather', weather_type: WeatherType, visibility_m: number }
  | { type: 'Unknown' };

export interface MlEventMetadata {
  device_id: string;
  truck_id: string;
  route_id: string;
  driver_id: string;
  camera_id: string;
  frame_timestamp: string;
  sensor_context: SensorContext | null;
  cpu_usage_percent: number;
  gpu_usage_percent: number;
  memory_used_bytes: number;
  temperature_c: number;
  model_checksum: string;
  retry_count: number;
  fallback_reason: string | null;
}

export interface SensorContext {
  speed_kmh: number;
  acceleration: number;
  steering_angle: number;
  gps_lat: number;
  gps_lon: number;
  time_of_day: string;
}

export interface MlEventSummary {
  id: string;
  event_id: string;
  truck_id: string;
  model_name: string;
  result_type: string;
  confidence: number;
  timestamp: string;
  is_alert: boolean;
  truck_license_plate: string;
  truck_model: string;
  truck_make: string;
}

export interface MlStats {
  total_events: number;
  alert_events: number;
  by_model: Record<string, number>;
  by_result: Record<string, number>;
  by_truck: Record<string, number>;
  avg_confidence: number;
  avg_latency_ms: number;
  last_24_hours: number;
  last_7_days: number;
  last_30_days: number;
}

export interface DashboardMlEvents {
  recent_ml_events: MlEventSummary[];
  stats: MlStats;
  top_models: [string, number][];
  events_by_result: Record<string, number>;
  top_trucks: [string, number][];
  time_range_hours: number;
}