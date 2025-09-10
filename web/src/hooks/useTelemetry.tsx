import { useState, useEffect, useCallback } from 'react';
import { useApi } from '../contexts/ApiContext';
import { getTruckTelemetry } from '../services/api';

export const useTelemetry = () => {
  const api = useApi();
  const [telemetry, setTelemetry] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchTelemetry = useCallback(async (truckId: string, params = {}) => {
    if (!api) return;

    try {
      setLoading(true);
      setError(null);

      const response = await getTruckTelemetry(truckId, { params });
      setTelemetry(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  return {
    telemetry,
    loading,
    error,
    fetchTelemetry,
  };
};