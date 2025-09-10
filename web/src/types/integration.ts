export interface Integration {
  id: string;
  name: string;
  type: IntegrationType;
  status: IntegrationStatus;
  config: IntegrationConfig;
  last_sync: string | null;
  next_sync: string | null;
  error_message: string | null;
  created_at: string;
  updated_at: string;
}

export enum IntegrationType {
  Telematics = 'telematics',
  ELD = 'eld', // Electronic Logging Device
  GPS = 'gps',
  FuelCard = 'fuel_card',
  Maintenance = 'maintenance',
  Insurance = 'insurance',
  Weather = 'weather',
  Traffic = 'traffic',
  Geofencing = 'geofencing',
  VideoAnalytics = 'video_analytics',
  Dvir = 'dvir', // Driver Vehicle Inspection Report
  Ifta = 'ifta', // International Fuel Tax Agreement
  HOS = 'hos', // Hours of Service
}

export enum IntegrationStatus {
  Active = 'active',
  Inactive = 'inactive',
  Connecting = 'connecting',
  Connected = 'connected',
  Disconnected = 'disconnected',
  Error = 'error',
}

export interface IntegrationConfig {
  endpoint: string;
  username: string;
  password: string;
  api_key: string;
  polling_interval_minutes: number;
  sync_direction: 'inbound' | 'outbound' | 'bidirectional';
  enabled_features: string[];
  custom_fields: Record<string, any>;
}

export interface TelematicsIntegrationConfig extends IntegrationConfig {
  provider: 'geotab' | 'samsara' | 'fleetio' | 'keeptruckin' | 'custom';
  vehicle_mapping: Record<string, string>; // External ID -> Internal ID
  driver_mapping: Record<string, string>; // External ID -> Internal ID
  include_trailers: boolean;
  include_sensors: boolean;
  include_video: boolean;
}

export interface ELDIntegrationConfig extends IntegrationConfig {
  provider: 'keeptruckin' | 'samsara' | 'bigroad' | 'custom';
  hos_ruleset: 'us_fmcsa' | 'canada' | 'texas' | 'custom';
  auto_certify_logs: boolean;
  require_electronically_signed: boolean;
  allow_paper_logs: boolean;
}

export interface GPSIntegrationConfig extends IntegrationConfig {
  provider: 'garmin' | 'tomtom' | 'magellan' | 'custom';
  update_frequency_seconds: number;
  include_traffic: boolean;
  include_weather: boolean;
  include_points_of_interest: boolean;
}

export interface FuelCardIntegrationConfig extends IntegrationConfig {
  provider: 'wex' | 'fleetcor' | 'comdata' | 'custom';
  card_mapping: Record<string, string>; // External Card ID -> Internal Truck ID
  include_transactions: boolean;
  include_receipts: boolean;
  auto_reconcile: boolean;
}

export interface MaintenanceIntegrationConfig extends IntegrationConfig {
  provider: 'maintainx' | 'fiix' | 'custom';
  work_order_mapping: Record<string, string>;
  include_parts_inventory: boolean;
  include_labor_costs: boolean;
  include_vendor_info: boolean;
}

export interface InsuranceIntegrationConfig extends IntegrationConfig {
  provider: 'progressive' | 'geico' | 'statefarm' | 'custom';
  policy_mapping: Record<string, string>;
  include_claims: boolean;
  include_coverage_details: boolean;
  include_premium_info: boolean;
}

export interface WeatherIntegrationConfig extends IntegrationConfig {
  provider: 'accuweather' | 'openweather' | 'custom';
  api_key: string;
  update_frequency_minutes: number;
  include_forecasts: boolean;
  include_historical: boolean;
  include_alerts: boolean;
}

export interface TrafficIntegrationConfig extends IntegrationConfig {
  provider: 'google' | 'here' | 'tomtom' | 'custom';
  api_key: string;
  update_frequency_minutes: number;
  include_incidents: boolean;
  include_construction: boolean;
  include_cameras: boolean;
}

export interface GeofencingIntegrationConfig extends IntegrationConfig {
  provider: 'geotab' | 'samsara' | 'custom';
  sync_geofences: boolean;
  sync_events: boolean;
  include_entry_events: boolean;
  include_exit_events: boolean;
  include_inside_events: boolean;
}

