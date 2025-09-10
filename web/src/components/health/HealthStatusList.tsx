// import React, { useState, useEffect } from 'react';
// import {
//   Box,
//   Typography,
//   Paper,
//   Grid,
//   Tabs,
//   Tab,
//   CircularProgress,
//   Alert,
//   Button,
//   TextField,
//   MenuItem,
//   Select,
//   FormControl,
//   InputLabel,
// } from '@mui/material';
// import { Search as SearchIcon } from '@mui/icons-material';
// import { HealthStatusCard } from './HealthStatusCard';
// import { FilterBar } from '../../common/FilterBar';
// import { StatusBadge } from '../../common/StatusBadge';
// import { useHealth } from '../../hooks/useHealth';

// const HealthStatusList = () => {
//   const { healthStatus, loading, error, fetchHealthStatus } = useHealth();
//   const [activeTab, setActiveTab] = useState(0);
//   const [filters, setFilters] = useState({
//     status: '',
//     truckId: '',
//     minCpu: '',
//     maxCpu: '',
//     minMemory: '',
//     maxMemory: '',
//     dateRange: [null, null],
//   });

//   useEffect(() => {
//     fetchHealthStatus();
//   }, [fetchHealthStatus]);

//   const handleTabChange = (event, newValue) => {
//     setActiveTab(newValue);
//   };

//   const filteredHealthStatus = healthStatus.filter(status => {
//     if (filters.status && status.status !== filters.status) return false;
//     if (filters.truckId && status.truck_id !== filters.truckId) return false;
//     if (filters.minCpu && status.cpu_percent < parseFloat(filters.minCpu)) return false;
//     if (filters.maxCpu && status.cpu_percent > parseFloat(filters.maxCpu)) return false;
//     if (filters.minMemory && status.memory_percent < parseFloat(filters.minMemory)) return false;
//     if (filters.maxMemory && status.memory_percent > parseFloat(filters.maxMemory)) return false;
//     if (filters.dateRange[0] && new Date(status.timestamp) < filters.dateRange[0]) return false;
//     if (filters.dateRange[1] && new Date(status.timestamp) > filters.dateRange[1]) return false;
//     return true;
//   });

//   const criticalStatus = filteredHealthStatus.filter(s => s.status === 'Critical');
//   const warningStatus = filteredHealthStatus.filter(s => s.status === 'Warning');
//   const okStatus = filteredHealthStatus.filter(s => s.status === 'Ok');

//   const highCpuStatus = filteredHealthStatus.filter(s => s.cpu_percent > 80);
//   const highMemoryStatus = filteredHealthStatus.filter(s => s.memory_percent > 85);
//   const highTempStatus = filteredHealthStatus.filter(s => s.temperature_c > 70);

//   const availableFilters = [
//     {
//       key: 'status',
//       label: 'Status',
//       type: 'select',
//       options: [
//         { value: 'Ok', label: 'Ok' },
//         { value: 'Warning', label: 'Warning' },
//         { value: 'Critical', label: 'Critical' },
//         { value: 'Degraded', label: 'Degraded' },
//         { value: 'ShutdownPending', label: 'Shutdown Pending' },
//       ],
//     },
//     {
//       key: 'minCpu',
//       label: 'Min CPU %',
//       type: 'select',
//       options: [
//         { value: '50', label: '50%' },
//         { value: '60', label: '60%' },
//         { value: '70', label: '70%' },
//         { value: '80', label: '80%' },
//         { value: '90', label: '90%' },
//       ],
//     },
//     {
//       key: 'maxCpu',
//       label: 'Max CPU %',
//       type: 'select',
//       options: [
//         { value: '60', label: '60%' },
//         { value: '70', label: '70%' },
//         { value: '80', label: '80%' },
//         { value: '90', label: '90%' },
//         { value: '100', label: '100%' },
//       ],
//     },
//     {
//       key: 'minMemory',
//       label: 'Min Memory %',
//       type: 'select',
//       options: [
//         { value: '50', label: '50%' },
//         { value: '60', label: '60%' },
//         { value: '70', label: '70%' },
//         { value: '80', label: '80%' },
//         { value: '90', label: '90%' },
//       ],
//     },
//     {
//       key: 'maxMemory',
//       label: 'Max Memory %',
//       type: 'select',
//       options: [
//         { value: '60', label: '60%' },
//         { value: '70', label: '70%' },
//         { value: '80', label: '80%' },
//         { value: '90', label: '90%' },
//         { value: '100', label: '100%' },
//       ],
//     },
//     {
//       key: 'dateRange',
//       label: 'Date Range',
//       type: 'dateRange',
//     },
//   ];

