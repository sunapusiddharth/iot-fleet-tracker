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
} from '@mui/material';
import { Search as SearchIcon } from '@mui/icons-material';
import { FilterBar } from '../common/FilterBar';
import { StatusBadge } from '../common/StatusBadge';
import { useOta } from '../../hooks/useOta';

const OtaUpdatesList = () => {
  const { otaUpdates, loading, error, fetchOtaUpdates } = useOta();
  const [activeTab, setActiveTab] = useState(0);
  const [filters, setFilters] = useState({
    target: '',
    priority: '',
    status: '',
    truckId: '',
    dateRange: [null, null],
  });

  useEffect(() => {
    fetchOtaUpdates();
  }, [fetchOtaUpdates]);

  const handleTabChange = (event, newValue) => {
    setActiveTab(newValue);
  };

  const filteredOtaUpdates = otaUpdates.filter(update => {
    if (filters.target && update.target !== filters.target) return false;
    if (filters.priority && update.priority !== filters.priority) return false;
    if (filters.status && update.status !== filters.status) return false;
    if (filters.truckId && update.truck_id !== filters.truckId) return false;
    if (filters.dateRange[0] && new Date(update.created_at) < filters.dateRange[0]) return false;
    if (filters.dateRange[1] && new Date(update.created_at) > filters.dateRange[1]) return false;
    return true;
  });

  const pendingUpdates = filteredOtaUpdates.filter(u => u.status === 'Pending');
  const inProgressUpdates = filteredOtaUpdates.filter(u => 
    u.status === 'Downloading' || u.status === 'Verifying' || u.status === 'Applying'
  );
  const completedUpdates = filteredOtaUpdates.filter(u => 
    u.status === 'Success' || u.status === 'Failed' || u.status === 'Rollback'
  );
  const criticalUpdates = filteredOtaUpdates.filter(u => u.priority === 'Critical');

  const availableFilters = [
    {
      key: 'target',
      label: 'Target',
      type: 'select',
      options: [
        { value: 'Agent', label: 'Agent' },
        { value: 'Model', label: 'Model' },
        { value: 'Config', label: 'Config' },
        { value: 'Firmware', label: 'Firmware' },
      ],
    },
    {
      key: 'priority',
      label: 'Priority',
      type: 'select',
      options: [
        { value: 'Critical', label: 'Critical' },
        { value: 'High', label: 'High' },
        { value: 'Medium', label: 'Medium' },
        { value: 'Low', label: 'Low' },
      ],
    },
    {
      key: 'status',
      label: 'Status',
      type: 'select',
      options: [
        { value: 'Pending', label: 'Pending' },
        { value: 'Downloading', label: 'Downloading' },
        { value: 'Verifying', label: 'Verifying' },
        { value: 'Applying', label: 'Applying' },
        { value: 'Success', label: 'Success' },
        { value: 'Failed', label: 'Failed' },
        { value: 'Rollback', label: 'Rollback' },
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
        Error loading OTA updates. Please try again later.
      </Alert>
    );
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4">
          OTA Updates ({filteredOtaUpdates.length})
        </Typography>
        <Button variant="contained" color="primary">
          Create Update
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
              Pending Updates
            </Typography>
            <Typography variant="h3" color="warning" sx={{ fontWeight: 'bold' }}>
              {pendingUpdates.length}
            </Typography>
            <Typography variant="body2" color="textSecondary">
              Updates waiting to be applied
            </Typography>
          </Paper>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              In Progress
            </Typography>
            <Typography variant="h3" color="primary" sx={{ fontWeight: 'bold' }}>
              {inProgressUpdates.length}
            </Typography>
            <Typography variant="body2" color="textSecondary">
              Updates currently being applied
            </Typography>
          </Paper>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Completed
            </Typography>
            <Typography variant="h3" color="success" sx={{ fontWeight: 'bold' }}>
              {completedUpdates.length}
            </Typography>
            <Typography variant="body2" color="textSecondary">
              Successfully applied updates
            </Typography>
          </Paper>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Critical Updates
            </Typography>
            <Typography variant="h3" color="error" sx={{ fontWeight: 'bold' }}>
              {criticalUpdates.length}
            </Typography>
            <Typography variant="body2" color="textSecondary">
              High priority security updates
            </Typography>
          </Paper>
        </Grid>
      </Grid>
      
      <Tabs
        value={activeTab}
        onChange={handleTabChange}
        sx={{ mb: 3 }}
      >
        <Tab label="All Updates" />
        <Tab label="Pending" />
        <Tab label="In Progress" />
        <Tab label="Completed" />
        <Tab label="Critical" />
        <Tab label="By Target" />
      </Tabs>
      
      {filteredOtaUpdates.length === 0 ? (
        <Alert severity="info">
          No OTA updates found matching your criteria.
        </Alert>
      ) : (
        <Grid container spacing={3}>
          {filteredOtaUpdates
            .filter(update => {
              switch (activeTab) {
                case 1:
                  return update.status === 'Pending';
                case 2:
                  return update.status === 'Downloading' || 
                         update.status === 'Verifying' || 
                         update.status === 'Applying';
                case 3:
                  return update.status === 'Success' || 
                         update.status === 'Failed' || 
                         update.status === 'Rollback';
                case 4:
                  return update.priority === 'Critical';
                case 5:
                  return true; // Will be handled by target filter
                default:
                  return true;
              }
            })
            .map((update) => (
              <Grid item xs={12} sm={6} md={4} key={update.id}>
                <OtaUpdateCard update={update} />
              </Grid>
            ))}
        </Grid>
      )}
    </Box>
  );
};

const OtaUpdateCard = ({ update }) => {
  const getTargetIcon = (target) => {
    switch (target) {
      case 'Agent':
        return 'A';
      case 'Model':
        return 'M';
      case 'Config':
        return 'C';
      case 'Firmware':
        return 'F';
      default:
        return '?';
    }
  };

  const getPriorityColor = (priority) => {
    switch (priority) {
      case 'Critical':
        return 'error';
      case 'High':
        return 'warning';
      case 'Medium':
        return 'primary';
      case 'Low':
        return 'success';
      default:
        return 'default';
    }
  };

  const getStatusColor = (status) => {
    switch (status) {
      case 'Pending':
        return 'warning';
      case 'Downloading':
      case 'Verifying':
      case 'Applying':
        return 'primary';
      case 'Success':
        return 'success';
      case 'Failed':
      case 'Rollback':
        return 'error';
      default:
        return 'default';
    }
  };

  return (
    <Paper sx={{ p: 2, height: '100%' }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', mb: 2 }}>
        <Box>
          <Typography variant="h6" component="div">
            {update.version}
          </Typography>
          <Typography variant="body2" color="textSecondary">
            {update.target} Update
          </Typography>
        </Box>
        <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-end', gap: 1 }}>
          <StatusBadge status={update.priority} severity={update.priority} />
          <StatusBadge status={update.status} severity={update.status} />
        </Box>
      </Box>
      
      <Divider sx={{ my: 2 }} />
      
      <Grid container spacing={2}>
        <Grid item xs={6}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Target
          </Typography>
          <Chip 
            label={getTargetIcon(update.target)} 
            color="primary"
            size="small"
            sx={{ mr: 1 }}
          />
          <Typography variant="body2">
            {update.target}
          </Typography>
        </Grid>
        
        <Grid item xs={6}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Priority
          </Typography>
          <Chip 
            label={update.priority} 
            color={getPriorityColor(update.priority)}
            size="small"
          />
        </Grid>
        
        <Grid item xs={6}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Status
          </Typography>
          <Chip 
            label={update.status} 
            color={getStatusColor(update.status)}
            size="small"
          />
        </Grid>
        
        <Grid item xs={6}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Progress
          </Typography>
          <Typography variant="body2">
            {update.progress_percent.toFixed(0)}%
          </Typography>
        </Grid>
        
        <Grid item xs={12}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Description
          </Typography>
          <Typography variant="body2">
            {update.meta.description}
          </Typography>
        </Grid>
        
        <Grid item xs={6}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Created
          </Typography>
          <Typography variant="body2">
            {new Date(update.created_at).toLocaleString()}
          </Typography>
        </Grid>
        
        <Grid item xs={6}>
          {update.completed_at && (
            <>
              <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                Completed
              </Typography>
              <Typography variant="body2">
                {new Date(update.completed_at).toLocaleString()}
              </Typography>
            </>
          )}
        </Grid>
      </Grid>
    </Paper>
  );
};

export default OtaUpdatesList;