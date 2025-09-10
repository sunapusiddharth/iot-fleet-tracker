export const formatNumber = (num:number, decimals = 2) => {
    if (num === null || num === undefined) return 'N/A';
    return num.toFixed(decimals);
};

export const formatPercentage = (num:number, decimals = 1) => {
    if (num === null || num === undefined) return 'N/A';
    return `${num.toFixed(decimals)}%`;
};

export const formatBytes = (bytes:number, decimals = 2) => {
    if (bytes === 0) return '0 Bytes';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
};

export const formatDate = (date:Date, format = 'PPp') => {
    if (!date) return 'N/A';
    return new Date(date).toLocaleString();
};

export const formatDuration = (seconds:number) => {
    if (seconds === null || seconds === undefined) return 'N/A';

    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    if (hours > 0) {
        return `${hours}h ${minutes}m ${secs}s`;
    } else if (minutes > 0) {
        return `${minutes}m ${secs}s`;
    } else {
        return `${secs}s`;
    }
};

export const formatSpeed = (speed:number) => {
    if (speed === null || speed === undefined) return 'N/A';
    return `${speed.toFixed(1)} km/h`;
};

export const formatTemperature = (temp:number) => {
    if (temp === null || temp === undefined) return 'N/A';
    return `${temp.toFixed(1)}°C`;
};

export const formatPressure = (pressure:number) => {
    if (pressure === null || pressure === undefined) return 'N/A';
    return `${pressure.toFixed(1)} psi`;
};

export const formatAlertType = (type:string) => {
    switch (type) {
        case 'DrowsyDriver':
            return 'Drowsy Driver';
        case 'LaneDeparture':
            return 'Lane Departure';
        case 'CargoTamper':
            return 'Cargo Tamper';
        case 'HarshBraking':
            return 'Harsh Braking';
        case 'RapidAcceleration':
            return 'Rapid Acceleration';
        case 'OverSpeeding':
            return 'Over Speeding';
        case 'HighTemperature':
            return 'High Temperature';
        case 'LowDiskSpace':
            return 'Low Disk Space';
        default:
            return type;
    }
};

export const formatModelName = (modelName:string) => {
    switch (modelName) {
        case 'drowsiness':
            return 'Drowsiness Detection';
        case 'lane_departure':
            return 'Lane Departure';
        case 'cargo_tamper':
            return 'Cargo Tamper';
        case 'license_plate':
            return 'License Plate Recognition';
        case 'weather':
            return 'Weather Classification';
        default:
            return modelName;
    }
};

export const formatTruckStatus = (status:string) => {
    switch (status) {
        case 'Online':
            return 'Online';
        case 'Offline':
            return 'Offline';
        case 'Maintenance':
            return 'Maintenance';
        default:
            return status;
    }
};

export const formatOtaTarget = (target:string) => {
    switch (target) {
        case 'Agent':
            return 'Agent';
        case 'Model':
            return 'Model';
        case 'Config':
            return 'Config';
        case 'Firmware':
            return 'Firmware';
        default:
            return target;
    }
};

export const formatOtaPriority = (priority:string) => {
    switch (priority) {
        case 'Critical':
            return 'Critical';
        case 'High':
            return 'High';
        case 'Medium':
            return 'Medium';
        case 'Low':
            return 'Low';
        default:
            return priority;
    }
};

export const formatOtaStatus = (status:string) => {
    switch (status) {
        case 'Pending':
            return 'Pending';
        case 'Downloading':
            return 'Downloading';
        case 'Verifying':
            return 'Verifying';
        case 'Applying':
            return 'Applying';
        case 'Success':
            return 'Success';
        case 'Failed':
            return 'Failed';
        case 'Rollback':
            return 'Rollback';
        default:
            return status;
    }
};

export const formatCommandType = (type:string) => {
    switch (type) {
        case 'Reboot':
            return 'Reboot';
        case 'Shutdown':
            return 'Shutdown';
        case 'RestartService':
            return 'Restart Service';
        case 'GetDiagnostics':
            return 'Get Diagnostics';
        case 'UpdateConfig':
            return 'Update Config';
        case 'RunHealthCheck':
            return 'Run Health Check';
        case 'CaptureSnapshot':
            return 'Capture Snapshot';
        case 'FlushWAL':
            return 'Flush WAL';
        default:
            return type;
    }
};

export const formatCommandStatus = (status:string) => {
    switch (status) {
        case 'Pending':
            return 'Pending';
        case 'Executing':
            return 'Executing';
        case 'Success':
            return 'Success';
        case 'Failed':
            return 'Failed';
        case 'Timeout':
            return 'Timeout';
        case 'Cancelled':
            return 'Cancelled';
        default:
            return status;
    }
};