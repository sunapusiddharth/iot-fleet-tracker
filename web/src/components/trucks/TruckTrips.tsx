import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Paper,
  Grid,
  Tabs,
  Tab,
  CircularProgress,
  Alert,
  Button,
  TextField,
  MenuItem,
  Select,
  FormControl,
  InputLabel,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from '@mui/material';
import { useTrucks } from '../../hooks/useTrucks';

const TruckTrips = ({ truckId }:{truckId:string}) => {
  const { truck, loading, error, fetchTruck } = useTrucks();
  const [activeTab, setActiveTab] = useState(0);
  const [timePeriod, setTimePeriod] = useState('week');

  useEffect(() => {
    fetchTruck(truckId);
  }, [truckId, fetchTruck]);

  const handleTabChange = (event:any, newValue:number) => {
    setActiveTab(newValue);
  };

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
        Error loading trip data. Please try again later.
      </Alert>
    );
  }

  if (!truck || !truck.recent_trips) {
    return (
      <Alert severity="info">
        No trip data available for this truck.
      </Alert>
    );
  }

  // Prepare data for charts
  const trips = truck.recent_trips.slice(0, 10);
  const tripData = trips.map(trip => ({
    name: `${new Date(trip.start_time).toLocaleDateString()}`,
    distance: trip.distance_km,
    duration: trip.duration_minutes,
    fuel: trip.fuel_consumed_liters,
    avgSpeed: trip.average_speed_kmh,
    maxSpeed: trip.max_speed_kmh,
  }));

  const alertData = truck.recent_trips.slice(0, 10).map(trip => ({
    name: `${new Date(trip.start_time).toLocaleDateString()}`,
    alerts: trip.alerts_count,
    events: trip.events_count,
  }));

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h5">
          Trips & Maintenance
        </Typography>
        <FormControl size="small">
          <InputLabel>Time Period</InputLabel>
          <Select
            value={timePeriod}
            onChange={(e) => setTimePeriod(e.target.value)}
            label="Time Period"
          >
            <MenuItem value="day">Last 24 Hours</MenuItem>
            <MenuItem value="week">Last Week</MenuItem>
            <MenuItem value="month">Last Month</MenuItem>
            <MenuItem value="year">Last Year</MenuItem>
          </Select>
        </FormControl>
      </Box>
      
      <Tabs
        value={activeTab}
        onChange={handleTabChange}
        sx={{ mb: 3 }}
      >
        <Tab label="Trip Statistics" />
        <Tab label="Trip History" />
        <Tab label="Maintenance Records" />
        <Tab label="Route Map" />
      </Tabs>
      
      {activeTab === 0 && (
        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Distance & Duration
              </Typography>
              <BarChart
                data={tripData}
                dataKey="name"
                bars={[
                  { dataKey: 'distance', name: 'Distance (km)', color: '#1976d2' },
                  { dataKey: 'duration', name: 'Duration (min)', color: '#dc004e' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Fuel Consumption & Speed
              </Typography>
              <BarChart
                data={tripData}
                dataKey="name"
                bars={[
                  { dataKey: 'fuel', name: 'Fuel (L)', color: '#ff9800' },
                  { dataKey: 'avgSpeed', name: 'Avg Speed (km/h)', color: '#4caf50' },
                  { dataKey: 'maxSpeed', name: 'Max Speed (km/h)', color: '#f44336' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Alerts & Events
              </Typography>
              <BarChart
                data={alertData}
                dataKey="name"
                bars={[
                  { dataKey: 'alerts', name: 'Alerts', color: '#f44336' },
                  { dataKey: 'events', name: 'Events', color: '#ff9800' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Trip Distribution
              </Typography>
              <PieChart
                data={[
                  { name: 'Short Trips (<100km)', value: trips.filter(t => t.distance_km < 100).length },
                  { name: 'Medium Trips (100-500km)', value: trips.filter(t => t.distance_km >= 100 && t.distance_km < 500).length },
                  { name: 'Long Trips (>500km)', value: trips.filter(t => t.distance_km >= 500).length },
                ]}
                dataKey="value"
                nameKey="name"
                height={300}
              />
            </Paper>
          </Grid>
        </Grid>
      )}
      
      {activeTab === 1 && (
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>
            Trip History ({trips.length} trips)
          </Typography>
          <TableContainer>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Date</TableCell>
                  <TableCell>Distance (km)</TableCell>
                  <TableCell>Duration</TableCell>
                  <TableCell>Avg Speed (km/h)</TableCell>
                  <TableCell>Max Speed (km/h)</TableCell>
                  <TableCell>Fuel (L)</TableCell>
                  <TableCell>Events</TableCell>
                  <TableCell>Alerts</TableCell>
                  <TableCell>Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {trips.map((trip, index) => (
                  <TableRow key={index}>
                    <TableCell>
                      {new Date(trip.start_time).toLocaleDateString()}<br/>
                      <Typography variant="caption" color="textSecondary">
                        {new Date(trip.start_time).toLocaleTimeString()} - {new Date(trip.end_time).toLocaleTimeString()}
                      </Typography>
                    </TableCell>
                    <TableCell>{trip.distance_km.toFixed(2)}</TableCell>
                    <TableCell>{Math.floor(trip.duration_minutes / 60)}h {trip.duration_minutes % 60}m</TableCell>
                    <TableCell>{trip.average_speed_kmh.toFixed(1)}</TableCell>
                    <TableCell>{trip.max_speed_kmh.toFixed(1)}</TableCell>
                    <TableCell>{trip.fuel_consumed_liters.toFixed(2)}</TableCell>
                    <TableCell>{trip.events_count}</TableCell>
                    <TableCell>{trip.alerts_count}</TableCell>
                    <TableCell>
                      <Button size="small" variant="outlined">
                        View Details
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </Paper>
      )}
      
      {activeTab === 2 && (
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>
            Maintenance Records ({truck.maintenance_history?.length || 0} records)
          </Typography>
          {truck.maintenance_history && truck.maintenance_history.length > 0 ? (
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Date</TableCell>
                    <TableCell>Type</TableCell>
                    <TableCell>Description</TableCell>
                    <TableCell>Cost</TableCell>
                    <TableCell>Performed By</TableCell>
                    <TableCell>Mileage</TableCell>
                    <TableCell>Next Due</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {truck.maintenance_history.map((record, index) => (
                    <TableRow key={index}>
                      <TableCell>{new Date(record.performed_at).toLocaleDateString()}</TableCell>
                      <TableCell>{record.maintenance_type}</TableCell>
                      <TableCell>{record.description}</TableCell>
                      <TableCell>${record.cost.toFixed(2)}</TableCell>
                      <TableCell>{record.performed_by}</TableCell>
                      <TableCell>{record.mileage.toLocaleString()} km</TableCell>
                      <TableCell>
                        {record.next_due_date ? new Date(record.next_due_date).toLocaleDateString() : 'N/A'}
                      </TableCell>
                      <TableCell>
                        <Button size="small" variant="outlined">
                          View Details
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          ) : (
            <Typography variant="body2" color="textSecondary">
              No maintenance records available
            </Typography>
          )}
          
          <Box sx={{ mt: 3 }}>
            <Button variant="contained" color="primary">
              Add Maintenance Record
            </Button>
          </Box>
        </Paper>
      )}
      
      {activeTab === 3 && (
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>
            Route Map
          </Typography>
          {trips.length > 0 ? (
            <Box sx={{ height: 600 }}>
              <MapChart 
                trucks={[]} 
                trips={trips}
                selectedTruckId={truckId}
              />
            </Box>
          ) : (
            <Typography variant="body2" color="textSecondary">
              No trip data available for mapping
            </Typography>
          )}
        </Paper>
      )}
    </Box>
  );
};

export default TruckTrips;