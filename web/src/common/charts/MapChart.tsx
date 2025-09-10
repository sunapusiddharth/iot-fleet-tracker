import { useState, useEffect } from 'react';
import ReactMapGL, { Marker, Popup } from 'react-map-gl';
import ViewportProps from "react-map-gl"
import {
  Box,
  Paper,
  Typography,
  Button,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material';
import { LocationOn as LocationOnIcon } from '@mui/icons-material';

interface Truck {
  id: string;
  truck_id: string;
  make: string;
  model: string;
  license_plate: string;
  status: 'Online' | 'Maintenance' | 'Offline';
  speed_kmh?: number;
  health_score: number;
  location: [longitude: number, latitude: number];
}

interface Trip {
  id: string;
  // Add other trip-related fields here
}

interface MapChartProps {
  trucks?: Truck[];
  trips?: Trip[];
  selectedTruckId?: string;
}

const MapChart: React.FC<MapChartProps> = ({
  trucks = [],
  trips = [],
  selectedTruckId,
}) => {
  const [viewport, setViewport] = useState<typeof ViewportProps>({
    latitude: 37.7749,
    longitude: -122.4194,
    zoom: 10,
    width: '100%',
    height: '100%',
  });

  const [selectedTruck, setSelectedTruck] = useState<Truck | null>(null);
  const [mapStyle, setMapStyle] = useState<string>(
    'mapbox://styles/mapbox/streets-v11'
  );

  useEffect(() => {
    if (trucks.length > 0) {
      setViewport((prev) => ({
        ...prev,
        latitude: trucks[0].location[1],
        longitude: trucks[0].location[0],
        zoom: 12,
      }));
    }
  }, [trucks]);

  const handleTruckClick = (truck: Truck) => {
    setSelectedTruck(truck);
    setViewport((prev) => ({
      ...prev,
      latitude: truck.location[1],
      longitude: truck.location[0],
      zoom: 15,
    }));
  };

  const handleMapStyleChange = (
    event: React.ChangeEvent<{ value: unknown }>
  ) => {
    setMapStyle(event.target.value as string);
  };

  return (
    <Box sx={{ position: 'relative', height: '100%' }}>
      <ReactMapGL
        {...viewport}
        onViewportChange={setViewport}
        mapboxApiAccessToken="your-mapbox-access-token"
        mapStyle={mapStyle}
      >
        {trucks.map((truck) => (
          <Marker
            key={truck.id}
            latitude={truck.location[1]}
            longitude={truck.location[0]}
            onClick={() => handleTruckClick(truck)}
          >
            <LocationOnIcon
              sx={{
                color:
                  truck.status === 'Online'
                    ? 'green'
                    : truck.status === 'Maintenance'
                    ? 'orange'
                    : 'red',
                fontSize: 40,
                transform: 'translate(-50%, -100%)',
                cursor: 'pointer',
                '&:hover': {
                  transform: 'translate(-50%, -100%) scale(1.2)',
                },
              }}
            />
          </Marker>
        ))}

        {selectedTruck && (
          <Popup
            latitude={selectedTruck.location[1]}
            longitude={selectedTruck.location[0]}
            onClose={() => setSelectedTruck(null)}
            closeOnClick={false}
          >
            <Paper sx={{ p: 2, minWidth: 200 }}>
              <Typography variant="h6" gutterBottom>
                {selectedTruck.truck_id}
              </Typography>
              <Typography variant="body2" color="textSecondary" gutterBottom>
                {selectedTruck.make} {selectedTruck.model}
              </Typography>
              <Typography variant="body2" gutterBottom>
                License: {selectedTruck.license_plate}
              </Typography>
              <Typography variant="body2" gutterBottom>
                Status: <strong>{selectedTruck.status}</strong>
              </Typography>
              <Typography variant="body2" gutterBottom>
                Speed: {selectedTruck.speed_kmh || 0} km/h
              </Typography>
              <Typography variant="body2" gutterBottom>
                Health: {selectedTruck.health_score.toFixed(0)}%
              </Typography>
              <Button
                size="small"
                variant="outlined"
                onClick={() => {
                  // Navigate to truck detail page
                }}
                sx={{ mt: 1 }}
              >
                View Details
              </Button>
            </Paper>
          </Popup>
        )}

        {trips.length > 0 && (
          // Add trip routes here using react-map-gl LineLayer or similar
          <></>
        )}
      </ReactMapGL>

      <Box
        sx={{
          position: 'absolute',
          top: 10,
          right: 10,
          zIndex: 1,
          bgcolor: 'background.paper',
          p: 1,
          borderRadius: 1,
          boxShadow: 1,
        }}
      >
        <FormControl size="small">
          <InputLabel>Map Style</InputLabel>
          <Select
            value={mapStyle}
            onChange={handleMapStyleChange}
            label="Map Style"
            sx={{ minWidth: 150 }}
          >
            <MenuItem value="mapbox://styles/mapbox/streets-v11">Streets</MenuItem>
            <MenuItem value="mapbox://styles/mapbox/satellite-v9">Satellite</MenuItem>
            <MenuItem value="mapbox://styles/mapbox/light-v10">Light</MenuItem>
            <MenuItem value="mapbox://styles/mapbox/dark-v10">Dark</MenuItem>
            <MenuItem value="mapbox://styles/mapbox/navigation-day-v1">Navigation Day</MenuItem>
            <MenuItem value="mapbox://styles/mapbox/navigation-night-v1">Navigation Night</MenuItem>
          </Select>
        </FormControl>
      </Box>
    </Box>
  );
};

export default MapChart;
