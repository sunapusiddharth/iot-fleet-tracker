import React from 'react';
import {
  Box,
  TextField,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Button,
  Grid,
  Typography,
  Chip,
} from '@mui/material';
import { DateRangePicker } from './DateRangePicker';
import { Close as CloseIcon } from '@mui/icons-material';
import { DateRange } from '@mui/x-date-pickers-pro';

type FilterType = 'text' | 'select' | 'dateRange';

interface FilterOption {
  label: string;
  value: string;
}

interface AvailableFilter {
  key: string;
  label: string;
  type: FilterType;
  options?: FilterOption[];
  defaultValue?: string | DateRange<Date>;
}

type FilterValue = string | DateRange<Date>;

interface FilterBarProps {
  filters: Record<string, FilterValue>;
  onFilterChange: (filters: Record<string, FilterValue>) => void;
  onApplyFilters: () => void;
  onResetFilters: () => void;
  availableFilters: AvailableFilter[];
}

const FilterBar: React.FC<FilterBarProps> = ({
  filters,
  onFilterChange,
  onApplyFilters,
  onResetFilters,
  availableFilters,
}) => {
  const [localFilters, setLocalFilters] = React.useState<Record<string, FilterValue>>(filters);

  const handleFilterChange = (key: string, value: FilterValue) => {
    setLocalFilters(prev => ({
      ...prev,
      [key]: value,
    }));
  };

  const handleApplyFilters = () => {
    onFilterChange(localFilters);
    onApplyFilters();
  };

  const handleResetFilters = () => {
    const resetFilters: Record<string, FilterValue> = {};
    availableFilters.forEach(filter => {
      resetFilters[filter.key] = filter.defaultValue || '';
    });
    setLocalFilters(resetFilters);
    onFilterChange(resetFilters);
    onResetFilters();
  };

  const activeFilters = Object.keys(localFilters).filter(
    key => {
      const value = localFilters[key];
      if (Array.isArray(value)) {
        return value[0] !== null || value[1] !== null;
      }
      return value !== '';
    }
  );

  return (
    <Box sx={{ mb: 3, p: 2, bgcolor: 'background.paper', borderRadius: 1 }}>
      <Typography variant="h6" gutterBottom>
        Filters
      </Typography>

      {activeFilters.length > 0 && (
        <Box sx={{ mb: 2 }}>
          <Typography variant="subtitle2" gutterBottom>
            Active Filters:
          </Typography>
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1 }}>
            {activeFilters.map(key => (
              <Chip
                key={key}
                label={`${key}: ${
                  Array.isArray(localFilters[key])
                    ? `${localFilters[key][0]?.toLocaleDateString() || ''} - ${localFilters[key][1]?.toLocaleDateString() || ''}`
                    : localFilters[key]
                }`}
                onDelete={() => {
                  const newFilters = { ...localFilters, [key]: '' };
                  setLocalFilters(newFilters);
                  onFilterChange(newFilters);
                }}
                deleteIcon={<CloseIcon />}
                size="small"
              />
            ))}
          </Box>
        </Box>
      )}

      <Grid container spacing={2}>
        {availableFilters.map(filter => (
          <Grid item xs={12} sm={6} md={4} key={filter.key}>
            <FormControl fullWidth size="small">
              <InputLabel>{filter.label}</InputLabel>
              {filter.type === 'select' ? (
                <Select
                  value={localFilters[filter.key] as string || ''}
                  onChange={(e) => handleFilterChange(filter.key, e.target.value)}
                  label={filter.label}
                >
                  <MenuItem value="">All</MenuItem>
                  {filter.options?.map(option => (
                    <MenuItem key={option.value} value={option.value}>
                      {option.label}
                    </MenuItem>
                  ))}
                </Select>
              ) : filter.type === 'dateRange' ? (
                <DateRangePicker
                  value={localFilters[filter.key] as DateRange<Date> || [null, null]}
                  onChange={(value) => handleFilterChange(filter.key, value)}
                />
              ) : (
                <TextField
                  value={localFilters[filter.key] as string || ''}
                  onChange={(e) => handleFilterChange(filter.key, e.target.value)}
                  label={filter.label}
                  size="small"
                  fullWidth
                />
              )}
            </FormControl>
          </Grid>
        ))}

        <Grid item xs={12}>
          <Box sx={{ display: 'flex', gap: 2, justifyContent: 'flex-end' }}>
            <Button
              variant="outlined"
              onClick={handleResetFilters}
              disabled={activeFilters.length === 0}
            >
              Reset Filters
            </Button>
            <Button
              variant="contained"
              onClick={handleApplyFilters}
              disabled={JSON.stringify(localFilters) === JSON.stringify(filters)}
            >
              Apply Filters
            </Button>
          </Box>
        </Grid>
      </Grid>
    </Box>
  );
};

export default FilterBar;
