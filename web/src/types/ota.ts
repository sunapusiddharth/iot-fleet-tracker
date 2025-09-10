import { v4 as uuidv4 } from 'uuid';

export enum UpdateTarget {
  Agent = 'Agent',
  Model = 'Model',
  Config = 'Config',
  Firmware = 'Firmware',
}

export enum UpdatePriority {
  Critical = 'Critical',
  High = 'High',
  Medium = 'Medium',
  Low = 'Low',
}

export enum OtaStatus {
  Pending = 'Pending',
  Downloading = 'Downloading',
  Verifying = 'Verifying',
  Applying = 'Applying',
  Rollback = 'Rollback',
  Success = 'Success',
  Failed = 'Failed',
}

export enum CommandType {
  Reboot = 'Reboot',
  Shutdown = 'Shutdown',
  RestartService = 'RestartService',
  GetDiagnostics = 'GetDiagnostics',
  UpdateConfig = 'UpdateConfig',
  RunHealthCheck = 'RunHealthCheck',
  CaptureSnapshot = 'CaptureSnapshot',
  FlushWAL = 'FlushWAL',
}

export enum CommandStatus {
  Pending = 'Pending',
  Executing = 'Executing',
  Success = 'Success',
  Failed = 'Failed',
  Timeout = 'Timeout',
  Cancelled = 'Cancelled',
}

export interface OtaUpdate {
  id: string;
  update_id: string;
  truck_id: string | null;
  fleet_id: string | null;
  version: string;
  target: UpdateTarget;
  url: string;
  checksum: string;
  signature: string;
  size_bytes: number;
  priority: UpdatePriority;
  requires_reboot: boolean;
  deadline: string | null;
  meta :UpdateMetadata;
  status: OtaStatus;
  progress_percent: number;
  started_at: string | null;
  completed_at: string | null;
  last_error: string | null;
  created_at: string;
  updated_at: string;
}

export interface UpdateMetadata {
  description: string;
  author: string;
  release_notes: string;
  compatibility: string[];
  estimated_apply_time_sec: number;
}

export interface RemoteCommand {
  id: string;
  command_id: string;
  truck_id: string | null;
  fleet_id: string | null;
  command_type: CommandType;
  parameters: any; // JSON object with command-specific parameters
  issued_at: string;
  deadline: string | null;
  requires_ack: boolean;
  status: CommandStatus;
  result: any | null; // JSON object with command result
  error: string | null;
  completed_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface OtaSummary {
  id: string;
  update_id: string;
  truck_id: string | null;
  fleet_id: string | null;
  version: string;
  target: UpdateTarget;
  priority: UpdatePriority;
  status: OtaStatus;
  progress_percent: number;
  created_at: string;
  truck_license_plate: string | null;
  truck_model: string | null;
  truck_make: string | null;
}

export interface OtaStats {
  total_updates: number;
  pending_updates: number;
  in_progress_updates: number;
  successful_updates: number;
  failed_updates: number;
  rollback_updates: number;
  by_target: Record<string, number>;
  by_priority: Record<string, number>;
  by_status: Record<string, number>;
  last_24_hours: number;
  last_7_days: number;
  last_30_days: number;
}