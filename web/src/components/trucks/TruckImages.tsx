import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Grid,
  Paper,
  CircularProgress,
  Alert,
  Tabs,
  Tab,
  IconButton,
  Dialog,
  DialogContent,
  DialogTitle,
  DialogActions,
  Button,
  TextField,
  MenuItem,
  Select,
  FormControl,
  InputLabel,
} from '@mui/material';
import {
  ZoomIn as ZoomInIcon,
  ZoomOut as ZoomOutIcon,
  RotateLeft as RotateLeftIcon,
  RotateRight as RotateRightIcon,
  Download as DownloadIcon,
  CalendarToday as CalendarTodayIcon,
  AccessTime as AccessTimeIcon,
} from '@mui/icons-material';
import { useTelemetry } from '../../hooks/useTelemetry';

const TruckImages = ({ truckId }) => {
  const { telemetry, loading, error, fetchTelemetry } = useTelemetry();
  const [activeTab, setActiveTab] = useState(0);
  const [selectedImage, setSelectedImage] = useState(null);
  const [zoom, setZoom] = useState(1);
  const [rotation, setRotation] = useState(0);
  const [dateFilter, setDateFilter] = useState('today');
  const [timeFilter, setTimeFilter] = useState('all');

  useEffect(() => {
    fetchTelemetry(truckId);
  }, [truckId, fetchTelemetry]);

  const handleTabChange = (event, newValue) => {
    setActiveTab(newValue);
  };

  const handleImageClick = (image) => {
    setSelectedImage(image);
    setZoom(1);
    setRotation(0);
  };

  const handleZoomIn = () => {
    setZoom(prev => Math.min(prev + 0.2, 3));
  };

  const handleZoomOut = () => {
    setZoom(prev => Math.max(prev - 0.2, 0.5));
  };

  const handleRotateLeft = () => {
    setRotation(prev => prev - 90);
  };

  const handleRotateRight = () => {
    setRotation(prev => prev + 90);
  };

  const handleDownload = () => {
    if (selectedImage && selectedImage.url) {
      const link = document.createElement('a');
      link.href = selectedImage.url;
      link.download = `truck-${truckId}-${new Date().toISOString()}.jpg`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    }
  };

  const filteredImages = telemetry
    .filter(t => t.cameras)
    .flatMap(t => {
      const images = [];
      if (t.cameras.front_camera) images.push({ ...t.cameras.front_camera, type: 'Front Camera', timestamp: t.timestamp });
      if (t.cameras.driver_camera) images.push({ ...t.cameras.driver_camera, type: 'Driver Camera', timestamp: t.timestamp });
      if (t.cameras.cargo_camera) images.push({ ...t.cameras.cargo_camera, type: 'Cargo Camera', timestamp: t.timestamp });
      return images;
    })
    .filter(image => {
      const imageDate = new Date(image.timestamp);
      const now = new Date();
      
      if (dateFilter === 'today') {
        return imageDate.toDateString() === now.toDateString();
      } else if (dateFilter === 'yesterday') {
        const yesterday = new Date(now);
        yesterday.setDate(yesterday.getDate() - 1);
        return imageDate.toDateString() === yesterday.toDateString();
      } else if (dateFilter === 'thisWeek') {
        const dayOfWeek = now.getDay();
        const diff = now.getDate() - dayOfWeek + (dayOfWeek === 0 ? -6 : 1);
        const monday = new Date(now);
        monday.setDate(diff);
        return imageDate >= monday;
      } else if (dateFilter === 'thisMonth') {
        return imageDate.getMonth() === now.getMonth() && imageDate.getFullYear() === now.getFullYear();
      }
      return true;
    })
    .filter(image => {
      if (timeFilter === 'morning') {
        const hour = new Date(image.timestamp).getHours();
        return hour >= 6 && hour < 12;
      } else if (timeFilter === 'afternoon') {
        const hour = new Date(image.timestamp).getHours();
        return hour >= 12 && hour < 18;
      } else if (timeFilter === 'evening') {
        const hour = new Date(image.timestamp).getHours();
        return hour >= 18 && hour < 22;
      } else if (timeFilter === 'night') {
        const hour = new Date(image.timestamp).getHours();
        return hour >= 22 || hour < 6;
      }
      return true;
    });

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
        Error loading images. Please try again later.
      </Alert>
    );
  }

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h5">
          Images & Video
        </Typography>
        <Box sx={{ display: 'flex', gap: 2 }}>
          <FormControl size="small">
            <InputLabel>Date</InputLabel>
            <Select
              value={dateFilter}
              onChange={(e) => setDateFilter(e.target.value)}
              label="Date"
            >
              <MenuItem value="today">Today</MenuItem>
              <MenuItem value="yesterday">Yesterday</MenuItem>
              <MenuItem value="thisWeek">This Week</MenuItem>
              <MenuItem value="thisMonth">This Month</MenuItem>
              <MenuItem value="all">All</MenuItem>
            </Select>
          </FormControl>
          
          <FormControl size="small">
            <InputLabel>Time</InputLabel>
            <Select
              value={timeFilter}
              onChange={(e) => setTimeFilter(e.target.value)}
              label="Time"
            >
              <MenuItem value="all">All</MenuItem>
              <MenuItem value="morning">Morning</MenuItem>
              <MenuItem value="afternoon">Afternoon</MenuItem>
              <MenuItem value="evening">Evening</MenuItem>
              <MenuItem value="night">Night</MenuItem>
            </Select>
          </FormControl>
        </Box>
      </Box>
      
      <Tabs
        value={activeTab}
        onChange={handleTabChange}
        sx={{ mb: 3 }}
      >
        <Tab label="Driver Camera" />
        <Tab label="Front Camera" />
        <Tab label="Cargo Camera" />
        <Tab label="All Cameras" />
      </Tabs>
      
      {filteredImages.length === 0 ? (
        <Alert severity="info">
          No images found for the selected filters.
        </Alert>
      ) : (
        <Grid container spacing={3}>
          {filteredImages
            .filter(image => activeTab === 3 || 
              (activeTab === 0 && image.type === 'Driver Camera') ||
              (activeTab === 1 && image.type === 'Front Camera') ||
              (activeTab === 2 && image.type === 'Cargo Camera')
            )
            .map((image, index) => (
              <Grid item xs={12} sm={6} md={4} lg={3} key={index}>
                <Paper
                  sx={{
                    p: 2,
                    cursor: 'pointer',
                    '&:hover': {
                      boxShadow: 3,
                    },
                  }}
                  onClick={() => handleImageClick(image)}
                >
                  <Box
                    sx={{
                      width: '100%',
                      height: 200,
                      overflow: 'hidden',
                      mb: 2,
                      position: 'relative',
                    }}
                  >
                    <img
                      src={image.thumbnail_url || image.url}
                      alt={image.type}
                      style={{
                        width: '100%',
                        height: '100%',
                        objectFit: 'cover',
                      }}
                    />
                  </Box>
                  
                  <Typography variant="subtitle2" gutterBottom>
                    {image.type}
                  </Typography>
                  
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
                    <CalendarTodayIcon fontSize="small" />
                    <Typography variant="caption">
                      {new Date(image.timestamp).toLocaleDateString()}
                    </Typography>
                  </Box>
                  
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                    <AccessTimeIcon fontSize="small" />
                    <Typography variant="caption">
                      {new Date(image.timestamp).toLocaleTimeString()}
                    </Typography>
                  </Box>
                </Paper>
              </Grid>
            ))}
        </Grid>
      )}
      
      <Dialog
        open={!!selectedImage}
        onClose={() => setSelectedImage(null)}
        maxWidth="md"
        fullWidth
      >
        {selectedImage && (
          <>
            <DialogTitle>
              <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <Typography variant="h6">
                  {selectedImage.type}
                </Typography>
                <Box>
                  <IconButton onClick={handleDownload}>
                    <DownloadIcon />
                  </IconButton>
                </Box>
              </Box>
            </DialogTitle>
            
            <DialogContent>
              <Box
                sx={{
                  display: 'flex',
                  justifyContent: 'center',
                  alignItems: 'center',
                  height: '60vh',
                  position: 'relative',
                }}
              >
                <img
                  src={selectedImage.url}
                  alt={selectedImage.type}
                  style={{
                    maxWidth: '100%',
                    maxHeight: '100%',
                    transform: `scale(${zoom}) rotate(${rotation}deg)`,
                    transition: 'transform 0.2s ease',
                  }}
                />
              </Box>
              
              <Box sx={{ mt: 2, display: 'flex', justifyContent: 'center', gap: 1 }}>
                <IconButton onClick={handleZoomOut} disabled={zoom <= 0.5}>
                  <ZoomOutIcon />
                </IconButton>
                <IconButton onClick={handleZoomIn} disabled={zoom >= 3}>
                  <ZoomInIcon />
                </IconButton>
                <IconButton onClick={handleRotateLeft}>
                  <RotateLeftIcon />
                </IconButton>
                <IconButton onClick={handleRotateRight}>
                  <RotateRightIcon />
                </IconButton>
              </Box>
              
              <Box sx={{ mt: 2 }}>
                <Typography variant="subtitle2" gutterBottom>
                  Image Details
                </Typography>
                <Grid container spacing={2}>
                  <Grid item xs={6}>
                    <Typography variant="body2">
                      <strong>Date:</strong> {new Date(selectedImage.timestamp).toLocaleDateString()}
                    </Typography>
                  </Grid>
                  <Grid item xs={6}>
                    <Typography variant="body2">
                      <strong>Time:</strong> {new Date(selectedImage.timestamp).toLocaleTimeString()}
                    </Typography>
                  </Grid>
                  <Grid item xs={6}>
                    <Typography variant="body2">
                      <strong>Resolution:</strong> {selectedImage.width}x{selectedImage.height}
                    </Typography>
                  </Grid>
                  <Grid item xs={6}>
                    <Typography variant="body2">
                      <strong>Size:</strong> {(selectedImage.size_bytes / 1024 / 1024).toFixed(2)} MB
                    </Typography>
                  </Grid>
                </Grid>
              </Box>
            </DialogContent>
            
            <DialogActions>
              <Button onClick={() => setSelectedImage(null)}>
                Close
              </Button>
            </DialogActions>
          </>
        )}
      </Dialog>
    </Box>
  );
};

export default TruckImages;