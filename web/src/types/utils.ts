// Common utility types
export type Nullable<T> = T | null;
export type Optional<T> = T | undefined;
export type Dictionary<T> = Record<string, T>;
export type Uuid = string;

// Date and time types
export type Timestamp = string; // ISO 8601 format
export type Duration = number; // in milliseconds

// Location types
export type Location = [number, number]; // [longitude, latitude]
export type BoundingBox = [number, number, number, number]; // [minLng, minLat, maxLng, maxLat]

// Status types
export enum Status {
  Active = 'Active',
  Inactive = 'Inactive',
  Pending = 'Pending',
  Completed = 'Completed',
  Failed = 'Failed',
}

// Priority types
export enum Priority {
  Low = 'Low',
  Medium = 'Medium',
  High = 'High',
  Critical = 'Critical',
}

// Severity types
export enum Severity {
  Info = 'Info',
  Warning = 'Warning',
  Error = 'Error',
  Critical = 'Critical',
}

// Generic response types
export interface SuccessResponse<T = void> {
  success: true;
  data?: T;
  message?: string;
}

export interface ErrorResponse {
  success: false;
  error: string;
  code?: string;
  details?: any;
}

export type ApiResponse<T = void> = SuccessResponse<T> | ErrorResponse;

// Generic request types
export interface ListRequest {
  page?: number;
  limit?: number;
  sort?: string;
  order?: 'asc' | 'desc';
  filters?: Record<string, any>;
}

export interface CreateRequest<T> {
  data: T;
}

export interface UpdateRequest<T> {
  id: string;
  data: Partial<T>;
}

export interface DeleteRequest {
  id: string;
}

// Generic entity types
export interface BaseEntity {
  id: string;
  created_at: string;
  updated_at: string;
}

export interface TimestampedEntity extends BaseEntity {
  timestamp: string;
}

export interface NamedEntity extends BaseEntity {
  name: string;
  description?: string;
}

// Generic filter types
export interface Filter<T> {
  field: keyof T;
  operator: 'eq' | 'ne' | 'gt' | 'gte' | 'lt' | 'lte' | 'contains' | 'startsWith' | 'endsWith';
  value: any;
}

export interface Sort<T> {
  field: keyof T;
  order: 'asc' | 'desc';
}

// Generic pagination types
export interface Pagination {
  page: number;
  limit: number;
  total: number;
  pages: number;
}

export interface PaginatedResult<T> {
  data: T[];
  pagination: Pagination;
}

// Generic error types
export interface ErrorInfo {
  message: string;
  code: string;
  details?: any;
  timestamp: string;
}

// Generic event types
export interface Event<T> {
  type: string;
  data: T;
  timestamp: string;
  source?: string;
  metadata?: Record<string, any>;
}

// Generic configuration types
export interface Config {
  [key: string]: any;
}

// Generic statistics types
export interface Statistics {
  total: number;
  count: number;
  average: number;
  min: number;
  max: number;
  sum: number;
  [key: string]: any;
}

// Generic chart types
export interface ChartData {
  name: string;
  value: number;
  [key: string]: any;
}

export interface ChartSeries {
  name: string;
  data: number[];
  color?: string;
}

// Generic map types
export interface MapMarker {
  id: string;
  position: Location;
  title: string;
  description?: string;
  icon?: string;
  color?: string;
  size?: number;
}

export interface MapRoute {
  id: string;
  points: Location[];
  color?: string;
  width?: number;
  dashArray?: string;
}

// Generic table types
export interface TableColumn<T> {
  key: keyof T;
  title: string;
  width?: number;
  align?: 'left' | 'right' | 'center';
  render?: (value: any, record: T) => React.ReactNode;
}

export interface TableProps<T> {
  columns: TableColumn<T>[];
  data: T[];
  loading?: boolean;
  pagination?: Pagination;
  onPaginationChange?: (page: number, limit: number) => void;
  onSortChange?: (sort: Sort<T>) => void;
  onFilterChange?: (filters: Filter<T>[]) => void;
}

