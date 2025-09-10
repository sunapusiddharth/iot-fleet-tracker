import React from 'react';
import {
  Paper,
  Typography,
  List,
  ListItem,
  ListItemText,
  ListItemIcon,
  Divider,
  Box,
  Avatar,
} from '@mui/material';
import {
  Warning as WarningIcon,
  Error as ErrorIcon,
  Info as InfoIcon,
} from '@mui/icons-material';
import { StatusBadge } from '../common/StatusBadge';
import { Link } from 'react-router-dom';

type Severity = 'Critical' | 'Emergency' | 'Warning' | 'Info';
type Status = 'Triggered' | 'Acknowledged' | 'Resolved' | 'Suppressed';

interface Alert {
  id: string;
  severity: Severity;
  status: Status;
  message: string;
  triggered_at: string;
  truck_license_plate: string;
  truck_model: string;
  truck_make: string;
}

interface RecentAlertsProps {
  alerts: Alert[];
}

const RecentAlerts: React.FC<RecentAlertsProps> = ({ alerts }) => {
  const getAlertIcon = (severity: Severity): JSX.Element => {
    switch (severity) {
      case 'Critical':
      case 'Emergency':
        return <ErrorIcon color="error" />;
      case 'Warning':
        return <WarningIcon color="warning" />;
      case 'Info':
      default:
        return <InfoIcon color="info" />;
    }
  };

  const getAlertColor = (severity: Severity): 'error' | 'warning' | 'info' | 'default' => {
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

  return (
    <Paper sx={{ p: 2 }}>
      <Typography variant="h6" gutterBottom>
        Recent Alerts
      </Typography>
      <List>
        {alerts.map((alert, index) => (
          <React.Fragment key={alert.id}>
            {index > 0 && <Divider component="li" />}
            <ListItem
              component={Link}
              to={`/alerts/${alert.id}`}
              sx={{
                textDecoration: 'none',
                color: 'inherit',
                '&:hover': {
                  backgroundColor: 'action.hover',
                },
              }}
            >
              <ListItemIcon>
                <Avatar sx={{ bgcolor: getAlertColor(alert.severity) }}>
                  {getAlertIcon(alert.severity)}
                </Avatar>
              </ListItemIcon>
              <ListItemText
                primary={
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                    <Typography variant="subtitle1">
                      {alert.message}
                    </Typography>
                    <StatusBadge status={alert.severity} severity={alert.severity} />
                  </Box>
                }
                secondary={
                  <Box>
                    <Typography variant="body2" color="textSecondary">
                      Truck: {alert.truck_license_plate} ({alert.truck_model} {alert.truck_make})
                    </Typography>
                    <Typography variant="caption" color="textSecondary">
                      {new Date(alert.triggered_at).toLocaleString()}
                    </Typography>
                  </Box>
                }
              />
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <StatusBadge status={alert.status} />
              </Box>
            </ListItem>
          </React.Fragment>
        ))}
      </List>
    </Paper>
  );
};

export default RecentAlerts;
