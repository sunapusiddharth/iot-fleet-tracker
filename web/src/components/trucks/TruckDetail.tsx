import  { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Tabs,
  Tab,
  Paper,
  Grid,
  CircularProgress,
  Alert,
  Breadcrumbs,
  Link as MuiLink,
} from '@mui/material';
import { useParams, Link } from 'react-router-dom';
import { useTrucks } from '../../hooks/useTrucks';
import { useAlerts } from '../../hooks/useAlerts';
import { useMlEvents } from '../../hooks/useMlEvents';

const TruckDetail = () => {
  const { id } = useParams();
  const { truck, loading: truckLoading, error: truckError, fetchTruck } = useTrucks();
  const { telemetry, loading: telemetryLoading, error: telemetryError, fetchTelemetry } = useTelemetry();
  const { alerts, loading: alertsLoading, error: alertsError, fetchAlerts } = useAlerts();
  const { mlEvents, loading: mlLoading, error: mlError, fetchMlEvents } = useMlEvents();
  
  const [activeTab, setActiveTab] = useState(0);

  useEffect(() => {
    fetchTruck(id);
    fetchTelemetry(id);
    fetchAlerts({ truckId: id });
    fetchMlEvents({ truckId: id });
  }, [id, fetchTruck, fetchTelemetry, fetchAlerts, fetchMlEvents]);

  const loading = truckLoading || telemetryLoading || alertsLoading || mlLoading;
  const error = truckError || telemetryError || alertsError || mlError;

  if (loading) {
    return (
      <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Alert severity="error">
        Error loading truck details. Please try again later.
      </Alert>
    );
  }

  if (!truck) {
    return (
      <Alert severity="info">
        Truck not found.
      </Alert>
    );
  }

  const handleTabChange = (event, newValue) => {
    setActiveTab(newValue);
  };

  return (
    <Box>
      <Breadcrumbs aria-label="breadcrumb" sx={{ mb: 3 }}>
        <MuiLink component={Link} to="/" color="inherit">
          Dashboard
        </MuiLink>
        <MuiLink component={Link} to="/trucks" color="inherit">
          Trucks
        </MuiLink>
        <Typography color="text.primary">{truck.truck_id}</Typography>
      </Breadcrumbs>
      
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Box>
          <Typography variant="h4">
            {truck.truck_id}
          </Typography>
          <Typography variant="subtitle1" color="textSecondary">
            {truck.make} {truck.model} ({truck.year}) - {truck.license_plate}
          </Typography>
        </Box>
        <StatusBadge status={truck.status} />
      </Box>
      
      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} md={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Current Status
            </Typography>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
              <Typography variant="subtitle1">Speed:</Typography>
              <Typography variant="h5" color={truck.speed_kmh > 100 ? 'error' : truck.speed_kmh > 80 ? 'warning' : 'success'}>
                {truck.speed_kmh || 0} km/h
              </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
              <Typography variant="subtitle1">Heading:</Typography>
              <Typography variant="h5">
                {truck.heading || 0}°
              </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
              <Typography variant="subtitle1">Health Score:</Typography>
              <Typography variant="h5" color={truck.health_score >= 80 ? 'success' : truck.health_score >= 60 ? 'warning' : 'error'}>
                {truck.health_score.toFixed(0)}%
              </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
              <Typography variant="subtitle1">Last Seen:</Typography>
              <Typography variant="body2">
                {new Date(truck.last_seen).toLocaleString()}
              </Typography>
            </Box>
          </Paper>
        </Grid>
        
        <Grid item xs={12} md={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Sensor Readings
            </Typography>
            {telemetry && telemetry.length > 0 ? (
              <>
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
                  <Typography variant="subtitle1">RPM:</Typography>
                  <Typography variant="h5">
                    {telemetry[0].sensors.obd.rpm}
                  </Typography>
                </Box>
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
                  <Typography variant="subtitle1">Coolant Temp:</Typography>
                  <Typography variant="h5" color={telemetry[0].sensors.obd.coolant_temp > 90 ? 'error' : 'inherit'}>
                    {telemetry[0].sensors.obd.coolant_temp}°C
                  </Typography>
                </Box>
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
                  <Typography variant="subtitle1">Fuel Level:</Typography>
                  <Typography variant="h5">
                    {telemetry[0].sensors.obd.fuel_level}%
                  </Typography>
                </Box>
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
                  <Typography variant="subtitle1">Tire Pressure:</Typography>
                  <Typography variant="body2">
                    FL: {telemetry[0].sensors.tpms.front_left.pressure_psi.toFixed(1)} psi, 
                    FR: {telemetry[0].sensors.tpms.front_right.pressure_psi.toFixed(1)} psi
                  </Typography>
                </Box>
              </>
            ) : (
              <Typography variant="body2" color="textSecondary">
                No sensor data available
              </Typography>
            )}
          </Paper>
        </Grid>
        
        <Grid item xs={12} md={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Recent Activity
            </Typography>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
              <Typography variant="subtitle1">Active Alerts:</Typography>
              <Typography variant="h5" color={truck.active_alerts > 0 ? 'error' : 'success'}>
                {truck.active_alerts}
              </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
              <Typography variant="subtitle1">ML Events:</Typography>
              <Typography variant="h5">
                {mlEvents.length}
              </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
              <Typography variant="subtitle1">Last Trip:</Typography>
              <Typography variant="body2">
                {truck.recent_trips && truck.recent_trips.length > 0 
                  ? new Date(truck.recent_trips[0].end_time).toLocaleDateString()
                  : 'No trips recorded'
                }
              </Typography>
            </Box>
          </Paper>
        </Grid>
      </Grid>
      
      <Tabs
        value={activeTab}
        onChange={handleTabChange}
        sx={{ mb: 3 }}
      >
        <Tab label="Images & Video" />
        <Tab label="Telemetry Data" />
        <Tab label="Trips & Maintenance" />
        <Tab label="Alerts & ML Events" />
      </Tabs>
      
      {activeTab === 0 && <TruckImages truckId={id} />}
      {activeTab === 1 && <TruckTelemetry truckId={id} />}
      {activeTab === 2 && <TruckTrips truckId={id} />}
      {activeTab === 3 && (
        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2 }}>
              <Typography variant="h6" gutterBottom>
                Recent Alerts ({alerts.length})
              </Typography>
              {/* Add alerts list here */}
            </Paper>
          </Grid>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2 }}>
              <Typography variant="h6" gutterBottom>
                Recent ML Events ({mlEvents.length})
              </Typography>
              {/* Add ML events list here */}
            </Paper>
          </Grid>
        </Grid>
      )}
    </Box>
  );
};

export default TruckDetail;