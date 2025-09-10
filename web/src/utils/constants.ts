export const TRUCK_STATUSES = [
  { value: 'Online', label: 'Online' },
  { value: 'Offline', label: 'Offline' },
  { value: 'Maintenance', label: 'Maintenance' },
];

export const ALERT_SEVERITIES = [
  { value: 'Critical', label: 'Critical' },
  { value: 'Warning', label: 'Warning' },
  { value: 'Info', label: 'Info' },
  { value: 'Emergency', label: 'Emergency' },
];

export const ALERT_TYPES = [
  { value: 'DrowsyDriver', label: 'Drowsy Driver' },
  { value: 'LaneDeparture', label: 'Lane Departure' },
  { value: 'CargoTamper', label: 'Cargo Tamper' },
  { value: 'HarshBraking', label: 'Harsh Braking' },
  { value: 'RapidAcceleration', label: 'Rapid Acceleration' },
  { value: 'OverSpeeding', label: 'Over Speeding' },
  { value: 'HighTemperature', label: 'High Temperature' },
  { value: 'LowDiskSpace', label: 'Low Disk Space' },
];

export const ALERT_STATUSES = [
  { value: 'Triggered', label: 'Triggered' },
  { value: 'Acknowledged', label: 'Acknowledged' },
  { value: 'Resolved', label: 'Resolved' },
  { value: 'Suppressed', label: 'Suppressed' },
];

export const ML_MODELS = [
  { value: 'drowsiness', label: 'Drowsiness Detection' },
  { value: 'lane_departure', label: 'Lane Departure' },
  { value: 'cargo_tamper', label: 'Cargo Tamper' },
  { value: 'license_plate', label: 'License Plate Recognition' },
  { value: 'weather', label: 'Weather Classification' },
];

export const HEALTH_STATUSES = [
  { value: 'Ok', label: 'Ok' },
  { value: 'Warning', label: 'Warning' },
  { value: 'Critical', label: 'Critical' },
  { value: 'Degraded', label: 'Degraded' },
  { value: 'ShutdownPending', label: 'Shutdown Pending' },
];

export const OTA_TARGETS = [
  { value: 'Agent', label: 'Agent' },
  { value: 'Model', label: 'Model' },
  { value: 'Config', label: 'Config' },
  { value: 'Firmware', label: 'Firmware' },
];

export const OTA_PRIORITIES = [
  { value: 'Critical', label: 'Critical' },
  { value: 'High', label: 'High' },
  { value: 'Medium', label: 'Medium' },
  { value: 'Low', label: 'Low' },
];

export const OTA_STATUSES = [
  { value: 'Pending', label: 'Pending' },
  { value: 'Downloading', label: 'Downloading' },
  { value: 'Verifying', label: 'Verifying' },
  { value: 'Applying', label: 'Applying' },
  { value: 'Success', label: 'Success' },
  { value: 'Failed', label: 'Failed' },
  { value: 'Rollback', label: 'Rollback' },
];

export const COMMAND_TYPES = [
  { value: 'Reboot', label: 'Reboot' },
  { value: 'Shutdown', label: 'Shutdown' },
  { value: 'RestartService', label: 'Restart Service' },
  { value: 'GetDiagnostics', label: 'Get Diagnostics' },
  { value: 'UpdateConfig', label: 'Update Config' },
  { value: 'RunHealthCheck', label: 'Run Health Check' },
  { value: 'CaptureSnapshot', label: 'Capture Snapshot' },
  { value: 'FlushWAL', label: 'Flush WAL' },
];

export const COMMAND_STATUSES = [
  { value: 'Pending', label: 'Pending' },
  { value: 'Executing', label: 'Executing' },
  { value: 'Success', label: 'Success' },
  { value: 'Failed', label: 'Failed' },
  { value: 'Timeout', label: 'Timeout' },
  { value: 'Cancelled', label: 'Cancelled' },
];

export const DATE_RANGES = [
  { value: 'today', label: 'Today' },
  { value: 'yesterday', label: 'Yesterday' },
  { value: 'thisWeek', label: 'This Week' },
  { value: 'thisMonth', label: 'This Month' },
  { value: 'lastMonth', label: 'Last Month' },
  { value: 'thisYear', label: 'This Year' },
  { value: 'all', label: 'All Time' },
];

export const TIME_RANGES = [
  { value: 'all', label: 'All' },
  { value: 'morning', label: 'Morning (6am-12pm)' },
  { value: 'afternoon', label: 'Afternoon (12pm-6pm)' },
  { value: 'evening', label: 'Evening (6pm-10pm)' },
  { value: 'night', label: 'Night (10pm-6am)' },
];

export const PAGINATION_OPTIONS = [10, 25, 50, 100];

export const CHART_COLORS = [
  '#1976d2', // primary
  '#dc004e', // secondary
  '#ff9800', // warning
  '#4caf50', // success
  '#f44336', // error
  '#9c27b0', // purple
  '#3f51b5', // indigo
  '#009688', // teal
  '#795548', // brown
  '#607d8b', // blueGrey
];

export const MAP_STYLES = [
  { value: 'mapbox://styles/mapbox/streets-v11', label: 'Streets' },
  { value: 'mapbox://styles/mapbox/satellite-v9', label: 'Satellite' },
  { value: 'mapbox://styles/mapbox/light-v10', label: 'Light' },
  { value: 'mapbox://styles/mapbox/dark-v10', label: 'Dark' },
  { value: 'mapbox://styles/mapbox/navigation-day-v1', label: 'Navigation Day' },
  { value: 'mapbox://styles/mapbox/navigation-night-v1', label: 'Navigation Night' },
];