import { v4 as uuidv4 } from 'uuid';

export enum HealthStatusType {
  Ok ,
  Warning ,
  Critical ,
  Degraded ,
  ShutdownPending ,
}

export enum AlertSeverity {
  Info ,
  Warning ,
  Critical,
}

export enum ActionType {
  ThrottleCameraFps ,
  DisableMlModel ,
  ReduceSensorRate ,
  RotateWalEarly ,
  DropCameraFrames ,
  ReduceLogLevel ,
  RebootSystem 
}

export interface HealthStatus {
  id: string;
  truck_id: string;
  timestamp: string;
  status: HealthStatusType;
  resources: ResourceUsage;
  tasks: TaskStatus[];
  alerts: HealthAlert[];
  actions_taken: HealthAction[];
  meta :HealthEventMetadata;
  created_at: string;
}

export interface ResourceUsage {
  cpu_percent: number;
  cpu_cores: number;
  memory_percent: number;
  memory_used_mb: number;
  memory_total_mb: number;
  memory_available_mb: number;
  swap_percent: number;
  disk_percent: number;
  disk_used_gb: number;
  disk_total_gb: number;
  disk_available_gb: number;
  temperature_c: number;
  thermal_throttling: boolean;
  uptime_sec: number;
  load_average: [number, number, number];
}

export interface TaskStatus {
  name: string;
  is_alive: boolean;
  last_seen_ms: number;
  cpu_usage_percent: number;
  memory_usage_mb: number;
  restarts: number;
  last_restart: string | null;
}

export interface HealthAlert {
  alert_id: string;
  alert_type: string;
  severity: AlertSeverity;
  message: string;
  triggered_at: string;
  source: string;
  recommended_action: string;
}

export interface HealthAction {
  action_id: string;
  action_type: ActionType;
  target_module: string;
  parameters: any; // JSON object with action-specific parameters
  executed_at: string;
  success: boolean;
  message: string;
}

export interface HealthEventMetadata {
  device_id: string;
  version: string;
  hostname: string;
  ip_address: string;
  mac_address: string;
  location: [number, number] | null;
  hardware_model: string;
}

export interface HealthSummary {
  truck_id: string;
  last_timestamp: string;
  status: HealthStatusType;
  cpu_percent: number;
  memory_percent: number;
  disk_percent: number;
  temperature_c: number;
  uptime_sec: number;
  active_alerts: number;
  health_score: number;
}

export interface HealthStats {
  total_health_events: number;
  by_status: Record<string, number>;
  by_truck: Record<string, number>;
  avg_cpu_percent: number;
  avg_memory_percent: number;
  avg_disk_percent: number;
  avg_temperature_c: number;
  last_24_hours: number;
  last_7_days: number;
  last_30_days: number;
}

export interface DashboardHealthStatus {
  recent_health_status: HealthSummary[];
  stats: HealthStats;
  status_by_type: Record<string, number>;
  top_trucks: [string, number][];
  avg_resources: AverageResources;
  time_range_hours: number;
}

export interface AverageResources {
  avg_cpu_percent: number;
  avg_memory_percent: number;
  avg_disk_percent: number;
  avg_temperature_c: number;
}