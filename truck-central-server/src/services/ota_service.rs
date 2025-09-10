use crate::models::ota::{OtaUpdate, RemoteCommand, OtaSummary, OtaStats, UpdateTarget, UpdatePriority, OtaStatus, CommandType, CommandStatus};
use crate::server::storage::StorageManager;
use crate::server::realtime::RealtimeManager;
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::sync::Arc;

pub struct OtaService {
    storage_manager: Arc<StorageManager>,
    realtime_manager: Arc<RealtimeManager>,
}

impl OtaService {
    pub fn new(storage_manager: Arc<StorageManager>, realtime_manager: Arc<RealtimeManager>) -> Self {
        Self {
            storage_manager,
            realtime_manager,
        }
    }

    pub async fn create_ota_update(&self, ota_update: &OtaUpdate) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_ota_update(ota_update).await?;
        
        // In production, broadcast to targeted trucks via MQTT or other protocol
        
        Ok(())
    }

    pub async fn get_ota_update(&self, id: Uuid) -> Result<OtaUpdate, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ota_update = store.get_ota_update(id).await?;
        
        Ok(ota_update)
    }

    pub async fn update_ota_update(&self, id: Uuid, status: OtaStatus, progress_percent: f32, last_error: Option<String>) -> Result<OtaUpdate, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let mut ota_update = store.get_ota_update(id).await?;
        ota_update.status = status;
        ota_update.progress_percent = progress_percent;
        ota_update.last_error = last_error;
        ota_update.updated_at = Utc::now();
        
        if status == OtaStatus::Success || status == OtaStatus::Failed || status == OtaStatus::Rollback {
            ota_update.completed_at = Some(Utc::now());
        }
        
        store.update_ota_update(&ota_update).await?;
        
        Ok(ota_update)
    }

    pub async fn list_ota_updates(&self, limit: i64, offset: i64) -> Result<Vec<OtaSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ota_updates = store.list_ota_updates(limit, offset).await?;
        
        Ok(ota_updates)
    }

    pub async fn get_ota_updates_by_target(&self, target: UpdateTarget, limit: i64, offset: i64) -> Result<Vec<OtaSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ota_updates = store.get_ota_updates_by_target(target, limit, offset).await?;
        
        Ok(ota_updates)
    }

    pub async fn get_ota_updates_by_priority(&self, priority: UpdatePriority, limit: i64, offset: i64) -> Result<Vec<OtaSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ota_updates = store.get_ota_updates_by_priority(priority, limit, offset).await?;
        
        Ok(ota_updates)
    }

    pub async fn get_ota_updates_by_status(&self, status: OtaStatus, limit: i64, offset: i64) -> Result<Vec<OtaSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ota_updates = store.get_ota_updates_by_status(status, limit, offset).await?;
        
        Ok(ota_updates)
    }

    pub async fn get_ota_updates_by_truck(&self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<OtaSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ota_updates = store.get_ota_updates_by_truck(truck_id, limit, offset).await?;
        
        Ok(ota_updates)
    }

    pub async fn get_ota_updates_by_time_range(&self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<OtaSummary>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let ota_updates = store.get_ota_updates_by_time_range(start_time, end_time, limit, offset).await?;
        
        Ok(ota_updates)
    }

    pub async fn get_ota_stats(&self) -> Result<OtaStats, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let stats = store.get_ota_stats().await?;
        
        Ok(stats)
    }

    pub async fn create_remote_command(&self, command: &RemoteCommand) -> Result<(), Box<dyn std::error::Error>> {
        self.storage_manager.store_remote_command(command).await?;
        
        // In production, send command to targeted trucks via MQTT or other protocol
        
        Ok(())
    }

    pub async fn get_remote_command(&self, id: Uuid) -> Result<RemoteCommand, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let command = store.get_remote_command(id).await?;
        
        Ok(command)
    }

    pub async fn update_remote_command(&self, id: Uuid, status: CommandStatus, result: Option<serde_json::Value>, error: Option<String>) -> Result<RemoteCommand, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let mut command = store.get_remote_command(id).await?;
        command.status = status;
        command.result = result;
        command.error = error;
        command.completed_at = Some(Utc::now());
        command.updated_at = Utc::now();
        
        store.update_remote_command(&command).await?;
        
        Ok(command)
    }

    pub async fn list_remote_commands(&self, limit: i64, offset: i64) -> Result<Vec<RemoteCommand>, Box<dyn std::error::Error>> {
        let document_store = self.storage_manager.get_document_store().await;
        let mut store = document_store.lock().await;
        
        let commands = store.list_remote_commands(limit, offset).await?;
        
        Ok(commands)
    }
}