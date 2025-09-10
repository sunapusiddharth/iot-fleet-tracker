export interface Report {
  id: string;
  name: string;
  description: string;
  type: ReportType;
  schedule: ReportSchedule | null;
  last_run: string | null;
  next_run: string | null;
  status: ReportStatus;
  config: ReportConfig;
  created_at: string;
  updated_at: string;
}

export enum ReportType {
  FleetOverview = 'fleet_overview',
  TruckDetails = 'truck_details',
  AlertSummary = 'alert_summary',
  MlEventAnalysis = 'ml_event_analysis',
  HealthStatusReport = 'health_status_report',
  MaintenanceReport = 'maintenance_report',
  DriverBehavior = 'driver_behavior',
  TripAnalysis = 'trip_analysis',
  FuelConsumption = 'fuel_consumption',
  CostAnalysis = 'cost_analysis',
  Compliance = 'compliance',
}

export interface ReportSchedule {
  frequency: 'daily' | 'weekly' | 'monthly' | 'quarterly' | 'yearly';
  day_of_week: number | null; // 0-6 (Sunday-Saturday)
  day_of_month: number | null; // 1-31
  month: number | null; // 1-12
  time: string; // HH:MM format
  timezone: string;
}

export enum ReportStatus {
  Active = 'active',
  Inactive = 'inactive',
  Running = 'running',
  Completed = 'completed',
  Failed = 'failed',
}

export interface ReportConfig {
  filters: Record<string, any>;
  format: 'pdf' | 'csv' | 'xlsx' | 'json';
  recipients: string[];
  include_charts: boolean;
  include_tables: boolean;
  include_maps: boolean;
  custom_css: string | null;
}

export interface GeneratedReport {
  id: string;
  report_id: string;
  generated_at: string;
  generated_by: string;
  file_path: string;
  file_size_bytes: number;
  format: 'pdf' | 'csv' | 'xlsx' | 'json';
  status: 'generated' | 'delivered' | 'failed';
  error_message: string | null;
  delivery_method: 'email' | 'download' | 'ftp' | 's3';
  delivery_status: string;
}

export interface ReportTemplate {
  id: string;
  name: string;
  description: string;
  type: ReportType;
  template: string; // Template content or reference
  variables: string[];
  created_at: string;
  updated_at: string;
}

export interface ScheduledReport {
  id: string;
  report_id: string;
  scheduled_at: string;
  status: 'scheduled' | 'running' | 'completed' | 'failed';
  run_at: string | null;
  completed_at: string | null;
  error_message: string | null;
  generated_report_id: string | null;
}

export interface ComplianceReport {
  id: string;
  report_id: string;
  compliance_standard: string; // e.g., FMCSA, DOT, ISO
  period_start: string;
  period_end: string;
  findings: ComplianceFinding[];
  recommendations: string[];
  status: 'draft' | 'submitted' | 'approved' | 'rejected';
  submitted_at: string | null;
  approved_at: string | null;
  approved_by: string | null;
}

export interface ComplianceFinding {
  id: string;
  category: string;
  description: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  evidence: string[];
  status: 'open' | 'in_progress' | 'resolved' | 'waived';
  assigned_to: string | null;
  due_date: string | null;
  resolution_notes: string | null;
  resolved_at: string | null;
}

export interface CostAnalysis {
  id: string;
  period_start: string;
  period_end: string;
  total_cost: number;
  breakdown: CostBreakdown;
  comparisons: CostComparison[];
  trends: CostTrend[];
  recommendations: string[];
}

export interface CostBreakdown {
  fuel: number;
  maintenance: number;
  insurance: number;
  depreciation: number;
  tires: number;
  labor: number;
  parts: number;
  fines: number;
  tolls: number;
  other: number;
}

export interface CostComparison {
  period: string;
  total_cost: number;
  difference_percent: number;
  improvement: boolean;
}

export interface CostTrend {
  month: string;
  total_cost: number;
  fuel_cost: number;
  maintenance_cost: number;
}

export interface DriverScorecard {
  driver_id: string;
  driver_name: string;
  period_start: string;
  period_end: string;
  score: number; // 0-100
  rank: number;
  total_miles: number;
  total_hours: number;
  avg_speed: number;
  max_speed: number;
  harsh_braking_events: number;
  rapid_acceleration_events: number;
  overspeeding_events: number;
  drowsy_events: number;
  lane_departure_events: number;
  seatbelt_violations: number;
  phone_usage_events: number;
  fuel_efficiency: number; // miles per gallon
  idle_time_hours: number;
  night_driving_hours: number;
  violations: Violation[];
  improvements: string[];
}

export interface Violation {
  type: string;
  timestamp: string;
  location: [number, number];
  speed_kmh: number;
  severity: 'minor' | 'moderate' | 'severe';
  resolved: boolean;
  notes: string;
}

export interface MaintenanceSchedule {
  id: string;
  truck_id: string;
  maintenance_type: string;
  description: string;
  due_date: string;
  due_mileage: number;
  priority: 'low' | 'medium' | 'high' | 'critical';
  status: 'scheduled' | 'in_progress' | 'completed' | 'cancelled';
  assigned_to: string | null;
  cost_estimate: number;
  actual_cost: number;
  parts_used: Part[];
  labor_hours: number;
  completed_at: string | null;
  completed_by: string | null;
  notes: string;
}

export interface Part {
  id: string;
  name: string;
  part_number: string;
  manufacturer: string;
  quantity: number;
  unit_price: number;
  total_price: number;
  supplier: string;
  warranty_months: number;
}

export interface FuelReport {
  id: string;
  period_start: string;
  period_end: string;
  total_fuel_consumed_liters: number;
  total_distance_km: number;
  avg_fuel_efficiency_km_per_liter: number;
  fuel_cost_total: number;
  avg_fuel_price_per_liter: number;
  by_truck: TruckFuelData[];
  by_driver: DriverFuelData[];
  by_route: RouteFuelData[];
  trends: FuelTrend[];
  anomalies: FuelAnomaly[];
}

export interface TruckFuelData {
  truck_id: string;
  truck_license_plate: string;
  total_fuel_consumed_liters: number;
  total_distance_km: number;
  avg_fuel_efficiency_km_per_liter: number;
  fuel_cost_total: number;
  avg_fuel_price_per_liter: number;
}

export interface DriverFuelData {
  driver_id: string;
  driver_name: string;
  total_fuel_consumed_liters: number;
  total_distance_km: number;
  avg_fuel_efficiency_km_per_liter: number;
  fuel_cost_total: number;
  avg_fuel_price_per_liter: number;
}

export interface RouteFuelData {
  route_id: string;
  route_name: string;
  total_fuel_consumed_liters: number;
  total_distance_km: number;
  avg_fuel_efficiency_km_per_liter: number;
  fuel_cost_total: number;
  avg_fuel_price_per_liter: number;
}

export interface FuelTrend {
  month: string;
  total_fuel_consumed_liters: number;
  total_distance_km: number;
  avg_fuel_efficiency_km_per_liter: number;
  fuel_cost_total: number;
  avg_fuel_price_per_liter: number;
}

export interface FuelAnomaly {
  id: string;
  truck_id: string;
  timestamp: string;
  location: [number, number];
  expected_fuel_consumption_liters: number;
  actual_fuel_consumption_liters: number;
  variance_percent: number;
  severity: 'low' | 'medium' | 'high' | 'critical';
  notes: string;
}