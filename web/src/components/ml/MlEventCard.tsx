import React from 'react';
import {
  Card,
  CardContent,
  CardActions,
  Typography,
  Box,
  Chip,
  Avatar,
  Button,
  Divider,
  LinearProgress,
  Tooltip,
  Grid,
} from '@mui/material';
import {
  Psychology as PsychologyIcon,
  TrendingUp as TrendingUpIcon,
  Warning as WarningIcon,
  CheckCircle as CheckCircleIcon,
} from '@mui/icons-material';
import { StatusBadge } from '../common/StatusBadge';
import { Link } from 'react-router-dom';

interface MlEventMeta {
  location?: [number, number];
  [key: string]: any;
}

interface MlEvent {
  id: string;
  model_name: string;
  result_type: string;
  is_alert: boolean;
  confidence: number;
  latency_ms: number;
  timestamp: string;
  hardware_used: string;
  truck_license_plate: string;
  truck_model: string;
  truck_make: string;
  meta?: MlEventMeta;
}

interface MlEventCardProps {
  event: MlEvent;
}

const MlEventCard: React.FC<MlEventCardProps> = ({ event }) => {
  const getModelIcon = (modelName: string) => {
    switch (modelName) {
      case 'drowsiness':
        return <PsychologyIcon />;
      case 'lane_departure':
        return <TrendingUpIcon />;
      case 'cargo_tamper':
        return <WarningIcon />;
      case 'license_plate':
        return <CheckCircleIcon />;
      case 'weather':
        return <PsychologyIcon />;
      default:
        return <PsychologyIcon />;
    }
  };

  const getModelColor = (modelName: string): 'error' | 'warning' | 'success' | 'info' | 'primary' => {
    switch (modelName) {
      case 'drowsiness':
        return 'error';
      case 'lane_departure':
        return 'warning';
      case 'cargo_tamper':
        return 'error';
      case 'license_plate':
        return 'success';
      case 'weather':
        return 'info';
      default:
        return 'primary';
    }
  };

  const formatModelName = (modelName: string): string => {
    const map: Record<string, string> = {
      drowsiness: 'Drowsiness Detection',
      lane_departure: 'Lane Departure',
      cargo_tamper: 'Cargo Tamper',
      license_plate: 'License Plate Recognition',
      weather: 'Weather Classification',
    };
    return map[modelName] || modelName;
  };

  const formatResultType = (resultType: string): string => {
    if (resultType.includes('Drowsiness')) return 'Drowsy Driver';
    if (resultType.includes('LaneDeparture')) return 'Lane Departure';
    if (resultType.includes('CargoTamper')) return 'Cargo Tamper';
    if (resultType.includes('LicensePlate')) return 'License Plate';
    if (resultType.includes('Weather')) return 'Weather';
    return resultType;
  };

  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', mb: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
            <Avatar sx={{ bgcolor: getModelColor(event.model_name) }}>
              {getModelIcon(event.model_name)}
            </Avatar>
            <Box>
              <Typography variant="h6">{formatModelName(event.model_name)}</Typography>
              <Typography variant="body2" color="textSecondary">
                {formatResultType(event.result_type)}
              </Typography>
            </Box>
          </Box>
          <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-end', gap: 1 }}>
            <StatusBadge
              status={event.is_alert ? 'Alert' : 'Normal'}
              severity={event.is_alert ? 'Critical' : 'Info'}
            />
            <Chip
              label={`${(event.confidence * 100).toFixed(1)}%`}
              color={
                event.confidence > 0.9
                  ? 'success'
                  : event.confidence > 0.8
                  ? 'warning'
                  : 'default'
              }
              size="small"
            />
          </Box>
        </Box>

        <Divider sx={{ my: 2 }} />

        <Grid container spacing={2}>
          <Grid item xs={12} sm={6}>
            <Box sx={{ mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Truck:</Typography>
              <Typography variant="body2">
                {event.truck_license_plate} ({event.truck_model} {event.truck_make})
              </Typography>
            </Box>

            <Box sx={{ mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Confidence:</Typography>
              <Box sx={{ width: '100%' }}>
                <LinearProgress
                  variant="determinate"
                  value={event.confidence * 100}
                  color={
                    event.confidence > 0.9
                      ? 'success'
                      : event.confidence > 0.8
                      ? 'warning'
                      : 'primary'
                  }
                  sx={{ height: 8, borderRadius: 4, mb: 1 }}
                />
                <Typography variant="body2">
                  {(event.confidence * 100).toFixed(1)}%
                </Typography>
              </Box>
            </Box>

            <Box sx={{ mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Latency:</Typography>
              <Typography variant="body2">
                {event.latency_ms.toFixed(1)} ms
              </Typography>
            </Box>
          </Grid>

          <Grid item xs={12} sm={6}>
            <Box sx={{ mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Triggered:</Typography>
              <Typography variant="body2">
                {new Date(event.timestamp).toLocaleString()}
              </Typography>
            </Box>

            <Box sx={{ mb: 1 }}>
              <Typography variant="subtitle2" color="textSecondary">Hardware:</Typography>
              <Typography variant="body2">{event.hardware_used}</Typography>
            </Box>

            {event.meta?.location && (
              <Box>
                <Typography variant="subtitle2" color="textSecondary">Location:</Typography>
                <Typography variant="body2">
                  {event.meta.location[0].toFixed(6)}, {event.meta.location[1].toFixed(6)}
                </Typography>
              </Box>
            )}
          </Grid>
        </Grid>
      </CardContent>

      <CardActions sx={{ justifyContent: 'flex-end' }}>
        <Button
          size="small"
          component={Link}
          to={`/ml-events/${event.id}`}
          variant="outlined"
        >
          View Details
        </Button>
      </CardActions>
    </Card>
  );
};

export default MlEventCard;