//   if (loading) {
//     return (
//       <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
//         <CircularProgress />
//       </Box>
//     );
//   }

//   if (error) {
//     return (
//       <Alert severity="error">
//         Error loading health status. Please try again later.
//       </Alert>
//     );
//   }

//   return (
//     <Box>
//       <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
//         <Typography variant="h4">
//           Health Status ({filteredHealthStatus.length})
//         </Typography>
//         <Button variant="contained" color="primary">
//           Refresh Status
//         </Button>
//       </Box>
      
//       <FilterBar
//         filters={filters}
//         onFilterChange={setFilters}
//         onApplyFilters={() => {}}
//         onResetFilters={() => {}}
//         availableFilters={availableFilters}
//       />
      
//       <Grid container spacing={3} sx={{ mb: 3 }}>
//         <Grid item xs={12} sm={4}>
//           <Paper sx={{ p: 2, height: '100%' }}>
//             <Typography variant="h6" gutterBottom>
//               Critical Status
//             </Typography>
//             <Typography variant="h3" color="error" sx={{ fontWeight: 'bold' }}>
//               {criticalStatus.length}
//             </Typography>
//             <Typography variant="body2" color="textSecondary">
//               Requires immediate attention
//             </Typography>
//           </Paper>
//         </Grid>
//         <Grid item xs={12} sm={4}>
//           <Paper sx={{ p: 2, height: '100%' }}>
//             <Typography variant="h6" gutterBottom>
//               Warning Status
//             </Typography>
//             <Typography variant="h3" color="warning" sx={{ fontWeight: 'bold' }}>
//               {warningStatus.length}
//             </Typography>
//             <Typography variant="body2" color="textSecondary">
//               Monitor and address soon
//             </Typography>
//           </Paper>
//         </Grid>
//         <Grid item xs={12} sm={4}>
//           <Paper sx={{ p: 2, height: '100%' }}>
//             <Typography variant="h6" gutterBottom>
//               OK Status
//             </Typography>
//             <Typography variant="h3" color="success" sx={{ fontWeight: 'bold' }}>
//               {okStatus.length}
//             </Typography>
//             <Typography variant="body2" color="textSecondary">
//               Systems operating normally
//             </Typography>
//           </Paper>
//         </Grid>
//         <Grid item xs={12} sm={4}>
//           <Paper sx={{ p: 2, height: '100%' }}>
//             <Typography variant="h6" gutterBottom>
//               High CPU Usage
//             </Typography>
//             <Typography variant="h3" color="warning" sx={{ fontWeight: 'bold' }}>
//               {highCpuStatus.length}
//             </Typography>
//             <Typography variant="body2" color="textSecondary">
//               CPU > 80%
//             </Typography>
//           </Paper>
//         </Grid>
//         <Grid item xs={12} sm={4}>
//           <Paper sx={{ p: 2, height: '100%' }}>
//             <Typography variant="h6" gutterBottom>
//               High Memory Usage
//             </Typography>
//             <Typography variant="h3" color="warning" sx={{ fontWeight: 'bold' }}>
//               {highMemoryStatus.length}
//             </Typography>
//             <Typography variant="body2" color="textSecondary">
//               Memory > 85%
//             </Typography>
//           </Paper>
//         </Grid>
//         <Grid item xs={12} sm={4}>
//           <Paper sx={{ p: 2, height: '100%' }}>
//             <Typography variant="h6" gutterBottom>
//               High Temperature
//             </Typography>
//             <Typography variant="h3" color="error" sx={{ fontWeight: 'bold' }}>
//               {highTempStatus.length}
//             </Typography>
//             <Typography variant="body2" color="textSecondary">
//               Temperature > 70°C
//             </Typography>
//           </Paper>
//         </Grid>
//       </Grid>
      
