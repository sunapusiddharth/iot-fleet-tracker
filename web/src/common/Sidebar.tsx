import React from 'react';
import {
  Drawer,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Typography,
  Divider,
  Box,
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  DirectionsCar as TruckIcon,
  Warning as AlertIcon,
  Psychology as MlIcon,
  HealthAndSafety as HealthIcon,
  SystemUpdate as OtaIcon,
  Settings as SettingsIcon,
} from '@mui/icons-material';
import { Link, useLocation } from 'react-router-dom';

interface MenuItem {
  text: string;
  icon: React.ReactElement;
  path: string;
}

const Sidebar: React.FC = () => {
  const location = useLocation();

  const menuItems: MenuItem[] = [
    { text: 'Dashboard', icon: <DashboardIcon />, path: '/' },
    { text: 'Trucks', icon: <TruckIcon />, path: '/trucks' },
    { text: 'Alerts', icon: <AlertIcon />, path: '/alerts' },
    { text: 'ML Events', icon: <MlIcon />, path: '/ml-events' },
    { text: 'Health Status', icon: <HealthIcon />, path: '/health' },
    { text: 'OTA Updates', icon: <OtaIcon />, path: '/ota/updates' },
    { text: 'Remote Commands', icon: <SettingsIcon />, path: '/ota/commands' },
  ];

  return (
    <Drawer
      variant="permanent"
      sx={{
        width: 240,
        flexShrink: 0,
        [`& .MuiDrawer-paper`]: { width: 240, boxSizing: 'border-box' },
      }}
    >
      <Box sx={{ p: 2 }}>
        <Typography variant="h6" component="div">
          Fleet Management
        </Typography>
      </Box>
      <Divider />
      <List>
        {menuItems.map((item) => (
          <ListItem
            button
            component={Link}
            to={item.path}
            key={item.text}
            selected={location.pathname === item.path}
            sx={{
              backgroundColor:
                location.pathname === item.path ? 'action.selected' : 'inherit',
              '&.Mui-selected': {
                backgroundColor: 'action.selected',
                '&:hover': {
                  backgroundColor: 'action.hover',
                },
              },
            }}
          >
            <ListItemIcon>{item.icon}</ListItemIcon>
            <ListItemText primary={item.text} />
          </ListItem>
        ))}
      </List>
    </Drawer>
  );
};

export default Sidebar;
