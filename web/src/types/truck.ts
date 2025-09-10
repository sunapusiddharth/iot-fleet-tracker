import { v4 as uuidv4 } from 'uuid';
import type { HealthStatus } from './health';
import type { MlEvent } from './ml';
import type { SensorData, CameraData } from './telemetry';

export enum TruckStatus {
  Online = 'Online',
  Offline = 'Offline',
  Maintenance = 'Maintenance',
  Inactive = 'Inactive',
}

export interface Truck {
  id: string;
  truck_id: string;
  model: string;
  make: string;
  year: string;
  license_plate: string;
  vin: string;
  fleet_id: string | null;
  driver_id: string | null;
  status: TruckStatus;
  last_seen: string;
  location: [number, number]; // [longitude, latitude]
  created_at: string;
  updated_at: string;
}

export interface CreateTruckRequest {
  model: string;
  make: string;
  year: string;
  license_plate: string;
  vin: string;
  fleet_id?: string;
}

export interface UpdateTruckRequest {
  model?: string;
  make?: string;
  year?: string;
  license_plate?: string;
  vin?: string;
  fleet_id?: string;
  driver_id?: string;
  status?: TruckStatus;
}

export interface TruckSummary {
  id: string;
  truck_id: string;
  model: string;
  make: string;
  license_plate: string;
  status: TruckStatus;
  last_seen: string;
  location: [number, number];
  speed_kmh?: number;
  heading?: number;
  active_alerts: number;
  health_score: number;
}

export interface TripSummary {
  id: string;
  start_time: string;
  end_time: string;
  start_location: [number, number];
  end_location: [number, number];
  distance_km: number;
  duration_minutes: number;
  average_speed_kmh: number;
  max_speed_kmh: number;
  fuel_consumed_liters: number;
  events_count: number;
  alerts_count: number;
}

export interface MaintenanceRecord {
  id: string;
  maintenance_type: string;
  description: string;
  performed_at: string;
  next_due_date: string | null;
  cost: number;
  performed_by: string;
  mileage: number;
}

export interface TruckDetail {
  summary: TruckSummary;
  sensors: SensorData | null;
  cameras: CameraData | null;
  ml_events: MlEvent[];
  health_status: HealthStatus | null;
  recent_trips: TripSummary[];
  maintenance_history: MaintenanceRecord[];
}