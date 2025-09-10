import axios, { type AxiosInstance, type AxiosRequestConfig, type AxiosResponse } from 'axios';
import type { Alert } from '../types/alert';
import type { ApiResponse, LoginResponse, User, PaginatedResponse } from '../types/api';
import type { HealthStatus } from '../types/health';
import type { MlEvent } from '../types/ml';
import type { OtaUpdate, RemoteCommand } from '../types/ota';
import type { TelemetryData } from '../types/telemetry';
import type { Truck, CreateTruckRequest, UpdateTruckRequest } from '../types/truck';


const api: AxiosInstance = axios.create({
  baseURL: process.env.REACT_APP_API_URL || 'http://localhost:8080/api',
  timeout: 10000,
});


// Add auth token to requests
api.interceptors.request.use(
  (config: AxiosRequestConfig) => {
    const token = localStorage.getItem('authToken');
    if (token && config.headers) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Handle auth errors
api.interceptors.response.use(
  (response: AxiosResponse) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('authToken');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);


// Auth
export async function login(username: string, password: string): Promise<ApiResponse<LoginResponse>> {
  const response = await api.post<ApiResponse<LoginResponse>>('/auth/login', { username, password });
  return response.data;
}

export async function validateToken(): Promise<ApiResponse<{ user: User }>> {
  const response = await api.get<ApiResponse<{ user: User }>>('/auth/validate');
  return response.data;
}

// Trucks
export async function getTrucks(params: Record<string, any> = {}): Promise<ApiResponse<PaginatedResponse<Truck>>> {
  const response = await api.get<ApiResponse<PaginatedResponse<Truck>>>('/trucks', { params });
  return response.data;
}

export async function getTruck(id: string): Promise<ApiResponse<Truck>> {
  const response = await api.get<ApiResponse<Truck>>(`/trucks/${id}`);
  return response.data;
}

export async function createTruckFn(truckData: CreateTruckRequest): Promise<ApiResponse<Truck>> {
  const response = await api.post<ApiResponse<Truck>>('/trucks', truckData);
  return response.data;
}

export async function updateTruck(id: string, truckData: UpdateTruckRequest): Promise<ApiResponse<Truck>> {
  const response = await api.put<ApiResponse<Truck>>(`/trucks/${id}`, truckData);
  return response.data;
}

export async function deleteTruck(id: string): Promise<ApiResponse<void>> {
  const response = await api.delete<ApiResponse<void>>(`/trucks/${id}`);
  return response.data;
}

// Telemetry
export async function getTruckTelemetry(truckId: string, params: Record<string, any> = {}): Promise<ApiResponse<PaginatedResponse<TelemetryData>>> {
  const response = await api.get<ApiResponse<PaginatedResponse<TelemetryData>>>(`/trucks/${truckId}/telemetry`, { params });
  return response.data;
}

// Alerts
export async function getAlerts(params: Record<string, any> = {}): Promise<ApiResponse<PaginatedResponse<Alert>>> {
  const response = await api.get<ApiResponse<PaginatedResponse<Alert>>>('/alerts', { params });
  return response.data;
}

export async function getAlert(id: string): Promise<ApiResponse<Alert>> {
  const response = await api.get<ApiResponse<Alert>>(`/alerts/${id}`);
  return response.data;
}

export async function acknowledgeAlert(id: string): Promise<ApiResponse<Alert>> {
  const response = await api.put<ApiResponse<Alert>>(`/alerts/${id}`, { status: 'Acknowledged' });
  return response.data;
}

export async function resolveAlert(id: string): Promise<ApiResponse<Alert>> {
  const response = await api.put<ApiResponse<Alert>>(`/alerts/${id}`, { status: 'Resolved' });
  return response.data;
}

// ML Events
export async function getMlEvents(params: Record<string, any> = {}): Promise<ApiResponse<PaginatedResponse<MlEvent>>> {
  const response = await api.get<ApiResponse<PaginatedResponse<MlEvent>>>('/ml-events', { params });
  return response.data;
}

export async function getMlEvent(id: string): Promise<ApiResponse<MlEvent>> {
  const response = await api.get<ApiResponse<MlEvent>>(`/ml-events/${id}`);
  return response.data;
}

// Health Status
export async function getHealthStatus(params: Record<string, any> = {}): Promise<ApiResponse<PaginatedResponse<HealthStatus>>> {
  const response = await api.get<ApiResponse<PaginatedResponse<HealthStatus>>>('/health', { params });
  return response.data;
}

export async function getHealth(id: string): Promise<ApiResponse<HealthStatus>> {
  const response = await api.get<ApiResponse<HealthStatus>>(`/health/${id}`);
  return response.data;
}

// OTA Updates
export async function getOtaUpdates(params: Record<string, any> = {}): Promise<ApiResponse<PaginatedResponse<OtaUpdate>>> {
  const response = await api.get<ApiResponse<PaginatedResponse<OtaUpdate>>>('/ota/updates', { params });
  return response.data;
}

export async function createOtaUpdate(updateData: CreateOtaUpdateRequest): Promise<ApiResponse<OtaUpdate>> {
  const response = await api.post<ApiResponse<OtaUpdate>>('/ota/updates', updateData);
  return response.data;
}

export async function updateOtaUpdate(id: string, updateData: UpdateOtaUpdateRequest): Promise<ApiResponse<OtaUpdate>> {
  const response = await api.put<ApiResponse<OtaUpdate>>(`/ota/updates/${id}`, updateData);
  return response.data;
}

// Remote Commands
export async function getRemoteCommands(params: Record<string, any> = {}): Promise<ApiResponse<PaginatedResponse<RemoteCommand>>> {
  const response = await api.get<ApiResponse<PaginatedResponse<RemoteCommand>>>('/ota/commands', { params });
  return response.data;
}

export async function createRemoteCommand(commandData: CreateRemoteCommandRequest): Promise<ApiResponse<RemoteCommand>> {
  const response = await api.post<ApiResponse<RemoteCommand>>('/ota/commands', commandData);
  return response.data;
}

export async function updateRemoteCommand(id: string, commandData: UpdateRemoteCommandRequest): Promise<ApiResponse<RemoteCommand>> {
  const response = await api.put<ApiResponse<RemoteCommand>>(`/ota/commands/${id}`, commandData);
  return response.data;
}
