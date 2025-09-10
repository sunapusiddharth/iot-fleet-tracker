export interface Notification {
  id: string;
  user_id: string;
  title: string;
  message: string;
  type: NotificationType;
  severity: NotificationSeverity;
  read: boolean;
  archived: boolean;
  action_url: string | null;
  created_at: string;
  read_at: string | null;
  archived_at: string | null;
}

export enum NotificationType {
  Alert = 'alert',
  System = 'system',
  Maintenance = 'maintenance',
  Report = 'report',
  Message = 'message',
  Reminder = 'reminder',
  Update = 'update',
}

export enum NotificationSeverity {
  Info = 'info',
  Success = 'success',
  Warning = 'warning',
  Error = 'error',
  Critical = 'critical',
}

export interface NotificationPreference {
  id: string;
  user_id: string;
  notification_type: NotificationType;
  channel: NotificationChannel;
  enabled: boolean;
  quiet_hours_start: string; // HH:MM format
  quiet_hours_end: string; // HH:MM format
  created_at: string;
  updated_at: string;
}

export enum NotificationChannel {
  Email = 'email',
  SMS = 'sms',
  Push = 'push',
  Webhook = 'webhook',
  Slack = 'slack',
  Teams = 'teams',
}

export interface NotificationTemplate {
  id: string;
  name: string;
  description: string;
  type: NotificationType;
  subject_template: string;
  body_template: string;
  channels: NotificationChannel[];
  variables: string[];
  created_at: string;
  updated_at: string;
}

export interface NotificationRule {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  conditions: NotificationCondition[];
  actions: NotificationAction[];
  created_at: string;
  updated_at: string;
}

export interface NotificationCondition {
  field: string;
  operator: 'equals' | 'not_equals' | 'greater_than' | 'less_than' | 'contains' | 'starts_with' | 'ends_with';
  value: any;
  group: 'and' | 'or'; // For grouping conditions
}

export interface NotificationAction {
  type: 'send_notification' | 'send_email' | 'send_sms' | 'call_webhook' | 'create_ticket';
  template_id: string;
  recipients: string[];
  parameters: Record<string, any>;
}

export interface ScheduledNotification {
  id: string;
  name: string;
  description: string;
  schedule: NotificationSchedule;
  template_id: string;
  recipients: string[];
  parameters: Record<string, any>;
  last_sent: string | null;
  next_scheduled: string;
  active: boolean;
  created_at: string;
  updated_at: string;
}

export interface NotificationSchedule {
  frequency: 'once' | 'daily' | 'weekly' | 'monthly' | 'cron';
  time: string; // HH:MM format or cron expression
  timezone: string;
  day_of_week: number | null; // 0-6 (Sunday-Saturday)
  day_of_month: number | null; // 1-31
  cron_expression: string | null;
}

export interface NotificationGroup {
  id: string;
  name: string;
  description: string;
  members: string[]; // User IDs
  created_at: string;
  updated_at: string;
}

export interface NotificationDelivery {
  id: string;
  notification_id: string;
  channel: NotificationChannel;
  recipient: string;
  status: 'pending' | 'sent' | 'delivered' | 'failed';
  sent_at: string | null;
  delivered_at: string | null;
  failed_at: string | null;
  error_message: string | null;
  provider_response: string | null;
  created_at: string;
}

export interface NotificationSetting {
  id: string;
  user_id: string;
  key: string;
  value: any;
  created_at: string;
  updated_at: string;
}

export interface NotificationEvent {
  id: string;
  event_type: string;
  source: string;
  payload: Record<string, any>;
  processed: boolean;
  processed_at: string | null;
  created_at: string;
}

export interface NotificationSubscription {
  id: string;
  user_id: string;
  event_type: string;
  subscribed: boolean;
  created_at: string;
  updated_at: string;
}

export interface NotificationDigest {
  id: string;
  user_id: string;
  title: string;
  notifications: Notification[];
  frequency: 'hourly' | 'daily' | 'weekly';
  next_digest: string;
  last_digest: string | null;
  created_at: string;
  updated_at: string;
}

export interface NotificationWebhook {
  id: string;
  name: string;
  description: string;
  url: string;
  secret: string;
  events: string[];
  active: boolean;
  created_at: string;
  updated_at: string;
}

export interface NotificationWebhookEvent {
  id: string;
  webhook_id: string;
  event_type: string;
  payload: Record<string, any>;
  delivered_at: string | null;
  delivery_status: 'pending' | 'delivered' | 'failed';
  response_code: number | null;
  response_body: string | null;
  retry_count: number;
  created_at: string;
}

export interface NotificationAuditLog {
  id: string;
  user_id: string;
  action: string;
  resource: string;
  resource_id: string;
  ip_address: string;
  user_agent: string;
  timestamp: string;
  details: Record<string, any>;
}