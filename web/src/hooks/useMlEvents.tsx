import { useState, useEffect, useCallback } from 'react';
import { useApi } from '../contexts/ApiContext';
import { getMlEvent } from '../services/api';

export const useMlEvents = () => {
  const api = useApi();
  const [mlEvents, setMlEvents] = useState([]);
  const [mlEvent, setMlEvent] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchMlEvents = useCallback(async (params = {}) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await getMlEvent(params);
      setMlEvents(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  const fetchMlEvent = useCallback(async (id) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await getMlEvent(id);
      setMlEvent(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  return {
    mlEvents,
    mlEvent,
    loading,
    error,
    fetchMlEvents,
    fetchMlEvent,
  };
};