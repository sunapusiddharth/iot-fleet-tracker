import React, { useContext, MouseEvent } from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  IconButton,
  Badge,
  Menu,
  MenuItem,
  Avatar,
  Tooltip,
} from '@mui/material';
import {
  Notifications as NotificationsIcon,
  AccountCircle as AccountCircleIcon,
  Menu as MenuIcon,
  Search as SearchIcon,
  Settings as SettingsIcon,
} from '@mui/icons-material';
import { useAlerts } from '../../hooks/useAlerts';
import { AuthContext } from '../../contexts/AuthContext';

interface Alert {
  id: string;
  message: string;
  severity: 'Info' | 'Warning' | 'Critical' | 'Emergency';
  triggered_at: string;
}

const Header: React.FC = () => {
  const { alerts } = useAlerts() as { alerts: Alert[] };
  const { user, logout } = useContext(AuthContext) as {
    user: { name?: string };
    logout: () => void;
  };

  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const [notificationAnchorEl, setNotificationAnchorEl] = React.useState<null | HTMLElement>(null);

  const handleMenu = (event: MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleNotificationMenu = (event: MouseEvent<HTMLElement>) => {
    setNotificationAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
    setNotificationAnchorEl(null);
  };

  const handleLogout = () => {
    logout();
    handleClose();
  };

  const criticalAlerts = alerts.filter(
    alert => alert.severity === 'Critical' || alert.severity === 'Emergency'
  ).length;

  return (
    <AppBar position="static" color="default" elevation={1}>
      <Toolbar>
        <IconButton edge="start" color="inherit" aria-label="menu" sx={{ mr: 2 }}>
          <MenuIcon />
        </IconButton>

        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          Truck Fleet Management Dashboard
        </Typography>

        <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
          <Tooltip title="Search">
            <IconButton color="inherit">
              <SearchIcon />
            </IconButton>
          </Tooltip>

          <Tooltip title="Notifications">
            <IconButton color="inherit" onClick={handleNotificationMenu}>
              <Badge badgeContent={criticalAlerts} color="error">
                <NotificationsIcon />
              </Badge>
            </IconButton>
          </Tooltip>

          <Menu
            anchorEl={notificationAnchorEl}
            open={Boolean(notificationAnchorEl)}
            onClose={handleClose}
            PaperProps={{ style: { maxHeight: 400, width: '350px' } }}
          >
            <MenuItem>
              <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                Critical Alerts ({criticalAlerts})
              </Typography>
            </MenuItem>
            {alerts
              .filter(alert => alert.severity === 'Critical' || alert.severity === 'Emergency')
              .slice(0, 5)
              .map(alert => (
                <MenuItem key={alert.id} onClick={handleClose}>
                  <div>
                    <Typography variant="subtitle2">{alert.message}</Typography>
                    <Typography variant="caption" color="textSecondary">
                      {new Date(alert.triggered_at).toLocaleString()}
                    </Typography>
                  </div>
                </MenuItem>
              ))}
            {criticalAlerts > 5 && (
              <MenuItem>
                <Typography variant="caption" color="textSecondary">
                  ... and {criticalAlerts - 5} more alerts
                </Typography>
              </MenuItem>
            )}
          </Menu>

          <Tooltip title="Settings">
            <IconButton color="inherit">
              <SettingsIcon />
            </IconButton>
          </Tooltip>

          <IconButton
            size="large"
            edge="end"
            aria-label="account of current user"
            aria-controls="menu-appbar"
            aria-haspopup="true"
            onClick={handleMenu}
            color="inherit"
          >
            <Avatar sx={{ width: 32, height: 32 }}>
              {user?.name?.charAt(0) || 'U'}
            </Avatar>
          </IconButton>

          <Menu
            id="menu-appbar"
            anchorEl={anchorEl}
            anchorOrigin={{ vertical: 'top', horizontal: 'right' }}
            keepMounted
            transformOrigin={{ vertical: 'top', horizontal: 'right' }}
            open={Boolean(anchorEl)}
            onClose={handleClose}
          >
            <MenuItem onClick={handleClose}>
              <Typography variant="subtitle1">Profile</Typography>
            </MenuItem>
            <MenuItem onClick={handleClose}>
              <Typography variant="subtitle1">My Account</Typography>
            </MenuItem>
            <MenuItem onClick={handleLogout}>
              <Typography variant="subtitle1">Logout</Typography>
            </MenuItem>
          </Menu>
        </div>
      </Toolbar>
    </AppBar>
  );
};

export default Header;
