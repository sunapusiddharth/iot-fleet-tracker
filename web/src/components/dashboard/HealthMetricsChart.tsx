import React, { useMemo } from 'react';
import {
  Box,
  Card,
  CardContent,
  CardHeader,
  Typography,
  useTheme,
  useMediaQuery,
} from '@mui/material';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  ReferenceLine,
  Label,
} from 'recharts';
import { HealthStatus } from '../../types/health';
import { format } from 'date-fns';

interface HealthMetricsChartProps {
  /**
   * Array of health status records
   */
  data: HealthStatus[];
  
  /**
   * Title for the chart
   * @default "System Health Metrics"
   */
  title?: string;
  
  /**
   * Height of the chart container
   * @default 400
   */
  height?: number;
  
  /**
   * Whether to show CPU usage line
   * @default true
   */
  showCpu?: boolean;
  
  /**
   * Whether to show memory usage line
   * @default true
   */
  showMemory?: boolean;
  
  /**
   * Whether to show disk usage line
   * @default true
   */
  showDisk?: boolean;
  
  /**
   * Whether to show temperature line
   * @default true
   */
  showTemperature?: boolean;
  
  /**
   * Time range in hours to display
   * @default 24
   */
  timeRangeHours?: number;
}

/**
 * Health Metrics Chart Component
 * 
 * Displays system health metrics over time including CPU, memory, disk, and temperature.
 * 
 * @example
 * ```tsx
 * <HealthMetricsChart 
 *   data={healthData} 
 *   timeRangeHours={48}
 *   showTemperature={false}
 * />
 * ```
 */
