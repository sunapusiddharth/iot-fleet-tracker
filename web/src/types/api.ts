// API Response Types
export interface ApiResponse<T> {
  data: T;
  success: boolean;
  message?: string;
  error?: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  pages?: number;
}

// API Request Types
export interface ApiRequest {
  [key: string]: any;
}

export interface PaginationParams {
  page?: number;
  limit?: number;
  offset?: number;
}

export interface FilterParams {
  [key: string]: any;
}

export interface SortParams {
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}

// Auth Types
export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export interface User {
  id: string;
  username: string;
  name: string;
  role: string;
  created_at: string;
}

// WebSocket Types
export enum WebSocketEventType {
  Telemetry = 'telemetry',
  Alert = 'alert',
  MlEvent = 'ml_event',
  HealthStatus = 'health_status',
}

export interface WebSocketMessage<T> {
  type: WebSocketEventType;
  data: T;
  timestamp: string;
}

// API Response Types
export interface ApiResponse<T> {
  data: T;
  success: boolean;
  message?: string;
  error?: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  pages?: number;
}

// Auth Types
export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export interface ValidateTokenResponse {
  user: User;
}

export interface User {
  id: string;
  username: string;
  name: string;
  role: string;
  created_at: string;
}