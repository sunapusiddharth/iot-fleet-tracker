import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider } from '@mui/material/styles';
import { CssBaseline } from '@mui/material';
import { ToastContainer } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import theme from './theme';
import Header from './components/common/Header';
import Sidebar from './components/common/Sidebar';
import Dashboard from './components/dashboard/Dashboard';
import TrucksList from './components/trucks/TrucksList';
import TruckDetail from './components/trucks/TruckDetail';
import AlertsList from './components/alerts/AlertsList';
import AlertDetail from './components/alerts/AlertDetail';
import MlEventsList from './components/ml/MlEventsList';
import HealthStatusList from './components/health/HealthStatusList';
import OtaUpdatesList from './components/ota/OtaUpdatesList';
import RemoteCommandsList from './components/ota/RemoteCommandsList';

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Router>
        <div style={{ display: 'flex', minHeight: '100vh' }}>
          <Sidebar />
          <div style={{ flexGrow: 1, display: 'flex', flexDirection: 'column' }}>
            <Header />
            <main style={{ padding: '24px', flexGrow: 1, backgroundColor: '#f5f5f5' }}>
              <Routes>
                <Route path="/" element={<Dashboard />} />
                <Route path="/trucks" element={<TrucksList />} />
                <Route path="/trucks/:id" element={<TruckDetail />} />
                <Route path="/alerts" element={<AlertsList />} />
                <Route path="/alerts/:id" element={<AlertDetail />} />
                <Route path="/ml-events" element={<MlEventsList />} />
                <Route path="/health" element={<HealthStatusList />} />
                <Route path="/ota/updates" element={<OtaUpdatesList />} />
                <Route path="/ota/commands" element={<RemoteCommandsList />} />
              </Routes>
            </main>
          </div>
        </div>
        <ToastContainer position="top-right" autoClose={5000} />
      </Router>
    </ThemeProvider>
  );
}

export default App;