//       <Tabs
//         value={activeTab}
//         onChange={handleTabChange}
//         sx={{ mb: 3 }}
//       >
//         <Tab label="All Status" />
//         <Tab label="Critical" />
//         <Tab label="Warning" />
//         <Tab label="OK" />
//         <Tab label="High CPU" />
//         <Tab label="High Memory" />
//         <Tab label="High Temperature" />
//       </Tabs>
      
//       {filteredHealthStatus.length === 0 ? (
//         <Alert severity="info">
//           No health status found matching your criteria.
//         </Alert>
//       ) : (
//         <Grid container spacing={3}>
//           {filteredHealthStatus
//             .filter(status => {
//               switch (activeTab) {
//                 case 1:
//                   return status.status === 'Critical';
//                 case 2:
//                   return status.status === 'Warning';
//                 case 3:
//                   return status.status === 'Ok';
//                 case 4:
//                   return status.cpu_percent > 80;
//                 case 5:
//                   return status.memory_percent > 85;
//                 case 6:
//                   return status.temperature_c > 70;
//                 default:
//                   return true;
//               }
//             })
//             .map((status) => (
//               <Grid item xs={12} sm={6} md={4} key={status.id}>
//                 <HealthStatusCard status={status} />
//               </Grid>
//             ))}
//         </Grid>
//       )}
//     </Box>
//   );
// };

// export default HealthStatusList;


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
} from '@mui/material';
import { HealthStatusCard } from './HealthStatusCard';
import { FilterBar } from '../../common/FilterBar';
import { useHealth } from '../../hooks/useHealth';
import { DateRange } from '@mui/x-date-pickers-pro';

type HealthStatusType = 'Ok' | 'Warning' | 'Critical' | 'Degraded' | 'ShutdownPending';

interface Alert {
  alert_type: string;
  severity: 'Critical' | 'Warning' | 'Info' | 'Emergency';
}

interface HealthStatus {
  id: string;
  truck_id: string;
  status: HealthStatusType;
  cpu_percent: number;
  memory_percent: number;
  disk_percent: number;
  temperature_c: number;
  uptime_sec: number;
  timestamp: string;
  alerts?: Alert[];
}

interface Filters {
  status: string;
  truckId: string;
  minCpu: string;
  maxCpu: string;
  minMemory: string;
  maxMemory: string;
  dateRange: DateRange<Date>;
}

