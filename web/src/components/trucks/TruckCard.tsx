import React from 'react';
import {
  Card,
  CardContent,
  CardActions,
  Typography,
  Box,
  Chip,
  Avatar,
  LinearProgress,
  Button,
} from '@mui/material';
import { StatusBadge } from '../common/StatusBadge';
import { Link } from 'react-router-dom';
import { DirectionsCar as TruckIcon } from '@mui/icons-material';

type TruckStatus = 'Online' | 'Offline' | 'Maintenance' | string;

interface Truck {
  id: string;
  truck_id: string;
  make: string;
  model: string;
  year: number;
  license_plate: string;
  status: TruckStatus;
  health_score: number;
  speed_kmh?: number;
  location: [number, number]; // [longitude, latitude]
  last_seen: string;
}

interface TruckCardProps {
  truck: Truck;
}

const TruckCard: React.FC<TruckCardProps> = ({ truck }) => {
  const getHealthColor = (score: number): 'success' | 'warning' | 'error' => {
    if (score >= 80) return 'success';
    if (score >= 60) return 'warning';
    return 'error';
  };

  const getSpeedColor = (speed: number): 'success' | 'warning' | 'error' => {
    if (speed > 100) return 'error';
    if (speed > 80) return 'warning';
    return 'success';
  };

  return (
    <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      <CardContent sx={{ flexGrow: 1 }}>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <Avatar sx={{ bgcolor: 'primary.main' }}>
              <TruckIcon />
            </Avatar>
            <Typography variant="h6">{truck.truck_id}</Typography>
          </Box>
          <StatusBadge status={truck.status} />
        </Box>

        <Typography variant="subtitle1" color="textSecondary" gutterBottom>
          {truck.make} {truck.model} ({truck.year})
        </Typography>

        <Typography variant="h6" gutterBottom>
          {truck.license_plate}
        </Typography>

        <Box sx={{ mt: 2 }}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Health Score
          </Typography>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
            <Typography variant="body2">{truck.health_score.toFixed(0)}%</Typography>
            <Chip
              label={`${truck.health_score.toFixed(0)}%`}
              color={getHealthColor(truck.health_score)}
              size="small"
            />
          </Box>
          <LinearProgress
            variant="determinate"
            value={truck.health_score}
            color={getHealthColor(truck.health_score)}
            sx={{ height: 8, borderRadius: 4 }}
          />
        </Box>

        <Box sx={{ mt: 2 }}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Current Speed
          </Typography>
          <Typography variant="h5" color={getSpeedColor(truck.speed_kmh || 0)}>
            {truck.speed_kmh || 0} km/h
          </Typography>
        </Box>

        <Box sx={{ mt: 2 }}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Location
          </Typography>
          <Typography variant="body2">
            Lat: {truck.location[1].toFixed(6)}, Lng: {truck.location[0].toFixed(6)}
          </Typography>
        </Box>

        <Box sx={{ mt: 2 }}>
          <Typography variant="subtitle2" color="textSecondary" gutterBottom>
            Last Seen
          </Typography>
          <Typography variant="body2">
            {new Date(truck.last_seen).toLocaleString()}
          </Typography>
        </Box>
      </CardContent>

      <CardActions>
        <Button
          size="small"
          component={Link}
          to={`/trucks/${truck.id}`}
          variant="contained"
          fullWidth
        >
          View Details
        </Button>
      </CardActions>
    </Card>
  );
};

export default TruckCard;
