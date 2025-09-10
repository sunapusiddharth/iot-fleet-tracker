import React, { useState, useEffect } from 'react';
import {
  Grid,
  Paper,
  Typography,
  Box,
  CircularProgress,
  Alert,
} from '@mui/material';
import { useTrucks } from '../../hooks/useTrucks';
import { useAlerts } from '../../hooks/useAlerts';
import { useMlEvents } from '../../hooks/useMlEvents';
import { useHealth } from '../../hooks/useHealth';
import DashboardStats from './DashboardStats';
import RecentAlerts from './RecentAlerts';
import TruckStatusChart from './TruckStatusChart';
import HealthMetricsChart from './HealthMetricsChart';
import { BarChart, LineChart, PieChart, MapChart } from '../charts';

const Dashboard = () => {
  const { trucks, loading: trucksLoading, error: trucksError } = useTrucks();
  const { alerts, loading: alertsLoading, error: alertsError } = useAlerts();
  const { mlEvents, loading: mlLoading, error: mlError } = useMlEvents();
  const { healthStatus, loading: healthLoading, error: healthError } = useHealth();
  
  const [dateRange, setDateRange] = useState([
    new Date(Date.now() - 7 * 24 * 60 * 60 * 1000),
    new Date(),
  ]);

  if (trucksLoading || alertsLoading || mlLoading || healthLoading) {
    return (
      <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
        <CircularProgress />
      </Box>
    );
  }

  if (trucksError || alertsError || mlError || healthError) {
    return (
      <Alert severity="error">
        Error loading dashboard data. Please try again later.
      </Alert>
    );
  }

  // Calculate statistics
  const onlineTrucks = trucks.filter(t => t.status === 'Online').length;
  const offlineTrucks = trucks.filter(t => t.status === 'Offline').length;
  const maintenanceTrucks = trucks.filter(t => t.status === 'Maintenance').length;
  const criticalAlerts = alerts.filter(a => a.severity === 'Critical' || a.severity === 'Emergency').length;
  const warningAlerts = alerts.filter(a => a.severity === 'Warning').length;
  const mlAlerts = mlEvents.filter(e => e.confidence > 0.8).length;
  const avgHealthScore = trucks.length > 0 
    ? trucks.reduce((sum, t) => sum + t.health_score, 0) / trucks.length 
    : 0;

  // Prepare chart data
  const truckStatusData = [
    { name: 'Online', value: onlineTrucks },
    { name: 'Offline', value: offlineTrucks },
    { name: 'Maintenance', value: maintenanceTrucks },
  ];

  const alertSeverityData = [
    { name: 'Critical', value: criticalAlerts },
    { name: 'Warning', value: warningAlerts },
    { name: 'Info', value: alerts.length - criticalAlerts - warningAlerts },
  ];

  const mlEventData = mlEvents.slice(0, 10).map(event => ({
    name: event.model_name,
    confidence: event.confidence,
    timestamp: new Date(event.timestamp).toLocaleTimeString(),
  }));

  const healthData = healthStatus.slice(0, 20).map(status => ({
    name: new Date(status.timestamp).toLocaleTimeString(),
    cpu: status.cpu_percent,
    memory: status.memory_percent,
    disk: status.disk_percent,
    temperature: status.temperature_c,
  }));

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Fleet Dashboard
      </Typography>
      
      <Grid container spacing={3}>
        {/* Stats Cards */}
        <Grid item xs={12}>
          <DashboardStats 
            trucks={trucks} 
            alerts={alerts} 
            mlEvents={mlEvents} 
            healthStatus={healthStatus}
          />
        </Grid>
        
        {/* Charts Row 1 */}
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Truck Status Distribution
            </Typography>
            <PieChart data={truckStatusData} dataKey="value" nameKey="name" />
          </Paper>
        </Grid>
        
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Alert Severity Distribution
            </Typography>
            <PieChart data={alertSeverityData} dataKey="value" nameKey="name" />
          </Paper>
        </Grid>
        
        {/* Charts Row 2 */}
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Recent ML Events (Confidence)
            </Typography>
            <BarChart 
              data={mlEventData} 
              dataKey="name" 
              bars={[
                { dataKey: 'confidence', name: 'Confidence' }
              ]}
            />
          </Paper>
        </Grid>
        
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              System Health Metrics
            </Typography>
            <LineChart 
              data={healthData} 
              dataKey="name" 
              lines={[
                { dataKey: 'cpu', name: 'CPU %' },
                { dataKey: 'memory', name: 'Memory %' },
                { dataKey: 'disk', name: 'Disk %' },
                { dataKey: 'temperature', name: 'Temp °C' },
              ]}
            />
          </Paper>
        </Grid>
        
        {/* Recent Alerts */}
        <Grid item xs={12}>
          <RecentAlerts alerts={alerts.slice(0, 5)} />
        </Grid>
        
        {/* Map View */}
        <Grid item xs={12}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Truck Locations
            </Typography>
            <Box sx={{ height: 400 }}>
              <MapChart trucks={trucks} />
            </Box>
          </Paper>
        </Grid>
      </Grid>
    </Box>
  );
};

export default Dashboard;