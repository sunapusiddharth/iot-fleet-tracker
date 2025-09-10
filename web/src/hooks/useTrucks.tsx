import { useState, useEffect, useCallback } from 'react';
import { useApi } from '../contexts/ApiContext';
import { getTrucks, getTruck, createTruckFn } from '../services/api';

export const useTrucks = () => {
  const api = useApi();
  const [trucks, setTrucks] = useState([]);
  const [truck, setTruck] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchTrucks = useCallback(async (params = {}) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await getTrucks(params);
      setTrucks(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  const fetchTruck = useCallback(async (id) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await getTruck(id);
      setTruck(response.data);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }, [api]);

  const createTruck = useCallback(async (truckData) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await createTruckFn(truckData);
      setTrucks(prev => [response.data, ...prev]);
      return response.data;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [api]);

  const updateTruck = useCallback(async (id:string, truckData) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const response = await api.put(`/trucks/${id}`, truckData);
      setTruck(response.data);
      setTrucks(prev => prev.map(t => t.id === id ? response.data : t));
      return response.data;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [api]);

  const deleteTruck = useCallback(async (id:string) => {
    if (!api) return;
    
    try {
      setLoading(true);
      setError(null);
      
      await deleteTruck(id);
      setTrucks(prev => prev.filter(t => t.id !== id));
      if (truck && truck.id === id) {
        setTruck(null);
      }
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [api, truck]);

  return {
    trucks,
    truck,
    loading,
    error,
    fetchTrucks,
    fetchTruck,
    createTruck,
    updateTruck,
    deleteTruck,
  };
};