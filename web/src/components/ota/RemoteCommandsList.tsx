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
  Chip,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from '@mui/material';
import { FilterBar } from '../common/FilterBar';
import { StatusBadge } from '../common/StatusBadge';
import { useOta } from '../../hooks/useOta';
import { DateRange } from '@mui/x-date-pickers-pro';

type CommandStatus =
  | 'Pending'
  | 'Executing'
  | 'Success'
  | 'Failed'
  | 'Timeout'
  | 'Cancelled';

type CommandType =
  | 'Reboot'
  | 'Shutdown'
  | 'RestartService'
  | 'GetDiagnostics'
  | 'UpdateConfig'
  | 'RunHealthCheck'
  | 'CaptureSnapshot'
  | 'FlushWAL';

interface RemoteCommand {
  id: string;
  command_id: string;
  command_type: CommandType;
  truck_id?: string;
  status: CommandStatus;
  issued_at: string;
  completed_at?: string;
  parameters: Record<string, any>;
}

interface Filters {
  commandType: string;
  status: string;
  truckId: string;
  dateRange: DateRange<Date>;
}

const RemoteCommandsList: React.FC = () => {
  const { remoteCommands, loading, error, fetchRemoteCommands } = useOta() as {
    remoteCommands: RemoteCommand[];
    loading: boolean;
    error: boolean;
    fetchRemoteCommands: () => void;
  };

  const [activeTab, setActiveTab] = useState<number>(0);
  const [filters, setFilters] = useState<Filters>({
    commandType: '',
    status: '',
    truckId: '',
    dateRange: [null, null],
  });

  useEffect(() => {
    fetchRemoteCommands();
  }, [fetchRemoteCommands]);

  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  const filteredCommands = remoteCommands.filter(command => {
    if (filters.commandType && command.command_type !== filters.commandType) return false;
    if (filters.status && command.status !== filters.status) return false;
    if (filters.truckId && command.truck_id !== filters.truckId) return false;
    if (filters.dateRange[0] && new Date(command.issued_at) < filters.dateRange[0]) return false;
    if (filters.dateRange[1] && new Date(command.issued_at) > filters.dateRange[1]) return false;
    return true;
  });

  const pendingCommands = filteredCommands.filter(c => c.status === 'Pending');
  const executingCommands = filteredCommands.filter(c => c.status === 'Executing');
  const completedCommands = filteredCommands.filter(c =>
    ['Success', 'Failed', 'Timeout', 'Cancelled'].includes(c.status)
  );
  const failedCommands = filteredCommands.filter(c =>
    ['Failed', 'Timeout'].includes(c.status)
  );

  const availableFilters = [
    {
      key: 'commandType',
      label: 'Command Type',
      type: 'select',
      options: [
        'Reboot',
        'Shutdown',
        'RestartService',
        'GetDiagnostics',
        'UpdateConfig',
        'RunHealthCheck',
        'CaptureSnapshot',
        'FlushWAL',
      ].map(v => ({ value: v, label: v })),
    },
    {
      key: 'status',
      label: 'Status',
      type: 'select',
      options: [
        'Pending',
        'Executing',
        'Success',
        'Failed',
        'Timeout',
        'Cancelled',
      ].map(v => ({ value: v, label: v })),
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
        Error loading remote commands. Please try again later.
      </Alert>
    );
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4">
          Remote Commands ({filteredCommands.length})
        </Typography>
        <Button variant="contained" color="primary">
          Create Command
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
          { label: 'Pending Commands', count: pendingCommands.length, color: 'warning', note: 'Commands waiting to be executed' },
          { label: 'Executing', count: executingCommands.length, color: 'primary', note: 'Commands currently executing' },
          { label: 'Completed', count: completedCommands.length, color: 'success', note: 'Successfully executed commands' },
          { label: 'Failed Commands', count: failedCommands.length, color: 'error', note: 'Commands that failed to execute' },
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
        <Tab label="All Commands" />
        <Tab label="Pending" />
        <Tab label="Executing" />
        <Tab label="Completed" />
        <Tab label="Failed" />
        <Tab label="By Type" />
      </Tabs>

      {filteredCommands.length === 0 ? (
        <Alert severity="info">
          No remote commands found matching your criteria.
        </Alert>
      ) : (
        <Paper sx={{ p: 2 }}>
          <TableContainer>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Command ID</TableCell>
                  <TableCell>Type</TableCell>
                  <TableCell>Truck</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Issued At</TableCell>
                  <TableCell>Completed At</TableCell>
                  <TableCell>Parameters</TableCell>
                  <TableCell>Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {filteredCommands
                  .filter(command => {
                    switch (activeTab) {
                      case 1: return command.status === 'Pending';
                      case 2: return command.status === 'Executing';
                      case 3: return ['Success', 'Failed', 'Timeout', 'Cancelled'].includes(command.status);
                      case 4: return ['Failed', 'Timeout'].includes(command.status);
                      case 5: return true;
                      default: return true;
                    }
                  })
                  .map(command => (
                    <TableRow key={command.id}>
                      <TableCell>{command.command_id}</TableCell>
                      <TableCell>
                        <Chip label={command.command_type} size="small" color="primary" />
                      </TableCell>
                      <TableCell>
                        {command.truck_id ? `TRK-${command.truck_id}` : 'All Trucks'}
                      </TableCell>
                      <TableCell>
                        <StatusBadge status={command.status} severity={command.status} />
                      </TableCell>
                      <TableCell>
                        {new Date(command.issued_at).toLocaleString()}
                      </TableCell>
                      <TableCell>
                        {command.completed_at ? new Date(command.completed_at).toLocaleString() : 'N/A'}
                      </TableCell>
                      <TableCell>
                        <Typography variant="caption">
                          {JSON.stringify(command.parameters, null, 2)}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Button size="small" variant="outlined">
                          View Details
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))}
              </TableBody>
            </Table>
          </TableContainer>
        </Paper>
      )}
    </Box>
  );
};

export default RemoteCommandsList;
