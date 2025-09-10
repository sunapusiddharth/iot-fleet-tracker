export interface User {
  id: string;
  username: string;
  email: string;
  name: string;
  role: UserRole;
  status: UserStatus;
  last_login: string | null;
  created_at: string;
  updated_at: string;
  preferences: UserPreferences;
}

export enum UserRole {
  Admin = 'admin',
  FleetManager = 'fleet_manager',
  TerminalOperator = 'terminal_operator',
  Driver = 'driver',
  Viewer = 'viewer',
}

export enum UserStatus {
  Active = 'active',
  Inactive = 'inactive',
  Suspended = 'suspended',
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'auto';
  language: string;
  timezone: string;
  notifications: NotificationPreferences;
  dashboard_layout: DashboardLayout;
  default_filters: DefaultFilters;
}

export interface NotificationPreferences {
  email_alerts: boolean;
  sms_alerts: boolean;
  push_notifications: boolean;
  alert_types: string[];
  quiet_hours_start: string; // HH:MM format
  quiet_hours_end: string; // HH:MM format
}

export interface DashboardLayout {
  widgets: Widget[];
  grid_columns: number;
  grid_rows: number;
}

export interface Widget {
  id: string;
  type: WidgetType;
  title: string;
  position: [number, number]; // [row, column]
  size: [number, number]; // [rows, columns]
  config: WidgetConfig;
}

export enum WidgetType {
  Stats = 'stats',
  Chart = 'chart',
  Map = 'map',
  Table = 'table',
  AlertList = 'alert_list',
  MlEventList = 'ml_event_list',
  HealthStatusList = 'health_status_list',
  TruckList = 'truck_list',
  TripList = 'trip_list',
  MaintenanceList = 'maintenance_list',
  OtaUpdateList = 'ota_update_list',
  RemoteCommandList = 'remote_command_list',
}

export type WidgetConfig = 
  | StatsWidgetConfig
  | ChartWidgetConfig
  | MapWidgetConfig
  | TableWidgetConfig
  | ListWidgetConfig;

export interface StatsWidgetConfig {
  metrics: string[];
  period: 'hour' | 'day' | 'week' | 'month';
  comparison_period: 'previous' | 'year' | 'none';
}

export interface ChartWidgetConfig {
  chart_type: 'line' | 'bar' | 'pie' | 'area';
  data_source: string;
  x_axis: string;
  y_axes: string[];
  period: 'hour' | 'day' | 'week' | 'month';
  group_by: string;
  filters: Record<string, any>;
}

export interface MapWidgetConfig {
  map_type: 'roadmap' | 'satellite' | 'hybrid';
  show_trucks: boolean;
  show_alerts: boolean;
  show_geofences: boolean;
  show_routes: boolean;
  default_zoom: number;
  default_center: [number, number];
  filters: Record<string, any>;
}

export interface TableWidgetConfig {
  data_source: string;
  columns: string[];
  sort_by: string;
  sort_order: 'asc' | 'desc';
  filters: Record<string, any>;
  pagination: boolean;
  page_size: number;
}

export interface ListWidgetConfig {
  data_source: string;
  item_fields: string[];
  sort_by: string;
  sort_order: 'asc' | 'desc';
  filters: Record<string, any>;
  limit: number;
}

export interface DefaultFilters {
  trucks: Record<string, any>;
  alerts: Record<string, any>;
  ml_events: Record<string, any>;
  health_status: Record<string, any>;
  ota_updates: Record<string, any>;
  remote_commands: Record<string, any>;
}

export interface Permission {
  id: string;
  name: string;
  description: string;
  resource: string;
  actions: string[];
}

export interface Role {
  id: string;
  name: string;
  description: string;
  permissions: Permission[];
}

export interface Session {
  id: string;
  user_id: string;
  token: string;
  ip_address: string;
  user_agent: string;
  created_at: string;
  expires_at: string;
  last_activity: string;
  is_active: boolean;
}

export interface AuditLog {
  id: string;
  user_id: string;
  action: string;
  resource: string;
  resource_id: string;
  ip_address: string;
  user_agent: string;
  timestamp: string;
  details: Record<string, any>;
}

export interface Notification {
  id: string;
  user_id: string;
  title: string;
  message: string;
  type: 'info' | 'warning' | 'error' | 'success';
  read: boolean;
  created_at: string;
  read_at: string | null;
  action_url: string | null;
}