export interface VideoAnalyticsIntegrationConfig extends IntegrationConfig {
  provider: 'lytx' | 'smartvue' | 'custom';
  include_driver_camera: boolean;
  include_road_camera: boolean;
  include_cargo_camera: boolean;
  include_ai_events: boolean;
  include_manual_reviews: boolean;
}

export interface DvirIntegrationConfig extends IntegrationConfig {
  provider: 'keeptruckin' | 'samsara' | 'custom';
  require_completion: boolean;
  require_certification: boolean;
  include_defects: boolean;
  include_repairs: boolean;
  include_inspector_signatures: boolean;
}

export interface IftaIntegrationConfig extends IntegrationConfig {
  provider: 'iftaonline' | 'custom';
  include_quarterly_reports: boolean;
  include_monthly_summaries: boolean;
  include_jurisdiction_breakdowns: boolean;
  auto_submit: boolean;
}

export interface HOSIntegrationConfig extends IntegrationConfig {
  provider: 'keeptruckin' | 'samsara' | 'custom';
  ruleset: 'us_fmcsa' | 'canada' | 'texas' | 'custom';
  include_driving_time: boolean;
  include_on_duty_time: boolean;
  include_off_duty_time: boolean;
  include_sleep_berth_time: boolean;
}

export interface SyncLog {
  id: string;
  integration_id: string;
  sync_type: 'full' | 'incremental' | 'manual';
  started_at: string;
  completed_at: string | null;
  status: 'started' | 'in_progress' | 'completed' | 'failed';
  records_processed: number;
  records_added: number;
  records_updated: number;
  records_deleted: number;
  error_message: string | null;
  details: Record<string, any>;
}

export interface ExternalEntityMapping {
  id: string;
  integration_id: string;
  entity_type: 'truck' | 'driver' | 'trailer' | 'location' | 'route';
  external_id: string;
  internal_id: string;
  last_synced: string;
  status: 'active' | 'inactive' | 'deleted';
  metadata: Record<string, any>;
}

export interface WebhookSubscription {
  id: string;
  integration_id: string;
  event_type: string;
  url: string;
  secret: string;
  active: boolean;
  created_at: string;
  updated_at: string;
}

export interface WebhookEvent {
  id: string;
  subscription_id: string;
  event_type: string;
  payload: Record<string, any>;
  delivered_at: string | null;
  delivery_status: 'pending' | 'delivered' | 'failed';
  response_code: number | null;
  response_body: string | null;
  retry_count: number;
  created_at: string;
}

export interface DataMapping {
  id: string;
  integration_id: string;
  source_field: string;
  target_field: string;
  transformation: string; // e.g., "uppercase", "lowercase", "multiply(1.60934)"
  condition: string | null; // e.g., "when(speed > 100)"
  default_value: any;
  required: boolean;
  created_at: string;
  updated_at: string;
}

export interface CustomField {
  id: string;
  integration_id: string;
  name: string;
  label: string;
  type: 'text' | 'number' | 'date' | 'boolean' | 'select' | 'multiselect';
  options: string[];
  required: boolean;
  searchable: boolean;
  display_in_ui: boolean;
  created_at: string;
  updated_at: string;
}

export interface IntegrationMetric {
  id: string;
  integration_id: string;
  metric_name: string;
  value: number;
  unit: string;
  timestamp: string;
  tags: Record<string, string>;
}

export interface IntegrationAlert {
  id: string;
  integration_id: string;
  alert_type: 'connection_lost' | 'sync_failed' | 'data_quality' | 'rate_limit' | 'authentication';
  severity: 'info' | 'warning' | 'critical';
  message: string;
  triggered_at: string;
  resolved_at: string | null;
  acknowledged_at: string | null;
  acknowledged_by: string | null;
  details: Record<string, any>;
}

export interface IntegrationAuditLog {
  id: string;
  integration_id: string;
  action: string;
  resource: string;
  resource_id: string;
  user_id: string;
  ip_address: string;
  user_agent: string;
  timestamp: string;
  details: Record<string, any>;
}