// Generic form types
export interface FormField<T> {
  name: keyof T;
  label: string;
  type: 'text' | 'number' | 'email' | 'password' | 'select' | 'checkbox' | 'radio' | 'date' | 'time' | 'datetime';
  required?: boolean;
  disabled?: boolean;
  placeholder?: string;
  options?: { value: string; label: string }[];
  validate?: (value: any) => string | null;
  render?: (props: any) => React.ReactNode;
}

export interface FormProps<T> {
  fields: FormField<T>[];
  initialValues?: T;
  onSubmit: (values: T) => void;
  onCancel?: () => void;
  loading?: boolean;
  submitLabel?: string;
  cancelLabel?: string;
}

// Generic modal types
export interface ModalProps {
  open: boolean;
  title: string;
  onClose: () => void;
  onSubmit?: () => void;
  submitLabel?: string;
  cancelLabel?: string;
  loading?: boolean;
  size?: 'sm' | 'md' | 'lg' | 'xl';
  fullWidth?: boolean;
  maxWidth?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | false;
}

// Generic notification types
export interface Notification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message: string;
  duration?: number;
  timestamp: string;
  read?: boolean;
}

// Generic settings types
export interface Settings {
  [key: string]: any;
}

// Generic permission types
export interface Permission {
  id: string;
  name: string;
  description?: string;
  actions: string[];
  resources: string[];
}

export interface Role {
  id: string;
  name: string;
  description?: string;
  permissions: Permission[];
}

// Generic audit types
export interface AuditLog {
  id: string;
  action: string;
  resource: string;
  resourceId: string;
  userId: string;
  userName: string;
  timestamp: string;
  ipAddress: string;
  userAgent: string;
  details?: any;
}

// Generic export types
export interface ExportOptions {
  format: 'csv' | 'json' | 'xlsx';
  filename?: string;
  columns?: string[];
  filters?: Record<string, any>;
  sort?: Record<string, 'asc' | 'desc'>;
}

// Generic import types
export interface ImportOptions {
  format: 'csv' | 'json' | 'xlsx';
  mapping?: Record<string, string>;
  validate?: boolean;
  dryRun?: boolean;
}

// Generic search types
export interface SearchOptions {
  query: string;
  fields?: string[];
  filters?: Record<string, any>;
  sort?: Record<string, 'asc' | 'desc'>;
  limit?: number;
  offset?: number;
}

export interface SearchResult<T> {
  data: T[];
  total: number;
  took: number;
  timed_out: boolean;
}

// Generic cache types
export interface CacheOptions {
  ttl?: number; // time to live in milliseconds
  maxSize?: number; // maximum number of items
  keyPrefix?: string;
}

// Generic queue types
export interface QueueOptions {
  name: string;
  concurrency?: number;
  timeout?: number;
  retry?: number;
  delay?: number;
}

// Generic worker types
export interface WorkerOptions {
  name: string;
  concurrency?: number;
  timeout?: number;
  retry?: number;
  delay?: number;
}

// Generic job types
export interface Job<T = any> {
  id: string;
  name: string;
  data: T;
  status: 'pending' | 'running' | 'completed' | 'failed' | 'retrying';
  attempts: number;
  maxAttempts: number;
  delay: number;
  timeout: number;
  createdAt: string;
  updatedAt: string;
  startedAt?: string;
  completedAt?: string;
  failedAt?: string;
  errorMessage?: string;
  stackTrace?: string;
}

// Generic rate limit types
export interface RateLimitOptions {
  windowMs: number;
  max: number;
  message?: string;
  statusCode?: number;
}

// Generic validation types
export interface ValidationRule {
  field: string;
  rule: 'required' | 'email' | 'min' | 'max' | 'minLength' | 'maxLength' | 'pattern' | 'custom';
  value?: any;
  message?: string;
}

export interface ValidationResult {
  valid: boolean;
  errors: Record<string, string[]>;
}

// Generic localization types
export interface Locale {
  code: string;
  name: string;
  nativeName: string;
  flag: string;
}
