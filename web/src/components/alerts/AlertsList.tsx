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
} from '@mui/material';
import { AlertCard } from './AlertCard';
import { FilterBar } from '../../common/FilterBar';
import { useAlerts } from '../../hooks/useAlerts';
import { DateRange } from '@mui/x-date-pickers-pro';

type Severity = 'Critical' | 'Warning' | 'Info' | 'Emergency';
type Status = 'Triggered' | 'Acknowledged' | 'Resolved' | 'Suppressed';

interface AlertItem {
  id: string;
  alert_type: string;
  severity: Severity;
  status: Status;
  truck_id: string;
  truck_license_plate: string;
  truck_model: string;
  truck_make: string;
  triggered_at: string;
  acknowledged_at?: string;
  resolved_at?: string;
  source: string;
  context?: Record<string, any>;
  message: string;
}

interface Filters {
  severity: string;
  alertType: string;
  status: string;
  truckId: string;
  dateRange: DateRange<Date>;
}

const AlertsList: React.FC = () => {
  const { alerts, loading, error, fetchAlerts } = useAlerts() as {
    alerts: AlertItem[];
    loading: boolean;
    error: boolean;
    fetchAlerts: () => void;
  };

  const [activeTab, setActiveTab] = useState<number>(0);
  const [filters, setFilters] = useState<Filters>({
    severity: '',
    alertType: '',
    status: '',
    truckId: '',
    dateRange: [null, null],
  });

  useEffect(() => {
    fetchAlerts();
  }, [fetchAlerts]);

  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  const filteredAlerts = alerts.filter(alert => {
    if (filters.severity && alert.severity !== filters.severity) return false;
    if (filters.alertType && alert.alert_type !== filters.alertType) return false;
    if (filters.status && alert.status !== filters.status) return false;
    if (filters.truckId && alert.truck_id !== filters.truckId) return false;
    if (filters.dateRange[0] && new Date(alert.triggered_at) < filters.dateRange[0]) return false;
    if (filters.dateRange[1] && new Date(alert.triggered_at) > filters.dateRange[1]) return false;
    return true;
  });

  const criticalAlerts = filteredAlerts.filter(a => a.severity === 'Critical' || a.severity === 'Emergency');
  const warningAlerts = filteredAlerts.filter(a => a.severity === 'Warning');
  const infoAlerts = filteredAlerts.filter(a => a.severity === 'Info');

  const availableFilters = [
    {
      key: 'severity',
      label: 'Severity',
      type: 'select',
      options: [
        { value: 'Critical', label: 'Critical' },
        { value: 'Warning', label: 'Warning' },
        { value: 'Info', label: 'Info' },
        { value: 'Emergency', label: 'Emergency' },
      ],
    },
    {
      key: 'alertType',
      label: 'Alert Type',
      type: 'select',
      options: [
        { value: 'DrowsyDriver', label: 'Drowsy Driver' },
        { value: 'LaneDeparture', label: 'Lane Departure' },
        { value: 'CargoTamper', label: 'Cargo Tamper' },
        { value: 'HarshBraking', label: 'Harsh Braking' },
        { value: 'RapidAcceleration', label: 'Rapid Acceleration' },
        { value: 'OverSpeeding', label: 'Over Speeding' },
        { value: 'HighTemperature', label: 'High Temperature' },
        { value: 'LowDiskSpace', label: 'Low Disk Space' },
      ],
    },
    {
      key: 'status',
      label: 'Status',
      type: 'select',
      options: [
        { value: 'Triggered', label: 'Triggered' },
        { value: 'Acknowledged', label: 'Acknowledged' },
        { value: 'Resolved', label: 'Resolved' },
        { value: 'Suppressed', label: 'Suppressed' },
      ],
    },
    {
      key: 'dateRange',
      label: 'Date Range',
      type: 'dateRange',
    },
  ];

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
        Error loading alerts. Please try again later.
      </Alert>
    );
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4">
          Alerts ({filteredAlerts.length})
        </Typography>
        <Button variant="contained" color="primary">
          Create Alert
        </Button>
      </Box>

      <FilterBar
        filters={filters}
        onFilterChange={setFilters}
        onApplyFilters={() => {}}
        onResetFilters={() => {}}
        availableFilters={availableFilters}
      />

      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} sm={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Critical Alerts
            </Typography>
            <Typography variant="h3" color="error" sx={{ fontWeight: 'bold' }}>
              {criticalAlerts.length}
            </Typography>
            <Typography variant="body2" color="textSecondary">
              Requires immediate attention
            </Typography>
          </Paper>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Warning Alerts
            </Typography>
            <Typography variant="h3" color="warning" sx={{ fontWeight: 'bold' }}>
              {warningAlerts.length}
            </Typography>
            <Typography variant="body2" color="textSecondary">
              Monitor and address soon
            </Typography>
          </Paper>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Info Alerts
            </Typography>
            <Typography variant="h3" color="info" sx={{ fontWeight: 'bold' }}>
              {infoAlerts.length}
            </Typography>
            <Typography variant="body2" color="textSecondary">
              Informational notifications
            </Typography>
          </Paper>
        </Grid>
      </Grid>

      <Tabs value={activeTab} onChange={handleTabChange} sx={{ mb: 3 }}>
        <Tab label="All Alerts" />
        <Tab label="Critical & Emergency" />
        <Tab label="Warning" />
        <Tab label="Informational" />
        <Tab label="Acknowledged" />
        <Tab label="Resolved" />
      </Tabs>

      {filteredAlerts.length === 0 ? (
        <Alert severity="info">
          No alerts found matching your criteria.
        </Alert>
      ) : (
        <Grid container spacing={3}>
          {filteredAlerts
            .filter(alert => {
              switch (activeTab) {
                case 1:
                  return alert.severity === 'Critical' || alert.severity === 'Emergency';
                case 2:
                  return alert.severity === 'Warning';
                case 3:
                  return alert.severity === 'Info';
                case 4:
                  return alert.status === 'Acknowledged';
                case 5:
                  return alert.status === 'Resolved';
                default:
                  return true;
              }
            })
            .map(alert => (
              <Grid item xs={12} key={alert.id}>
                <AlertCard alert={alert} />
              </Grid>
            ))}
        </Grid>
      )}
    </Box>
  );
};

export default AlertsList;
