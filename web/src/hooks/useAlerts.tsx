import { useState, useEffect, useCallback } from 'react';
import { useApi } from '../contexts/ApiContext';
import { getAlert, getAlerts, resolveAlertfn } from '../services/api';

export const useAlerts = () => {
  const api = useApi();
  const [alerts, setAlerts] = useState([]);
  const [alert, setAlert] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchAlerts = useCallback(async (params = {}) => {
    if (!api) return;

    try {
      setLoading(true);
      setError(null);

      const response = await getAlerts(params);
      setAlerts(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  const fetchAlert = useCallback(async (id: string) => {
    if (!api) return;

    try {
      setLoading(true);
      setError(null);

      const response = await getAlert(id);
      setAlert(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  const acknowledgeAlert = useCallback(async (id: string) => {
    if (!api) return;

    try {
      setLoading(true);
      setError(null);
      const response = await acknowledgeAlert(id);
      setAlert(response.data);
      setAlerts(prev => prev.map(a => a.id === id ? response.data : a));
      return response.data;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [api]);

  const resolveAlert = useCallback(async (id:string) => {
    if (!api) return;

    try {
      setLoading(true);
      setError(null);

      const response = await resolveAlertfn(id);
      setAlert(response.data);
      setAlerts(prev => prev.map(a => a.id === id ? response.data : a));
      return response.data;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [api]);

  return {
    alerts,
    alert,
    loading,
    error,
    fetchAlerts,
    fetchAlert,
    acknowledgeAlert,
    resolveAlert,
  };
};