import React from 'react';
import {
  Card,
  CardContent,
  CardActions,
  Typography,
  Box,
  Chip,
  Avatar,
  Button,
  Divider,
  LinearProgress,
  Grid,
} from '@mui/material';
import {
  HealthAndSafety as HealthIcon,
  Memory as MemoryIcon,
  Storage as StorageIcon,
  DeviceThermostat as TempIcon,
  Speed as SpeedIcon,
  Warning as WarningIcon,
  Error as ErrorIcon,
  CheckCircle as CheckCircleIcon,
} from '@mui/icons-material';
import { StatusBadge } from '../common/StatusBadge';
import { Link } from 'react-router-dom';

type HealthStatusType = 'Critical' | 'Warning' | 'Ok' | string;

interface Alert {
  alert_type: string;
  severity: 'Critical' | 'Warning' | 'Info' | 'Emergency';
}

interface HealthStatus {
  id: string;
  truck_id: string;
  status: HealthStatusType;
  cpu_percent: number;
  memory_percent: number;
  disk_percent: number;
  temperature_c: number;
  uptime_sec: number;
  timestamp: string;
  alerts?: Alert[];
}

interface HealthStatusCardProps {
  status: HealthStatus;
}

const HealthStatusCard: React.FC<HealthStatusCardProps> = ({ status }) => {
  const getStatusIcon = (statusType: HealthStatusType) => {
    switch (statusType) {
      case 'Critical':
        return <ErrorIcon />;
      case 'Warning':
        return <WarningIcon />;
      case 'Ok':
        return <CheckCircleIcon />;
      default:
        return <HealthIcon />;
    }
  };

  const getStatusColor = (statusType: HealthStatusType): 'error' | 'warning' | 'success' | 'primary' => {
    switch (statusType) {
      case 'Critical':
        return 'error';
      case 'Warning':
        return 'warning';
      case 'Ok':
        return 'success';
      default:
        return 'primary';
    }
  };

  const getResourceColor = (value: number, max: number): 'error' | 'warning' | 'primary' | 'success' => {
    const percentage = (value / max) * 100;
    if (percentage > 90) return 'error';
    if (percentage > 80) return 'warning';
    if (percentage > 60) return 'primary';
    return 'success';
  };

  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', mb: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
            <Avatar sx={{ bgcolor: getStatusColor(status.status) }}>
              {getStatusIcon(status.status)}
            </Avatar>
            <Box>
              <Typography variant="h6">Truck {status.truck_id}</Typography>
              <Typography variant="body2" color="textSecondary">
                Health Status: {status.status}
              </Typography>
            </Box>
          </Box>
          <StatusBadge status={status.status} severity={status.status} />
        </Box>

        <Divider sx={{ my: 2 }} />

        <Grid container spacing={2}>
          {/* CPU */}
          <Grid item xs={12} sm={6}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 2 }}>
              <SpeedIcon color="primary" />
              <Typography variant="subtitle2" color="textSecondary">CPU Usage:</Typography>
            </Box>
            <LinearProgress
              variant="determinate"
              value={status.cpu_percent}
              color={getResourceColor(status.cpu_percent, 100)}
              sx={{ height: 8, borderRadius: 4, mb: 1 }}
            />
            <Typography variant="body2">{status.cpu_percent.toFixed(1)}%</Typography>
          </Grid>

          {/* Memory */}
          <Grid item xs={12} sm={6}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 2 }}>
              <MemoryIcon color="primary" />
              <Typography variant="subtitle2" color="textSecondary">Memory Usage:</Typography>
            </Box>
            <LinearProgress
              variant="determinate"
              value={status.memory_percent}
              color={getResourceColor(status.memory_percent, 100)}
              sx={{ height: 8, borderRadius: 4, mb: 1 }}
            />
            <Typography variant="body2">{status.memory_percent.toFixed(1)}%</Typography>
          </Grid>

          {/* Disk */}
          <Grid item xs={12} sm={6}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 2 }}>
              <StorageIcon color="primary" />
              <Typography variant="subtitle2" color="textSecondary">Disk Usage:</Typography>
            </Box>
            <LinearProgress
              variant="determinate"
              value={status.disk_percent}
              color={getResourceColor(status.disk_percent, 100)}
              sx={{ height: 8, borderRadius: 4, mb: 1 }}
            />
            <Typography variant="body2">{status.disk_percent.toFixed(1)}%</Typography>
          </Grid>

          {/* Temperature */}
          <Grid item xs={12} sm={6}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 2 }}>
              <TempIcon color="primary" />
              <Typography variant="subtitle2" color="textSecondary">Temperature:</Typography>
            </Box>
            <LinearProgress
              variant="determinate"
              value={Math.min(status.temperature_c, 100)}
              color={status.temperature_c > 80 ? 'error' : status.temperature_c > 70 ? 'warning' : 'success'}
              sx={{ height: 8, borderRadius: 4, mb: 1 }}
            />
            <Typography variant="body2">{status.temperature_c.toFixed(1)}°C</Typography>
          </Grid>

          {/* Uptime & Alerts */}
          <Grid item xs={12}>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Uptime:</Typography>
              <Typography variant="body2">
                {Math.floor(status.uptime_sec / 3600)}h {Math.floor((status.uptime_sec % 3600) / 60)}m
              </Typography>
            </Box>

            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Last Updated:</Typography>
              <Typography variant="body2">
                {new Date(status.timestamp).toLocaleString()}
              </Typography>
            </Box>

            {status.alerts && status.alerts.length > 0 && (
              <Box>
                <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                  Active Alerts ({status.alerts.length}):
                </Typography>
                <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1 }}>
                  {status.alerts.slice(0, 3).map((alert, index) => (
                    <Chip
                      key={index}
                      label={alert.alert_type}
                      size="small"
                      color={
                        alert.severity === 'Critical'
                          ? 'error'
                          : alert.severity === 'Warning'
                          ? 'warning'
                          : 'default'
                      }
                    />
                  ))}
                  {status.alerts.length > 3 && (
                    <Chip label={`+${status.alerts.length - 3} more`} size="small" />
                  )}
                </Box>
              </Box>
            )}
          </Grid>
        </Grid>
      </CardContent>

      <CardActions sx={{ justifyContent: 'flex-end' }}>
        <Button
          size="small"
          component={Link}
          to={`/health/${status.id}`}
          variant="outlined"
        >
          View Details
        </Button>
      </CardActions>
    </Card>
  );
};

export default HealthStatusCard;