const HealthStatusList: React.FC = () => {
  const { healthStatus, loading, error, fetchHealthStatus } = useHealth() as {
    healthStatus: HealthStatus[];
    loading: boolean;
    error: boolean;
    fetchHealthStatus: () => void;
  };

  const [activeTab, setActiveTab] = useState<number>(0);
  const [filters, setFilters] = useState<Filters>({
    status: '',
    truckId: '',
    minCpu: '',
    maxCpu: '',
    minMemory: '',
    maxMemory: '',
    dateRange: [null, null],
  });

  useEffect(() => {
    fetchHealthStatus();
  }, [fetchHealthStatus]);

  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  const filteredHealthStatus = healthStatus.filter(status => {
    if (filters.status && status.status !== filters.status) return false;
    if (filters.truckId && status.truck_id !== filters.truckId) return false;
    if (filters.minCpu && status.cpu_percent < parseFloat(filters.minCpu)) return false;
    if (filters.maxCpu && status.cpu_percent > parseFloat(filters.maxCpu)) return false;
    if (filters.minMemory && status.memory_percent < parseFloat(filters.minMemory)) return false;
    if (filters.maxMemory && status.memory_percent > parseFloat(filters.maxMemory)) return false;
    if (filters.dateRange[0] && new Date(status.timestamp) < filters.dateRange[0]) return false;
    if (filters.dateRange[1] && new Date(status.timestamp) > filters.dateRange[1]) return false;
    return true;
  });

  const criticalStatus = filteredHealthStatus.filter(s => s.status === 'Critical');
  const warningStatus = filteredHealthStatus.filter(s => s.status === 'Warning');
  const okStatus = filteredHealthStatus.filter(s => s.status === 'Ok');
  const highCpuStatus = filteredHealthStatus.filter(s => s.cpu_percent > 80);
  const highMemoryStatus = filteredHealthStatus.filter(s => s.memory_percent > 85);
  const highTempStatus = filteredHealthStatus.filter(s => s.temperature_c > 70);

  const availableFilters = [
    {
      key: 'status',
      label: 'Status',
      type: 'select',
      options: [
        { value: 'Ok', label: 'Ok' },
        { value: 'Warning', label: 'Warning' },
        { value: 'Critical', label: 'Critical' },
        { value: 'Degraded', label: 'Degraded' },
        { value: 'ShutdownPending', label: 'Shutdown Pending' },
      ],
    },
    {
      key: 'minCpu',
      label: 'Min CPU %',
      type: 'select',
      options: ['50', '60', '70', '80', '90'].map(v => ({ value: v, label: `${v}%` })),
    },
    {
      key: 'maxCpu',
      label: 'Max CPU %',
      type: 'select',
      options: ['60', '70', '80', '90', '100'].map(v => ({ value: v, label: `${v}%` })),
    },
    {
      key: 'minMemory',
      label: 'Min Memory %',
      type: 'select',
      options: ['50', '60', '70', '80', '90'].map(v => ({ value: v, label: `${v}%` })),
    },
    {
      key: 'maxMemory',
      label: 'Max Memory %',
      type: 'select',
      options: ['60', '70', '80', '90', '100'].map(v => ({ value: v, label: `${v}%` })),
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
        Error loading health status. Please try again later.
      </Alert>
    );
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4">
          Health Status ({filteredHealthStatus.length})
        </Typography>
        <Button variant="contained" color="primary" onClick={fetchHealthStatus}>
          Refresh Status
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
          { label: 'Critical Status', count: criticalStatus.length, color: 'error', note: 'Requires immediate attention' },
          { label: 'Warning Status', count: warningStatus.length, color: 'warning', note: 'Monitor and address soon' },
          { label: 'OK Status', count: okStatus.length, color: 'success', note: 'Systems operating normally' },
          { label: 'High CPU Usage', count: highCpuStatus.length, color: 'warning', note: 'CPU > 80%' },
          { label: 'High Memory Usage', count: highMemoryStatus.length, color: 'warning', note: 'Memory > 85%' },
          { label: 'High Temperature', count: highTempStatus.length, color: 'error', note: 'Temperature > 70°C' },
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
        <Tab label="All Status" />
        <Tab label="Critical" />
        <Tab label="Warning" />
        <Tab label="OK" />
        <Tab label="High CPU" />
        <Tab label="High Memory" />
        <Tab label="High Temperature" />
      </Tabs>

      {filteredHealthStatus.length === 0 ? (
        <Alert severity="info">No health status found matching your criteria.</Alert>
      ) : (
        <Grid container spacing={3}>
          {filteredHealthStatus
            .filter(status => {
              switch (activeTab) {
                case 1: return status.status === 'Critical';
                case 2: return status.status === 'Warning';
                case 3: return status.status === 'Ok';
                case 4: return status.cpu_percent > 80;
                case 5: return status.memory_percent > 85;
                case 6: return status.temperature_c > 70;
                default: return true;
              }
            })
            .map(status => (
              <Grid item xs={12} sm={6} md={4} key={status.id}>
                <HealthStatusCard status={status} />
              </Grid>
            ))}
        </Grid>
      )}
    </Box>
  );
};

export default HealthStatusList;
