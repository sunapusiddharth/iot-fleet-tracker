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
  PieChart,
  Pie,
  Cell,
  ResponsiveContainer,
  Tooltip,
  Legend,
} from 'recharts';
import { Truck, TruckStatus } from '../../types/truck';

interface TruckStatusChartProps {
  /**
   * Array of truck objects
   */
  trucks: Truck[];
  
  /**
   * Title for the chart
   * @default "Truck Status Distribution"
   */
  title?: string;
  
  /**
   * Height of the chart container
   * @default 400
   */
  height?: number;
  
  /**
   * Whether to show legend
   * @default true
   */
  showLegend?: boolean;
  
  /**
   * Whether to show tooltip
   * @default true
   */
  showTooltip?: boolean;
}

/**
 * Truck Status Chart Component
 * 
 * Displays distribution of truck statuses (Online, Offline, Maintenance, etc.)
 * 
 * @example
 * ```tsx
 * <TruckStatusChart 
 *   trucks={truckData} 
 *   title="Fleet Status Overview"
 * />
 * ```
 */
const TruckStatusChart: React.FC<TruckStatusChartProps> = ({
  trucks,
  title = "Truck Status Distribution",
  height = 400,
  showLegend = true,
  showTooltip = true,
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down('sm'));
  
  // Colors for different statuses
  const statusColors: Record<TruckStatus, string> = {
    [TruckStatus.Online]: theme.palette.success.main,
    [TruckStatus.Offline]: theme.palette.error.main,
    [TruckStatus.Maintenance]: theme.palette.warning.main,
    [TruckStatus.Inactive]: theme.palette.grey[500],
  };
  
  // Process data for chart
  const chartData = useMemo(() => {
    if (!trucks || trucks.length === 0) return [];
    
    // Count trucks by status
    const statusCounts: Record<TruckStatus, number> = {
      [TruckStatus.Online]: 0,
      [TruckStatus.Offline]: 0,
      [TruckStatus.Maintenance]: 0,
      [TruckStatus.Inactive]: 0,
    };
    
    trucks.forEach(truck => {
      statusCounts[truck.status]++;
    });
    
    // Convert to chart format
    return Object.entries(statusCounts)
      .filter(([_, count]) => count > 0)
      .map(([status, count]) => ({
        name: status,
        value: count,
        color: statusColors[status as TruckStatus],
      }));
  }, [trucks, statusColors]);
  
  // Calculate total trucks
  const totalTrucks = useMemo(() => {
    return chartData.reduce((sum, item) => sum + item.value, 0);
  }, [chartData]);
  
  // Custom tooltip component
  const CustomTooltip = ({ active, payload }: any) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      const percentage = ((data.value / totalTrucks) * 100).toFixed(1);
      
      return (
        <Card variant="outlined" sx={{ bgcolor: 'background.paper' }}>
          <CardContent sx={{ p: 1.5 }}>
            <Typography 
              variant="subtitle2" 
              sx={{ 
                color: data.color,
                fontWeight: 'bold',
                mb: 0.5
              }}
            >
              {data.name}
            </Typography>
            <Typography variant="body2">
              Count: {data.value}
            </Typography>
            <Typography variant="body2">
              Percentage: {percentage}%
            </Typography>
          </CardContent>
        </Card>
      );
    }
    return null;
  };
  
  // Custom label formatter
  const renderCustomLabel = (props: any) => {
    const { cx, cy, midAngle, outerRadius, percent, name, value } = props;
    const RADIAN = Math.PI / 180;
    
    // Calculate position for label
    const radius = outerRadius * 1.1;
    const x = cx + radius * Math.cos(-midAngle * RADIAN);
    const y = cy + radius * Math.sin(-midAngle * RADIAN);
    
    // Only show label if percentage is significant
    if (percent < 0.05) return null;
    
    return (
      <text 
        x={x} 
        y={y} 
        fill={theme.palette.text.primary} 
        textAnchor={x > cx ? 'start' : 'end'} 
        dominantBaseline="central"
        fontSize={isMobile ? 12 : 14}
      >
        {`${name}: ${(percent * 100).toFixed(0)}%`}
      </text>
    );
  };
  
  // Determine if we have enough data to show
  const hasData = chartData.length > 0;
  
  return (
    <Card elevation={2}>
      <CardHeader
        title={title}
        subheader={`Total trucks: ${totalTrucks}`}
        sx={{ pb: 0 }}
      />
      <CardContent>
        {hasData ? (
          <Box sx={{ height: height, width: '100%' }}>
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={chartData}
                  cx="50%"
                  cy="50%"
                  label={renderCustomLabel}
                  outerRadius={isMobile ? 80 : 120}
                  innerRadius={isMobile ? 40 : 60}
                  fill="#8884d8"
                  dataKey="value"
                  nameKey="name"
                  paddingAngle={2}
                >
                  {chartData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
                {showTooltip && <Tooltip content={<CustomTooltip />} />}
                {showLegend && (
                  <Legend 
                    layout={isMobile ? "horizontal" : "vertical"}
                    verticalAlign="middle"
                    align={isMobile ? "center" : "right"}
                    height={36}
                  />
                )}
              </PieChart>
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
              No truck data available
            </Typography>
          </Box>
        )}
      </CardContent>
    </Card>
  );
};

export default TruckStatusChart;