const HealthMetricsChart: React.FC<HealthMetricsChartProps> = ({
  data,
  title = "System Health Metrics",
  height = 400,
  showCpu = true,
  showMemory = true,
  showDisk = true,
  showTemperature = true,
  timeRangeHours = 24,
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down('sm'));
  
  // Process data for chart
  const chartData = useMemo(() => {
    if (!data || data.length === 0) return [];
    
    // Filter data by time range
    const now = new Date();
    const startTime = new Date(now.getTime() - timeRangeHours * 60 * 60 * 1000);
    
    return data
      .filter(item => new Date(item.timestamp) >= startTime)
      .map(item => ({
        timestamp: item.timestamp,
        time: format(new Date(item.timestamp), 'HH:mm'),
        date: format(new Date(item.timestamp), 'MMM dd'),
        cpu: item.resources.cpu_percent,
        memory: item.resources.memory_percent,
        disk: item.resources.disk_percent,
        temperature: item.resources.temperature_c,
      }))
      .sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime());
  }, [data, timeRangeHours]);
  
  // Determine if we have enough data to show
  const hasData = chartData.length > 0;
  
  // Calculate averages for reference lines
  const averages = useMemo(() => {
    if (!hasData) return { cpu: 0, memory: 0, disk: 0, temperature: 0 };
    
    const sum = chartData.reduce(
      (acc, curr) => ({
        cpu: acc.cpu + curr.cpu,
        memory: acc.memory + curr.memory,
        disk: acc.disk + curr.disk,
        temperature: acc.temperature + curr.temperature,
      }),
      { cpu: 0, memory: 0, disk: 0, temperature: 0 }
    );
    
    const count = chartData.length;
    return {
      cpu: sum.cpu / count,
      memory: sum.memory / count,
      disk: sum.disk / count,
      temperature: sum.temperature / count,
    };
  }, [chartData, hasData]);
  
  // Format tooltip values
  const formatTooltipValue = (value: number, name: string) => {
    if (name === 'temperature') {
      return `${value.toFixed(1)}°C`;
    }
    return `${value.toFixed(1)}%`;
  };
  
  // Custom tooltip component
  const CustomTooltip = ({ active, payload, label }: any) => {
    if (active && payload && payload.length) {
      return (
        <Card variant="outlined" sx={{ bgcolor: 'background.paper' }}>
          <CardContent sx={{ p: 1 }}>
            <Typography variant="subtitle2" sx={{ mb: 1 }}>
              {label}
            </Typography>
            {payload.map((entry: any, index: number) => (
              <Typography 
                key={index} 
                variant="body2"
                sx={{ 
                  color: entry.color,
                  display: 'flex',
                  justifyContent: 'space-between',
                }}
              >
                <span>{entry.name}:</span>
                <span>{formatTooltipValue(entry.value, entry.dataKey)}</span>
              </Typography>
            ))}
          </CardContent>
        </Card>
      );
    }
    return null;
  };
  
  return (
    <Card elevation={2}>
      <CardHeader
        title={title}
        subheader={`Last ${timeRangeHours} hours`}
        sx={{ pb: 0 }}
      />
      <CardContent>
        {hasData ? (
          <Box sx={{ height: height, width: '100%' }}>
            <ResponsiveContainer width="100%" height="100%">
              <LineChart
                data={chartData}
                margin={{
                  top: 20,
                  right: isMobile ? 10 : 30,
                  left: isMobile ? 10 : 20,
                  bottom: 20,
                }}
              >
                <CartesianGrid strokeDasharray="3 3" strokeOpacity={0.3} />
                <XAxis 
                  dataKey="time" 
                  interval={isMobile ? 4 : 2}
                >
                  <Label 
                    value="Time" 
                    position="insideBottom" 
                    offset={-10} 
                    style={{ textAnchor: 'middle' }} 
                  />
                </XAxis>
                <YAxis 
                  domain={[0, 100]}
                  tickFormatter={(value) => `${value}%`}
                >
                  <Label 
                    value="Percentage (%)" 
                    angle={-90} 
                    position="insideLeft" 
                    style={{ textAnchor: 'middle' }} 
                  />
                </YAxis>
                <Tooltip content={<CustomTooltip />} />
                <Legend 
                  verticalAlign="top" 
                  height={36}
                  wrapperStyle={{ paddingBottom: '10px' }}
                />
                
                {/* Reference lines for averages */}
                {showCpu && (
                  <ReferenceLine 
                    y={averages.cpu} 
                    stroke={theme.palette.primary.main} 
                    strokeDasharray="3 3"
                  >
                    <Label 
                      value={`Avg CPU: ${averages.cpu.toFixed(1)}%`} 
                      position="insideTopLeft" 
                      fill={theme.palette.primary.main}
                      offset={10}
                    />
                  </ReferenceLine>
                )}
                
                {showMemory && (
                  <ReferenceLine 
                    y={averages.memory} 
                    stroke={theme.palette.secondary.main} 
                    strokeDasharray="3 3"
                  >
                    <Label 
                      value={`Avg Mem: ${averages.memory.toFixed(1)}%`} 
                      position="insideTopRight" 
                      fill={theme.palette.secondary.main}
                      offset={10}
                    />
                  </ReferenceLine>
                )}
                
                {/* Data lines */}
                {showCpu && (
                  <Line
                    type="monotone"
                    dataKey="cpu"
                    name="CPU Usage"
                    stroke={theme.palette.primary.main}
                    strokeWidth={2}
                    dot={false}
                    activeDot={{ r: 6 }}
                  />
                )}
                
                {showMemory && (
                  <Line
                    type="monotone"
                    dataKey="memory"
                    name="Memory Usage"
                    stroke={theme.palette.secondary.main}
                    strokeWidth={2}
                    dot={false}
                    activeDot={{ r: 6 }}
                  />
                )}
                
                {showDisk && (
                  <Line
                    type="monotone"
                    dataKey="disk"
                    name="Disk Usage"
                    stroke={theme.palette.info.main}
                    strokeWidth={2}
                    dot={false}
                    activeDot={{ r: 6 }}
                  />
                )}
                
                {showTemperature && (
                  <Line
                    type="monotone"
                    dataKey="temperature"
                    name="Temperature"
                    stroke={theme.palette.warning.main}
                    strokeWidth={2}
                    dot={false}
                    activeDot={{ r: 6 }}
                    yAxisId="temperature"
                  />
                )}
              </LineChart>
            </ResponsiveContainer>
          </Box>
        ) : (
          <Box 
            sx={{ 
              height: height, 
              display: 'flex', 
              alignItems: 'center', 
              justifyContent: 'center' 
            }}
          >
            <Typography variant="body2" color="textSecondary">
              No health data available for the selected time range
            </Typography>
          </Box>
        )}
      </CardContent>
    </Card>
  );
};

export default HealthMetricsChart;