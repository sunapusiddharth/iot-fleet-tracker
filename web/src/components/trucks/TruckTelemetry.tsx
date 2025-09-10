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
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  InputAdornment,
  Button,
} from '@mui/material';
import {
  LineChart,
  BarChart,
  PieChart,
} from '../charts';
import { Search as SearchIcon } from '@mui/icons-material';
import { useTelemetry } from '../../hooks/useTelemetry';
import { DateTimePicker } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';

const TruckTelemetry = ({ truckId }) => {
  const { telemetry, loading, error, fetchTelemetry } = useTelemetry();
  const [activeTab, setActiveTab] = useState(0);
  const [dateRange, setDateRange] = useState([
    new Date(Date.now() - 24 * 60 * 60 * 1000),
    new Date(),
  ]);
  const [searchTerm, setSearchTerm] = useState('');

  useEffect(() => {
    fetchTelemetry(truckId);
  }, [truckId, fetchTelemetry]);

  const handleTabChange = (event, newValue) => {
    setActiveTab(newValue);
  };

  const filteredTelemetry = telemetry
    .filter(t => {
      const timestamp = new Date(t.timestamp);
      return timestamp >= dateRange[0] && timestamp <= dateRange[1];
    })
    .filter(t => {
      if (!searchTerm) return true;
      return (
        t.sensors.obd.rpm.toString().includes(searchTerm) ||
        t.sensors.obd.coolant_temp.toString().includes(searchTerm) ||
        t.sensors.obd.fuel_level.toString().includes(searchTerm) ||
        t.speed_kmh.toString().includes(searchTerm)
      );
    });

  const prepareChartData = () => {
    return filteredTelemetry.map(t => ({
      name: new Date(t.timestamp).toLocaleTimeString(),
      speed: t.speed_kmh,
      rpm: t.sensors.obd.rpm,
      coolant: t.sensors.obd.coolant_temp,
      fuel: t.sensors.obd.fuel_level,
      accel_x: t.sensors.imu.accel_x,
      accel_y: t.sensors.imu.accel_y,
      accel_z: t.sensors.imu.accel_z,
      tire_pressure: (
        t.sensors.tpms.front_left.pressure_psi +
        t.sensors.tpms.front_right.pressure_psi +
        t.sensors.tpms.rear_left.pressure_psi +
        t.sensors.tpms.rear_right.pressure_psi
      ) / 4,
    }));
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
        Error loading telemetry data. Please try again later.
      </Alert>
    );
  }

  const chartData = prepareChartData();

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h5">
          Telemetry Data
        </Typography>
        <Box sx={{ display: 'flex', gap: 2 }}>
          <LocalizationProvider dateAdapter={AdapterDateFns}>
            <DateTimePicker
              label="Start Date"
              value={dateRange[0]}
              onChange={(newValue) => setDateRange([newValue, dateRange[1]])}
              renderInput={(params) => <TextField {...params} size="small" />}
            />
            <DateTimePicker
              label="End Date"
              value={dateRange[1]}
              onChange={(newValue) => setDateRange([dateRange[0], newValue])}
              renderInput={(params) => <TextField {...params} size="small" />}
            />
          </LocalizationProvider>
          <TextField
            size="small"
            placeholder="Search values..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            InputProps={{
              startAdornment: (
                <InputAdornment position="start">
                  <SearchIcon />
                </InputAdornment>
              ),
            }}
          />
        </Box>
      </Box>
      
      <Tabs
        value={activeTab}
        onChange={handleTabChange}
        sx={{ mb: 3 }}
      >
        <Tab label="Speed & RPM" />
        <Tab label="Engine & Fuel" />
        <Tab label="Acceleration" />
        <Tab label="Tire Pressure" />
        <Tab label="Raw Data" />
      </Tabs>
      
      {activeTab === 0 && (
        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Speed Over Time
              </Typography>
              <LineChart
                data={chartData}
                dataKey="name"
                lines={[
                  { dataKey: 'speed', name: 'Speed (km/h)', color: '#1976d2' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                RPM Over Time
              </Typography>
              <LineChart
                data={chartData}
                dataKey="name"
                lines={[
                  { dataKey: 'rpm', name: 'RPM', color: '#dc004e' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
        </Grid>
      )}
      
      {activeTab === 1 && (
        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Coolant Temperature
              </Typography>
              <LineChart
                data={chartData}
                dataKey="name"
                lines={[
                  { dataKey: 'coolant', name: 'Temperature (°C)', color: '#ff9800' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Fuel Level
              </Typography>
              <LineChart
                data={chartData}
                dataKey="name"
                lines={[
                  { dataKey: 'fuel', name: 'Fuel Level (%)', color: '#4caf50' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
        </Grid>
      )}
      
      {activeTab === 2 && (
        <Grid container spacing={3}>
          <Grid item xs={12}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Acceleration (G-Force)
              </Typography>
              <LineChart
                data={chartData}
                dataKey="name"
                lines={[
                  { dataKey: 'accel_x', name: 'X-Axis', color: '#1976d2' },
                  { dataKey: 'accel_y', name: 'Y-Axis', color: '#dc004e' },
                  { dataKey: 'accel_z', name: 'Z-Axis', color: '#ff9800' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
        </Grid>
      )}
      
      {activeTab === 3 && (
        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Average Tire Pressure
              </Typography>
              <LineChart
                data={chartData}
                dataKey="name"
                lines={[
                  { dataKey: 'tire_pressure', name: 'Pressure (psi)', color: '#4caf50' },
                ]}
                height={300}
              />
            </Paper>
          </Grid>
          <Grid item xs={12} md={6}>
            <Paper sx={{ p: 2, height: 400 }}>
              <Typography variant="h6" gutterBottom>
                Tire Pressure Distribution (Latest)
              </Typography>
              {filteredTelemetry.length > 0 ? (
                <PieChart
                  data={[
                    { name: 'Front Left', value: filteredTelemetry[0].sensors.tpms.front_left.pressure_psi },
                    { name: 'Front Right', value: filteredTelemetry[0].sensors.tpms.front_right.pressure_psi },
                    { name: 'Rear Left', value: filteredTelemetry[0].sensors.tpms.rear_left.pressure_psi },
                    { name: 'Rear Right', value: filteredTelemetry[0].sensors.tpms.rear_right.pressure_psi },
                  ]}
                  dataKey="value"
                  nameKey="name"
                  height={300}
                />
              ) : (
                <Typography variant="body2" color="textSecondary">
                  No data available
                </Typography>
              )}
            </Paper>
          </Grid>
        </Grid>
      )}
      
      {activeTab === 4 && (
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>
            Raw Telemetry Data ({filteredTelemetry.length} records)
          </Typography>
          <Box sx={{ overflowX: 'auto' }}>
            <table style={{ width: '100%', borderCollapse: 'collapse' }}>
              <thead>
                <tr>
                  <th style={{ border: '1px solid #ddd', padding: '8px', textAlign: 'left' }}>Timestamp</th>
                  <th style={{ border: '1px solid #ddd', padding: '8px', textAlign: 'left' }}>Speed (km/h)</th>
                  <th style={{ border: '1px solid #ddd', padding: '8px', textAlign: 'left' }}>RPM</th>
                  <th style={{ border: '1px solid #ddd', padding: '8px', textAlign: 'left' }}>Coolant (°C)</th>
                  <th style={{ border: '1px solid #ddd', padding: '8px', textAlign: 'left' }}>Fuel (%)</th>
                  <th style={{ border: '1px solid #ddd', padding: '8px', textAlign: 'left' }}>Tire Pressure (psi)</th>
                </tr>
              </thead>
              <tbody>
                {filteredTelemetry.map((t, index) => (
                  <tr key={index} style={{ backgroundColor: index % 2 === 0 ? '#f9f9f9' : 'white' }}>
                    <td style={{ border: '1px solid #ddd', padding: '8px' }}>
                      {new Date(t.timestamp).toLocaleString()}
                    </td>
                    <td style={{ border: '1px solid #ddd', padding: '8px' }}>
                      {t.speed_kmh}
                    </td>
                    <td style={{ border: '1px solid #ddd', padding: '8px' }}>
                      {t.sensors.obd.rpm}
                    </td>
                    <td style={{ border: '1px solid #ddd', padding: '8px' }}>
                      {t.sensors.obd.coolant_temp}
                    </td>
                    <td style={{ border: '1px solid #ddd', padding: '8px' }}>
                      {t.sensors.obd.fuel_level}
                    </td>
                    <td style={{ border: '1px solid #ddd', padding: '8px' }}>
                      {(
                        t.sensors.tpms.front_left.pressure_psi +
                        t.sensors.tpms.front_right.pressure_psi +
                        t.sensors.tpms.rear_left.pressure_psi +
                        t.sensors.tpms.rear_right.pressure_psi
                      ) / 4}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </Box>
        </Paper>
      )}
    </Box>
  );
};

export default TruckTelemetry;