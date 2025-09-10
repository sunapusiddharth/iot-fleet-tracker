import { v4 as uuidv4 } from 'uuid';

export enum AlertType {
  DrowsyDriver = 'DrowsyDriver',
  LaneDeparture = 'LaneDeparture',
  CargoTamper = 'CargoTamper',
  LicensePlateMatch = 'LicensePlateMatch',
  WeatherHazard = 'WeatherHazard',
  HighTemperature = 'HighTemperature',
  LowDiskSpace = 'LowDiskSpace',
  HighCpuUsage = 'HighCpuUsage',
  NetworkFailure = 'NetworkFailure',
  SensorFailure = 'SensorFailure',
  HarshBraking = 'HarshBraking',
  RapidAcceleration = 'RapidAcceleration',
  SeatbeltNotFastened = 'SeatbeltNotFastened',
  DoorOpenWhileMoving = 'DoorOpenWhileMoving',
  OverSpeeding = 'OverSpeeding',
  UpdateAvailable = 'UpdateAvailable',
  UpdateFailed = 'UpdateFailed',
  RollbackTriggered = 'RollbackTriggered',
  ConfigError = 'ConfigError',
}

export enum AlertSeverity {
  Info = 'Info',
  Warning = 'Warning',
  Critical = 'Critical',
  Emergency = 'Emergency',
}

export enum ActionType {
  TriggerBuzzer = 'TriggerBuzzer',
  FlashLed = 'FlashLed',
  ShowOnDisplay = 'ShowOnDisplay',
  SendCanMessage = 'SendCanMessage',
  ActivateRelay = 'ActivateRelay',
  LogToServer = 'LogToServer',
  SendSms = 'SendSms',
}

export enum AlertStatus {
  Triggered = 'Triggered',
  Acknowledged = 'Acknowledged',
  Resolved = 'Resolved',
  Suppressed = 'Suppressed',
}

export interface Alert {
  id: string;
  alert_id: string;
  truck_id: string;
  alert_type: AlertType;
  severity: AlertSeverity;
  message: string;
  triggered_at: string;
  acknowledged_at: string | null;
  resolved_at: string | null;
  source: string;
  context: any; // JSON object with alert-specific context
  actions: AlertAction[];
  status: AlertStatus;
  created_at: string;
  updated_at: string;
}

export interface AlertAction {
  action_id: string;
  action_type: ActionType;
  target: string;
  parameters: any; // JSON object with action-specific parameters
  executed_at: string | null;
  success: boolean;
  error: string | null;
}

export interface AlertSummary {
  id: string;
  alert_id: string;
  truck_id: string;
  alert_type: AlertType;
  severity: AlertSeverity;
  message: string;
  triggered_at: string;
  status: AlertStatus;
  truck_license_plate: string;
  truck_model: string;
  truck_make: string;
}

export interface AlertStats {
  total_alerts: number;
  active_alerts: number;
  acknowledged_alerts: number;
  resolved_alerts: number;
  by_severity: Record<string, number>;
  by_type: Record<string, number>;
  by_truck: Record<string, number>;
  last_24_hours: number;
  last_7_days: number;
  last_30_days: number;
}

export interface DashboardAlerts {
  recent_alerts: AlertSummary[];
  stats: AlertStats;
  top_trucks: [string, number][];
  alerts_by_type: Record<string, number>;
  alerts_by_severity: Record<string, number>;
  time_range_hours: number;
}