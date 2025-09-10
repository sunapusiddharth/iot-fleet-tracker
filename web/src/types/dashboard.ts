import { TruckSummary } from './truck';
import { AlertSummary } from './alert';
import { MlEventSummary } from './ml';
import { HealthSummary } from './health';
import { OtaSummary } from './ota';

export interface DashboardStats {
  total_trucks: number;
  online_trucks: number;
  offline_trucks: number;
  maintenance_trucks: number;
  inactive_trucks: number;
  total_alerts: number;
  active_alerts: number;
  acknowledged_alerts: number;
  resolved_alerts: number;
  critical_alerts: number;
  warning_alerts: number;
  info_alerts: number;
  total_ml_events: number;
  alert_ml_events: number;
  high_confidence_ml_events: number;
  total_health_events: number;
  critical_health_events: number;
  warning_health_events: number;
  ok_health_events: number;
  total_ota_updates: number;
  pending_ota_updates: number;
  in_progress_ota_updates: number;
  successful_ota_updates: number;
  failed_ota_updates: number;
  total_remote_commands: number;
  pending_remote_commands: number;
  executing_remote_commands: number;
  successful_remote_commands: number;
  failed_remote_commands: number;
  avg_truck_health_score: number;
  avg_truck_uptime_hours: number;
  total_trips_today: number;
  total_distance_today_km: number;
  total_fuel_consumed_today_liters: number;
  harsh_braking_events_today: number;
  rapid_acceleration_events_today: number;
  overspeeding_events_today: number;
}

export interface DashboardSummary {
  stats: DashboardStats;
  recent_alerts: AlertSummary[];
  recent_ml_events: MlEventSummary[];
  recent_health_status: HealthSummary[];
  recent_ota_updates: OtaSummary[];
  recent_trucks: TruckSummary[];
  alerts_by_severity: Record<string, number>;
  alerts_by_type: Record<string, number>;
  ml_events_by_model: Record<string, number>;
  health_status_by_type: Record<string, number>;
  ota_updates_by_target: Record<string, number>;
  trucks_by_status: Record<string, number>;
  top_alert_trucks: [string, number][];
  top_ml_trucks: [string, number][];
  top_health_trucks: [string, number][];
  time_range_hours: number;
}

export interface FleetOverview {
  total_trucks: number;
  trucks_online: number;
  trucks_offline: number;
  trucks_maintenance: number;
  trucks_inactive: number;
  active_alerts: number;
  critical_alerts: number;
  warning_alerts: number;
  recent_trips: number;
  total_distance_km: number;
  avg_fuel_efficiency_km_per_liter: number;
  total_fuel_consumed_liters: number;
  avg_speed_kmh: number;
  max_speed_kmh: number;
  harsh_braking_events: number;
  rapid_acceleration_events: number;
  overspeeding_events: number;
  drowsy_driver_events: number;
  lane_departure_events: number;
  cargo_tamper_events: number;
  avg_health_score: number;
  trucks_needing_maintenance: number;
  pending_updates: number;
  failed_updates: number;
}

export interface TruckGroup {
  id: string;
  name: string;
  description: string;
  truck_count: number;
  trucks: TruckSummary[];
  stats: GroupStats;
}

export interface GroupStats {
  total_trucks: number;
  online_trucks: number;
  offline_trucks: number;
  maintenance_trucks: number;
  inactive_trucks: number;
  active_alerts: number;
  critical_alerts: number;
  warning_alerts: number;
  avg_health_score: number;
  avg_uptime_hours: number;
  total_trips: number;
  total_distance_km: number;
  total_fuel_consumed_liters: number;
  harsh_braking_events: number;
  rapid_acceleration_events: number;
  overspeeding_events: number;
  drowsy_driver_events: number;
  lane_departure_events: number;
  cargo_tamper_events: number;
}

export interface AlertTrend {
  timestamp: string;
  total_alerts: number;
  critical_alerts: number;
  warning_alerts: number;
  info_alerts: number;
  resolved_alerts: number;
}

export interface MlEventTrend {
  timestamp: string;
  total_events: number;
  alert_events: number;
  high_confidence_events: number;
  avg_confidence: number;
}

export interface HealthTrend {
  timestamp: string;
  total_events: number;
  critical_events: number;
  warning_events: number;
  ok_events: number;
  avg_cpu_percent: number;
  avg_memory_percent: number;
  avg_disk_percent: number;
  avg_temperature_c: number;
}

export interface TripTrend {
  timestamp: string;
  total_trips: number;
  total_distance_km: number;
  avg_trip_distance_km: number;
  avg_trip_duration_minutes: number;
  total_fuel_consumed_liters: number;
  avg_fuel_efficiency_km_per_liter: number;
}

export interface ResourceUsageTrend {
  timestamp: string;
  avg_cpu_percent: number;
  avg_memory_percent: number;
  avg_disk_percent: number;
  avg_temperature_c: number;
  max_cpu_percent: number;
  max_memory_percent: number;
  max_disk_percent: number;
  max_temperature_c: number;
}

export interface PerformanceMetrics {
  avg_response_time_ms: number;
  max_response_time_ms: number;
  avg_processing_time_ms: number;
  max_processing_time_ms: number;
  total_requests: number;
  successful_requests: number;
  failed_requests: number;
  error_rate_percent: number;
  uptime_percent: number;
  last_downtime: string | null;
  total_downtime_minutes: number;
}

export interface SystemHealth {
  overall_status: 'healthy' | 'degraded' | 'unhealthy';
  components: ComponentHealth[];
  metrics: PerformanceMetrics;
  last_checked: string;
}

export interface ComponentHealth {
  name: string;
  status: 'healthy' | 'degraded' | 'unhealthy';
  message: string;
  last_checked: string;
  metrics: Record<string, number>;
}

export interface Geofence {
  id: string;
  name: string;
  description: string;
  coordinates: [number, number][]; // Polygon coordinates
  radius_meters: number;
  type: 'polygon' | 'circle';
  color: string;
  alert_on_enter: boolean;
  alert_on_exit: boolean;
  active: boolean;
}

export interface GeofenceEvent {
  id: string;
  truck_id: string;
  geofence_id: string;
  event_type: 'enter' | 'exit' | 'inside' | 'outside';
  timestamp: string;
  location: [number, number];
  speed_kmh: number;
  heading: number;
  truck_license_plate: string;
  truck_model: string;
  truck_make: string;
  geofence_name: string;
}

export interface DriverBehavior {
  driver_id: string;
  driver_name: string;
  truck_id: string;
  total_trips: number;
  total_distance_km: number;
  total_driving_hours: number;
  avg_speed_kmh: number;
  max_speed_kmh: number;
  harsh_braking_events: number;
  rapid_acceleration_events: number;
  overspeeding_events: number;
  drowsy_driver_events: number;
  seatbelt_violations: number;
  phone_usage_events: number;
  lane_departure_events: number;
  score: number; // 0-100 behavioral score
  ranking: number; // Position among all drivers
  last_trip_date: string;
  last_trip_distance_km: number;
  last_trip_duration_minutes: number;
  last_trip_fuel_consumed_liters: number;
  last_trip_harsh_events: number;
  last_trip_drowsy_events: number;
  last_trip_lane_departures: number;
}