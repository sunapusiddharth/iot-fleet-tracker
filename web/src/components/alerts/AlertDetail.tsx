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
  Breadcrumbs,
  Link as MuiLink,
  Chip,
  Avatar,
  Divider,
  TextField,
  MenuItem,
  Select,
  FormControl,
  InputLabel,
} from '@mui/material';
import {
  Warning as WarningIcon,
  Error as ErrorIcon,
  Info as InfoIcon,
  CheckCircle as CheckCircleIcon,
  Schedule as ScheduleIcon,
  LocationOn as LocationOnIcon,
  Speed as SpeedIcon,
  Psychology as PsychologyIcon,
  Edit as EditIcon,
  History as HistoryIcon,
} from '@mui/icons-material';
import { useParams, Link } from 'react-router-dom';
import { useAlerts } from '../../hooks/useAlerts';
import { StatusBadge } from '../../common/StatusBadge';

const AlertDetail = () => {
  const { id } = useParams();
  const { alert, loading, error, fetchAlert, acknowledgeAlert, resolveAlert } = useAlerts();
  const [activeTab, setActiveTab] = useState(0);
  const [resolutionNotes, setResolutionNotes] = useState('');

  useEffect(() => {
    fetchAlert(id);
  }, [id, fetchAlert]);

  const handleTabChange = (event, newValue) => {
    setActiveTab(newValue);
  };

  const handleAcknowledge = async () => {
    try {
      await acknowledgeAlert(id);
      fetchAlert(id); // Refresh alert data
    } catch (error) {
      console.error('Error acknowledging alert:', error);
    }
  };

  const handleResolve = async () => {
    try {
      // In production, you would pass resolution notes to the API
      await resolveAlert(id);
      fetchAlert(id); // Refresh alert data
    } catch (error) {
      console.error('Error resolving alert:', error);
    }
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
        Error loading alert details. Please try again later.
      </Alert>
    );
  }

  if (!alert) {
    return (
      <Alert severity="info">
        Alert not found.
      </Alert>
    );
  }

  const getAlertIcon = (type) => {
    switch (type) {
      case 'DrowsyDriver':
        return <PsychologyIcon />;
      case 'LaneDeparture':
        return <SpeedIcon />;
      case 'CargoTamper':
        return <LocationOnIcon />;
      case 'HarshBraking':
        return <SpeedIcon />;
      case 'RapidAcceleration':
        return <SpeedIcon />;
      case 'OverSpeeding':
        return <SpeedIcon />;
      case 'HighTemperature':
        return <ErrorIcon />;
      case 'LowDiskSpace':
        return <WarningIcon />;
      default:
        return <InfoIcon />;
    }
  };

  const getAlertColor = (severity) => {
    switch (severity) {
      case 'Critical':
      case 'Emergency':
        return 'error';
      case 'Warning':
        return 'warning';
      case 'Info':
        return 'info';
      default:
        return 'default';
    }
  };

  const formatAlertType = (type) => {
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

  return (
    <Box>
      <Breadcrumbs aria-label="breadcrumb" sx={{ mb: 3 }}>
        <MuiLink component={Link} to="/" color="inherit">
          Dashboard
        </MuiLink>
        <MuiLink component={Link} to="/alerts" color="inherit">
          Alerts
        </MuiLink>
        <Typography color="text.primary">{formatAlertType(alert.alert_type)}</Typography>
      </Breadcrumbs>
      
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', mb: 3 }}>
        <Box>
          <Typography variant="h4" component="div">
            {formatAlertType(alert.alert_type)}
          </Typography>
          <Typography variant="subtitle1" color="textSecondary" gutterBottom>
            {alert.message}
          </Typography>
        </Box>
        <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-end', gap: 1 }}>
          <StatusBadge status={alert.severity} severity={alert.severity} />
          <StatusBadge status={alert.status} />
        </Box>
      </Box>
      
      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Alert Details
            </Typography>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, mb: 2 }}>
              <Avatar sx={{ bgcolor: getAlertColor(alert.severity) }}>
                {getAlertIcon(alert.alert_type)}
              </Avatar>
              <Box>
                <Typography variant="h6" component="div">
                  {formatAlertType(alert.alert_type)}
                </Typography>
                <Typography variant="body2" color="textSecondary">
                  {alert.message}
                </Typography>
              </Box>
            </Box>
            
            <Divider sx={{ my: 2 }} />
            
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                  Truck Information
                </Typography>
                <Typography variant="body2">
                  {alert.truck_license_plate} ({alert.truck_model} {alert.truck_make})
                </Typography>
              </Grid>
              <Grid item xs={6}>
                <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                  Alert ID
                </Typography>
                <Typography variant="body2">
                  {alert.alert_id}
                </Typography>
              </Grid>
              <Grid item xs={6}>
                <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                  Source
                </Typography>
                <Typography variant="body2">
                  {alert.source}
                </Typography>
              </Grid>
              <Grid item xs={6}>
                <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                  Triggered At
                </Typography>
                <Typography variant="body2">
                  {new Date(alert.triggered_at).toLocaleString()}
                </Typography>
              </Grid>
              {alert.acknowledged_at && (
                <Grid item xs={6}>
                  <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                    Acknowledged At
                  </Typography>
                  <Typography variant="body2">
                    {new Date(alert.acknowledged_at).toLocaleString()}
                  </Typography>
                </Grid>
              )}
              {alert.resolved_at && (
                <Grid item xs={6}>
                  <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                    Resolved At
                  </Typography>
                  <Typography variant="body2">
                    {new Date(alert.resolved_at).toLocaleString()}
                  </Typography>
                </Grid>
              )}
            </Grid>
          </Paper>
        </Grid>
        
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2, height: '100%' }}>
            <Typography variant="h6" gutterBottom>
              Context & Metadata
            </Typography>
            {alert.context && Object.keys(alert.context).length > 0 ? (
              <Box sx={{ 
                maxHeight: 300, 
                overflowY: 'auto', 
                p: 2, 
                bgcolor: 'background.paper', 
                borderRadius: 1,
                border: '1px solid',
                borderColor: 'divider'
              }}>
                {Object.entries(alert.context).map(([key, value]) => (
                  <Box key={key} sx={{ mb: 2 }}>
                    <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                      {key}
                    </Typography>
                    <Typography variant="body2">
                      {typeof value === 'object' ? JSON.stringify(value, null, 2) : value.toString()}
                    </Typography>
                  </Box>
                ))}
              </Box>
            ) : (
              <Typography variant="body2" color="textSecondary">
                No context available
              </Typography>
            )}
          </Paper>
        </Grid>
      </Grid>
      
      <Tabs
        value={activeTab}
        onChange={handleTabChange}
        sx={{ mb: 3 }}
      >
        <Tab label="Actions" />
        <Tab label="History" />
        <Tab label="Related Events" />
      </Tabs>
      
      {activeTab === 0 && (
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>
            Alert Actions
          </Typography>
          
          {alert.status === 'Triggered' && (
            <Box sx={{ mb: 3 }}>
              <Button
                variant="contained"
                color="primary"
                onClick={handleAcknowledge}
                startIcon={<CheckCircleIcon />}
                sx={{ mr: 2 }}
              >
                Acknowledge Alert
              </Button>
              <Button
                variant="contained"
                color="success"
                onClick={handleResolve}
                startIcon={<CheckCircleIcon />}
              >
                Resolve Alert
              </Button>
            </Box>
          )}
          
          {alert.status === 'Acknowledged' && (
            <Box sx={{ mb: 3 }}>
              <Button
                variant="contained"
                color="success"
                onClick={handleResolve}
                startIcon={<CheckCircleIcon />}
              >
                Resolve Alert
              </Button>
            </Box>
          )}
          
          <Box>
            <Typography variant="subtitle2" gutterBottom>
              Resolution Notes
            </Typography>
            <TextField
              fullWidth
              multiline
              rows={4}
              placeholder="Add resolution notes here..."
              value={resolutionNotes}
              onChange={(e) => setResolutionNotes(e.target.value)}
              disabled={alert.status === 'Resolved'}
            />
          </Box>
        </Paper>
      )}
      
      {activeTab === 1 && (
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>
            Alert History
          </Typography>
          <Box sx={{ 
            maxHeight: 400, 
            overflowY: 'auto', 
            p: 2, 
            bgcolor: 'background.paper', 
            borderRadius: 1,
            border: '1px solid',
            borderColor: 'divider'
          }}>
            <Box sx={{ mb: 2, p: 2, bgcolor: 'action.hover', borderRadius: 1 }}>
              <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                Alert Created
              </Typography>
              <Typography variant="body2">
                {new Date(alert.created_at).toLocaleString()}
              </Typography>
            </Box>
            
            {alert.acknowledged_at && (
              <Box sx={{ mb: 2, p: 2, bgcolor: 'action.hover', borderRadius: 1 }}>
                <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                  Alert Acknowledged
                </Typography>
                <Typography variant="body2">
                  {new Date(alert.acknowledged_at).toLocaleString()}
                </Typography>
                {alert.acknowledged_by && (
                  <Typography variant="body2" color="textSecondary">
                    By: {alert.acknowledged_by}
                  </Typography>
                )}
              </Box>
            )}
            
            {alert.resolved_at && (
              <Box sx={{ p: 2, bgcolor: 'action.hover', borderRadius: 1 }}>
                <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                  Alert Resolved
                </Typography>
                <Typography variant="body2">
                  {new Date(alert.resolved_at).toLocaleString()}
                </Typography>
                {alert.resolved_by && (
                  <Typography variant="body2" color="textSecondary">
                    By: {alert.resolved_by}
                  </Typography>
                )}
                {alert.resolution_notes && (
                  <Typography variant="body2" color="textSecondary" sx={{ mt: 1 }}>
                    Notes: {alert.resolution_notes}
                  </Typography>
                )}
              </Box>
            )}
          </Box>
        </Paper>
      )}
      
      {activeTab === 2 && (
        <Paper sx={{ p: 2 }}>
          <Typography variant="h6" gutterBottom>
            Related Events
          </Typography>
          <Typography variant="body2" color="textSecondary">
            This feature will show related telemetry, ML events, and health status around the time of this alert.
          </Typography>
          {/* In production, you would fetch and display related events here */}
        </Paper>
      )}
    </Box>
  );
};

export default AlertDetail;