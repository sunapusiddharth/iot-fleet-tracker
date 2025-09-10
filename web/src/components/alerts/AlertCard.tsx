import React from 'react';
import {
  Card,
  CardContent,
  CardActions,
  Typography,
  Box,
  Avatar,
  Button,
  Divider,
  Grid,
} from '@mui/material';
import {
  Warning as WarningIcon,
  Error as ErrorIcon,
  Info as InfoIcon,
  Speed as SpeedIcon,
  Psychology as PsychologyIcon,
  LocationOn as LocationOnIcon,
} from '@mui/icons-material';
import { StatusBadge } from '../../common/StatusBadge';
import { Link } from 'react-router-dom';

type AlertSeverity = 'Info' | 'Warning' | 'Critical' | 'Emergency';
type AlertStatus = 'Triggered' | 'Acknowledged' | 'Resolved';

interface Alert {
  id: string;
  alert_type: string;
  severity: AlertSeverity;
  status: AlertStatus;
  message: string;
  triggered_at: string;
  acknowledged_at?: string;
  resolved_at?: string;
  truck_license_plate: string;
  truck_model: string;
  truck_make: string;
  source: string;
  context?: Record<string, any>;
}

interface AlertCardProps {
  alert: Alert;
}

const AlertCard: React.FC<AlertCardProps> = ({ alert }) => {
  const getAlertIcon = (type: string): JSX.Element => {
    switch (type) {
      case 'DrowsyDriver':
        return <PsychologyIcon />;
      case 'LaneDeparture':
      case 'HarshBraking':
      case 'RapidAcceleration':
      case 'OverSpeeding':
        return <SpeedIcon />;
      case 'CargoTamper':
        return <LocationOnIcon />;
      case 'HighTemperature':
        return <ErrorIcon />;
      case 'LowDiskSpace':
        return <WarningIcon />;
      default:
        return <InfoIcon />;
    }
  };

  const getAlertColor = (severity: AlertSeverity): 'error' | 'warning' | 'info' | 'default' => {
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

  const formatAlertType = (type: string): string => {
    const map: Record<string, string> = {
      DrowsyDriver: 'Drowsy Driver',
      LaneDeparture: 'Lane Departure',
      CargoTamper: 'Cargo Tamper',
      HarshBraking: 'Harsh Braking',
      RapidAcceleration: 'Rapid Acceleration',
      OverSpeeding: 'Over Speeding',
      HighTemperature: 'High Temperature',
      LowDiskSpace: 'Low Disk Space',
    };
    return map[type] || type;
  };

  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', mb: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
            <Avatar sx={{ bgcolor: getAlertColor(alert.severity) }}>
              {getAlertIcon(alert.alert_type)}
            </Avatar>
            <Box>
              <Typography variant="h6">{formatAlertType(alert.alert_type)}</Typography>
              <Typography variant="body2" color="textSecondary">
                {alert.message}
              </Typography>
            </Box>
          </Box>
          <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-end', gap: 1 }}>
            <StatusBadge status={alert.severity} severity={alert.severity} />
            <StatusBadge status={alert.status} />
          </Box>
        </Box>

        <Divider sx={{ my: 2 }} />

        <Grid container spacing={2}>
          <Grid item xs={12} sm={6}>
            <Box sx={{ mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Truck:</Typography>
              <Typography variant="body2">
                {alert.truck_license_plate} ({alert.truck_model} {alert.truck_make})
              </Typography>
            </Box>

            <Box sx={{ mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Triggered:</Typography>
              <Typography variant="body2">
                {new Date(alert.triggered_at).toLocaleString()}
              </Typography>
            </Box>

            {alert.acknowledged_at && (
              <Box sx={{ mb: 1 }}>
                <Typography variant="subtitle2" color="textSecondary">Acknowledged:</Typography>
                <Typography variant="body2">
                  {new Date(alert.acknowledged_at).toLocaleString()}
                </Typography>
              </Box>
            )}

            {alert.resolved_at && (
              <Box>
                <Typography variant="subtitle2" color="textSecondary">Resolved:</Typography>
                <Typography variant="body2">
                  {new Date(alert.resolved_at).toLocaleString()}
                </Typography>
              </Box>
            )}
          </Grid>

          <Grid item xs={12} sm={6}>
            <Box sx={{ mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Source:</Typography>
              <Typography variant="body2">{alert.source}</Typography>
            </Box>

            {alert.context && (
              <>
                <Typography variant="subtitle2" color="textSecondary" gutterBottom>
                  Context:
                </Typography>
                <Box
                  sx={{
                    maxHeight: 120,
                    overflowY: 'auto',
                    p: 1,
                    bgcolor: 'background.paper',
                    borderRadius: 1,
                    border: '1px solid',
                    borderColor: 'divider',
                  }}
                >
                  <pre style={{ margin: 0, fontSize: '0.875rem' }}>
                    {JSON.stringify(alert.context, null, 2)}
                  </pre>
                </Box>
              </>
            )}
          </Grid>
        </Grid>
      </CardContent>

      <CardActions sx={{ justifyContent: 'flex-end' }}>
        <Button
          size="small"
          component={Link}
          to={`/alerts/${alert.id}`}
          variant="outlined"
        >
          View Details
        </Button>
        {alert.status === 'Triggered' && (
          <Button size="small" variant="contained" color="primary">
            Acknowledge
          </Button>
        )}
      </CardActions>
    </Card>
  );
};

export default AlertCard;
