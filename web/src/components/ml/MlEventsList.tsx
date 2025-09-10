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
import { MlEventCard } from './MlEventCard';
import { FilterBar } from '../../common/FilterBar';
import { useMlEvents } from '../../hooks/useMlEvents';
import { DateRange } from '@mui/x-date-pickers-pro';

interface MlEvent {
  id: string;
  model_name: string;
  result_type: string;
  is_alert: boolean;
  confidence: number;
  latency_ms: number;
  timestamp: string;
  hardware_used: string;
  truck_id: string;
  truck_license_plate: string;
  truck_model: string;
  truck_make: string;
  meta?: {
    location?: [number, number];
    [key: string]: any;
  };
}

interface Filters {
  model: string;
  resultType: string;
  isAlert: string;
  truckId: string;
  dateRange: DateRange<Date>;
}

const MlEventsList: React.FC = () => {
  const { mlEvents, loading, error, fetchMlEvents } = useMlEvents() as {
    mlEvents: MlEvent[];
    loading: boolean;
    error: boolean;
    fetchMlEvents: () => void;
  };

  const [activeTab, setActiveTab] = useState<number>(0);
  const [filters, setFilters] = useState<Filters>({
    model: '',
    resultType: '',
    isAlert: '',
    truckId: '',
    dateRange: [null, null],
  });

  useEffect(() => {
    fetchMlEvents();
  }, [fetchMlEvents]);

  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  const filteredMlEvents = mlEvents.filter(event => {
    if (filters.model && event.model_name !== filters.model) return false;
    if (filters.resultType && !event.result_type.includes(filters.resultType)) return false;
    if (filters.isAlert !== '' && event.is_alert !== (filters.isAlert === 'true')) return false;
    if (filters.truckId && event.truck_id !== filters.truckId) return false;
    if (filters.dateRange[0] && new Date(event.timestamp) < filters.dateRange[0]) return false;
    if (filters.dateRange[1] && new Date(event.timestamp) > filters.dateRange[1]) return false;
    return true;
  });

  const alertEvents = filteredMlEvents.filter(e => e.is_alert);
  const highConfidenceEvents = filteredMlEvents.filter(e => e.confidence > 0.9);
  const recentEvents = filteredMlEvents.filter(e =>
    new Date(e.timestamp) > new Date(Date.now() - 24 * 60 * 60 * 1000)
  );

  const availableFilters = [
    {
      key: 'model',
      label: 'Model',
      type: 'select',
      options: [
        { value: 'drowsiness', label: 'Drowsiness Detection' },
        { value: 'lane_departure', label: 'Lane Departure' },
        { value: 'cargo_tamper', label: 'Cargo Tamper' },
        { value: 'license_plate', label: 'License Plate' },
        { value: 'weather', label: 'Weather Classification' },
      ],
    },
    {
      key: 'resultType',
      label: 'Result Type',
      type: 'select',
      options: [
        { value: 'Drowsiness', label: 'Drowsiness' },
        { value: 'LaneDeparture', label: 'Lane Departure' },
        { value: 'CargoTamper', label: 'Cargo Tamper' },
        { value: 'LicensePlate', label: 'License Plate' },
        { value: 'Weather', label: 'Weather' },
      ],
    },
    {
      key: 'isAlert',
      label: 'Is Alert',
      type: 'select',
      options: [
        { value: 'true', label: 'Yes' },
        { value: 'false', label: 'No' },
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
        Error loading ML events. Please try again later.
      </Alert>
    );
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4">
          ML Events ({filteredMlEvents.length})
        </Typography>
        <Button variant="contained" color="primary">
          Create ML Event
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
        {[
          { label: 'Alert Events', count: alertEvents.length, color: 'error', note: 'Events with confidence > 0.8' },
          { label: 'High Confidence', count: highConfidenceEvents.length, color: 'success', note: 'Events with confidence > 0.9' },
          { label: 'Recent Events', count: recentEvents.length, color: 'primary', note: 'Last 24 hours' },
        ].map((stat, idx) => (
          <Grid item xs={12} sm={4} key={idx}>
            <Paper sx={{ p: 2, height: '100%' }}>
              <Typography variant="h6" gutterBottom>{stat.label}</Typography>
              <Typography variant="h3" color={stat.color as any} sx={{ fontWeight: 'bold' }}>
                {stat.count}
              </Typography>
              <Typography variant="body2" color="textSecondary">{stat.note}</Typography>
            </Paper>
          </Grid>
        ))}
      </Grid>

      <Tabs value={activeTab} onChange={handleTabChange} sx={{ mb: 3 }}>
        <Tab label="All Events" />
        <Tab label="Alert Events" />
        <Tab label="High Confidence" />
        <Tab label="Recent Events" />
        <Tab label="By Model" />
      </Tabs>

      {filteredMlEvents.length === 0 ? (
        <Alert severity="info">
          No ML events found matching your criteria.
        </Alert>
      ) : (
        <Grid container spacing={3}>
          {filteredMlEvents
            .filter(event => {
              switch (activeTab) {
                case 1: return event.is_alert;
                case 2: return event.confidence > 0.9;
                case 3: return new Date(event.timestamp) > new Date(Date.now() - 24 * 60 * 60 * 1000);
                case 4: return true;
                default: return true;
              }
            })
            .map(event => (
              <Grid item xs={12} sm={6} md={4} key={event.id}>
                <MlEventCard event={event} />
              </Grid>
            ))}
        </Grid>
      )}
    </Box>
  );
};

export default MlEventsList;
