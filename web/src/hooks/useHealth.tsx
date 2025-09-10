import { useState, useEffect, useCallback } from 'react';
import { useApi } from '../contexts/ApiContext';
import { getHealth } from '../services/api';

export const useHealth = () => {
  const api = useApi();
  const [healthStatus, setHealthStatus] = useState([]);
  const [health, setHealth] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchHealthStatus = useCallback(async (params = {}) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await getHealth(params);
      setHealthStatus(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  const fetchHealth = useCallback(async (id:string) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await getHealth(id);
      setHealth(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  return {
    healthStatus,
    health,
    loading,
    error,
    fetchHealthStatus,
    fetchHealth,
  };
};