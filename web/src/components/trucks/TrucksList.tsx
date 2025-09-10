import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Grid,
  Card,
  CardContent,
  CardActions,
  Button,
  CircularProgress,
  Alert,
  TextField,
  InputAdornment,
} from '@mui/material';
import { Search as SearchIcon } from '@mui/icons-material';
import { useTrucks } from '../../hooks/useTrucks';
import { TruckCard } from './TruckCard';
import { FilterBar } from '../common/FilterBar';
import { Link } from 'react-router-dom';

const TrucksList = () => {
  const { trucks, loading, error, fetchTrucks } = useTrucks();
  const [filteredTrucks, setFilteredTrucks] = useState([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [filters, setFilters] = useState({
    status: '',
    make: '',
    model: '',
  });

  useEffect(() => {
    fetchTrucks();
  }, [fetchTrucks]);

  useEffect(() => {
    let filtered = trucks;
    
    // Apply search filter
    if (searchTerm) {
      filtered = filtered.filter(truck =>
        truck.truck_id.toLowerCase().includes(searchTerm.toLowerCase()) ||
        truck.license_plate.toLowerCase().includes(searchTerm.toLowerCase()) ||
        truck.make.toLowerCase().includes(searchTerm.toLowerCase()) ||
        truck.model.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    // Apply status filter
    if (filters.status) {
      filtered = filtered.filter(truck => truck.status === filters.status);
    }
    
    // Apply make filter
    if (filters.make) {
      filtered = filtered.filter(truck => truck.make === filters.make);
    }
    
    // Apply model filter
    if (filters.model) {
      filtered = filtered.filter(truck => truck.model === filters.model);
    }
    
    setFilteredTrucks(filtered);
  }, [trucks, searchTerm, filters]);

  const availableFilters = [
    {
      key: 'status',
      label: 'Status',
      type: 'select',
      options: [
        { value: 'Online', label: 'Online' },
        { value: 'Offline', label: 'Offline' },
        { value: 'Maintenance', label: 'Maintenance' },
      ],
    },
    {
      key: 'make',
      label: 'Make',
      type: 'select',
      options: [
        { value: 'Volvo', label: 'Volvo' },
        { value: 'Scania', label: 'Scania' },
        { value: 'Mercedes', label: 'Mercedes' },
        { value: 'MAN', label: 'MAN' },
      ],
    },
    {
      key: 'model',
      label: 'Model',
      type: 'select',
      options: [
        { value: 'FH16', label: 'FH16' },
        { value: 'R-series', label: 'R-series' },
        { value: 'Actros', label: 'Actros' },
        { value: 'TGX', label: 'TGX' },
      ],
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
        Error loading trucks. Please try again later.
      </Alert>
    );
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4">
          Trucks ({filteredTrucks.length})
        </Typography>
        <Button
          variant="contained"
          component={Link}
          to="/trucks/new"
        >
          Add New Truck
        </Button>
      </Box>
      
      <Box sx={{ mb: 3 }}>
        <TextField
          fullWidth
          placeholder="Search trucks by ID, license plate, make, or model..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          InputProps={{
            startAdornment: (
              <InputAdornment position="start">
                <SearchIcon />
              </InputAdornment>
            ),
          }}
          size="small"
        />
      </Box>
      
      <FilterBar
        filters={filters}
        onFilterChange={setFilters}
        onApplyFilters={() => {}}
        onResetFilters={() => {}}
        availableFilters={availableFilters}
      />
      
      {filteredTrucks.length === 0 ? (
        <Alert severity="info">
          No trucks found matching your criteria.
        </Alert>
      ) : (
        <Grid container spacing={3}>
          {filteredTrucks.map((truck) => (
            <Grid item xs={12} sm={6} md={4} lg={3} key={truck.id}>
              <TruckCard truck={truck} />
            </Grid>
          ))}
        </Grid>
      )}
    </Box>
  );
};

export default TrucksList;