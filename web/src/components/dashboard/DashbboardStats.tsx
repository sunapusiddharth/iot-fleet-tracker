import React from 'react';
import {
  Grid,
  Paper,
  Typography,
  Box,
  LinearProgress,
} from '@mui/material';
import { StatusBadge } from '../common/StatusBadge';

type TruckStatus = 'Online' | 'Offline' | 'Maintenance';

interface Truck {
  id: string;
  status: TruckStatus;
  health_score: number;
  // Add other truck fields as needed
}

interface Alert {
  id: string;
  severity: 'Critical' | 'Warning' | 'Info' | 'Emergency';
  // Add other alert fields as needed
}

interface MlEvent {
  id: string;
  confidence: number;
  // Add other ML event fields as needed
}

interface DashboardStatsProps {
  trucks: Truck[];
  alerts: Alert[];
  mlEvents: MlEvent[];
  healthStatus?: any; // You can refine this if needed
}

const DashboardStats: React.FC<DashboardStatsProps> = ({
  trucks,
  alerts,
  mlEvents,
  healthStatus,
}) => {
  const onlineTrucks = trucks.filter(t => t.status === 'Online').length;
  const offlineTrucks = trucks.filter(t => t.status === 'Offline').length;
  const maintenanceTrucks = trucks.filter(t => t.status === 'Maintenance').length;

  const criticalAlerts = alerts.filter(a => a.severity === 'Critical' || a.severity === 'Emergency').length;
  const warningAlerts = alerts.filter(a => a.severity === 'Warning').length;

  const mlAlerts = mlEvents.filter(e => e.confidence > 0.8).length;

  const avgHealthScore = trucks.length > 0
    ? trucks.reduce((sum, t) => sum + t.health_score, 0) / trucks.length
    : 0;

  return (
    <Grid container spacing={3}>
      {/* Truck Stats */}
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, height: '100%' }}>
          <Typography variant="subtitle2" color="textSecondary">
            Total Trucks
          </Typography>
          <Typography variant="h4">{trucks.length}</Typography>
          <Box sx={{ mt: 2 }}>
            <Typography variant="caption" color="textSecondary">
              Online: {onlineTrucks}
            </Typography>
            <LinearProgress
              variant="determinate"
              value={(onlineTrucks / trucks.length) * 100}
              sx={{ mt: 1, height: 8, borderRadius: 4 }}
            />
          </Box>
        </Paper>
      </Grid>

      {/* Alert Stats */}
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, height: '100%' }}>
          <Typography variant="subtitle2" color="textSecondary">
            Active Alerts
          </Typography>
          <Typography variant="h4">{alerts.length}</Typography>
          <Box sx={{ mt: 2 }}>
            <Typography variant="caption" color="textSecondary">
              Critical: {criticalAlerts}
            </Typography>
            <LinearProgress
              variant="determinate"
              value={criticalAlerts > 0 ? (criticalAlerts / alerts.length) * 100 : 0}
              color="error"
              sx={{ mt: 1, height: 8, borderRadius: 4 }}
            />
          </Box>
        </Paper>
      </Grid>

      {/* ML Stats */}
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, height: '100%' }}>
          <Typography variant="subtitle2" color="textSecondary">
            ML Events
          </Typography>
          <Typography variant="h4">{mlEvents.length}</Typography>
          <Box sx={{ mt: 2 }}>
            <Typography variant="caption" color="textSecondary">
              High Confidence: {mlAlerts}
            </Typography>
            <LinearProgress
              variant="determinate"
              value={mlAlerts > 0 ? (mlAlerts / mlEvents.length) * 100 : 0}
              color="warning"
              sx={{ mt: 1, height: 8, borderRadius: 4 }}
            />
          </Box>
        </Paper>
      </Grid>

      {/* Health Stats */}
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, height: '100%' }}>
          <Typography variant="subtitle2" color="textSecondary">
            Avg Health Score
          </Typography>
          <Typography variant="h4">{avgHealthScore.toFixed(1)}%</Typography>
          <Box sx={{ mt: 2 }}>
            <StatusBadge
              status={`${avgHealthScore.toFixed(0)}%`}
              severity={
                avgHealthScore > 80
                  ? 'Success'
                  : avgHealthScore > 60
                  ? 'Warning'
                  : 'Critical'
              }
            />
            <LinearProgress
              variant="determinate"
              value={avgHealthScore}
              color={
                avgHealthScore > 80
                  ? 'success'
                  : avgHealthScore > 60
                  ? 'warning'
                  : 'error'
              }
              sx={{ mt: 1, height: 8, borderRadius: 4 }}
            />
          </Box>
        </Paper>
      </Grid>
    </Grid>
  );
};

export default DashboardStats;
