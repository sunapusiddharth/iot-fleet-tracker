import { useState, useEffect, useCallback } from 'react';
import { useApi } from '../contexts/ApiContext';
import { getRemoteCommands } from '../services/api';

export const useOta = () => {
  const api = useApi();
  const [otaUpdates, setOtaUpdates] = useState([]);
  const [remoteCommands, setRemoteCommands] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchOtaUpdates = useCallback(async (params = {}) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await api.get('/ota/updates', { params });
      setOtaUpdates(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  const createOtaUpdate = useCallback(async (updateData) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await api.post('/ota/updates', updateData);
      setOtaUpdates(prev => [response.data, ...prev]);
      return response.data;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [api]);

  const updateOtaUpdate = useCallback(async (id:string, updateData) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      updateOtaUpdate
      const response = await api.put(`/ota/updates/${id}`, updateData);
      setOtaUpdates(prev => prev.map(u => u.id === id ? response.data : u));
      return response.data;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [api]);

  const fetchRemoteCommands = useCallback(async (params = {}) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await getRemoteCommands(params);
      setRemoteCommands(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  const createRemoteCommand = useCallback(async (commandData) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await createRemoteCommand(commandData);
      setRemoteCommands(prev => [response.data, ...prev]);
      return response.data;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [api]);

  return {
    otaUpdates,
    remoteCommands,
    loading,
    error,
    fetchOtaUpdates,
    createOtaUpdate,
    updateOtaUpdate,
    fetchRemoteCommands,
    createRemoteCommand,
  };
};