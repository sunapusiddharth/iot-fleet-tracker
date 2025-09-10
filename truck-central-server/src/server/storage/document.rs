use mongodb::{Client, Database, Collection};
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOptions, UpdateOptions};
use crate::models::truck::{Truck, TruckSummary, TripSummary, MaintenanceRecord};
use crate::models::telemetry::TelemetryData;
use crate::models::alert::Alert;
use crate::models::ml::MlEvent;
use crate::models::health::HealthStatus;
use crate::models::ota::{OtaUpdate, RemoteCommand};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

pub struct DocumentStore {
    database: Database,
    trucks: Collection<Document>,
    telemetry: Collection<Document>,
    alerts: Collection<Document>,
    ml_events: Collection<Document>,
    health_status: Collection<Document>,
    ota_updates: Collection<Document>,
    remote_commands: Collection<Document>,
    trips: Collection<Document>,
    maintenance_records: Collection<Document>,
}

impl DocumentStore {
    pub fn new(client: Client, database_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let database = client.database(database_name);
        
        let trucks = database.collection("trucks");
        let telemetry = database.collection("telemetry");
        let alerts = database.collection("alerts");
        let ml_events = database.collection("ml_events");
        let health_status = database.collection("health_status");
        let ota_updates = database.collection("ota_updates");
        let remote_commands = database.collection("remote_commands");
        let trips = database.collection("trips");
        let maintenance_records = database.collection("maintenance_records");
        
        // Create indexes
        trucks.create_index(doc!{"truck_id": 1}, None).await?;
        trucks.create_index(doc!{"status": 1}, None).await?;
        trucks.create_index(doc!{"last_seen": -1}, None).await?;
        
        telemetry.create_index(doc!{"truck_id": 1}, None).await?;
        telemetry.create_index(doc!{"timestamp": -1}, None).await?;
        telemetry.create_index(doc!{"location": "2dsphere"}, None).await?;
        
        alerts.create_index(doc!{"truck_id": 1}, None).await?;
        alerts.create_index(doc!{"alert_type": 1}, None).await?;
        alerts.create_index(doc!{"severity": 1}, None).await?;
        alerts.create_index(doc!{"status": 1}, None).await?;
        alerts.create_index(doc!{"triggered_at": -1}, None).await?;
        
        ml_events.create_index(doc!{"truck_id": 1}, None).await?;
        ml_events.create_index(doc!{"model_name": 1}, None).await?;
        ml_events.create_index(doc!{"timestamp": -1}, None).await?;
        
        health_status.create_index(doc!{"truck_id": 1}, None).await?;
        health_status.create_index(doc!{"status": 1}, None).await?;
        health_status.create_index(doc!{"timestamp": -1}, None).await?;
        
        ota_updates.create_index(doc!{"truck_id": 1}, None).await?;
        ota_updates.create_index(doc!{"status": 1}, None).await?;
        ota_updates.create_index(doc!{"created_at": -1}, None).await?;
        
        remote_commands.create_index(doc!{"truck_id": 1}, None).await?;
        remote_commands.create_index(doc!{"status": 1}, None).await?;
        remote_commands.create_index(doc!{"issued_at": -1}, None).await?;
        
        trips.create_index(doc!{"truck_id": 1}, None).await?;
        trips.create_index(doc!{"start_time": -1}, None).await?;
        
        maintenance_records.create_index(doc!{"truck_id": 1}, None).await?;
        maintenance_records.create_index(doc!{"performed_at": -1}, None).await?;
        
        Ok(Self {
            database,
            trucks,
            telemetry,
            alerts,
            ml_events,
            health_status,
            ota_updates,
            remote_commands,
            trips,
            maintenance_records,
        })
    }
    
    pub async fn store_truck(&mut self, truck: &Truck) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(truck)?;
        self.trucks.replace_one(doc!{"id": truck.id}, doc, UpdateOptions::builder().upsert(true).build()).await?;
        Ok(())
    }
    
    pub async fn get_truck(&mut self, id: Uuid) -> Result<Truck, Box<dyn std::error::Error>> {
        let filter = doc!{"id": id};
        let doc = self.trucks.find_one(filter, None).await?.ok_or("Truck not found")?;
        let truck: Truck = bson::from_document(doc)?;
        Ok(truck)
    }
    
    pub async fn update_truck(&mut self, truck: &Truck) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(truck)?;
        self.trucks.replace_one(doc!{"id": truck.id}, doc, None).await?;
        Ok(())
    }
    
    pub async fn delete_truck(&mut self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc!{"id": id};
        self.trucks.delete_one(filter, None).await?;
        Ok(())
    }
    
    pub async fn list_trucks(&mut self, limit: i64, offset: i64) -> Result<Vec<TruckSummary>, Box<dyn std::error::Error>> {
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"last_seen": -1}).build();
        let mut cursor = self.trucks.find(None, options).await?;
        let mut trucks = Vec::new();
        while let Some(doc) = cursor.next().await {
            let truck: Truck = bson::from_document(doc?)?;
            let summary = TruckSummary {
                id: truck.id,
                truck_id: truck.truck_id,
                model: truck.model,
                make: truck.make,
                license_plate: truck.license_plate,
                status: truck.status,
                last_seen: truck.last_seen,
                location: truck.location,
                speed_kmh: None,
                heading: None,
                active_alerts: 0,
                health_score: 100.0,
            };
            trucks.push(summary);
        }
        Ok(trucks)
    }
    
    pub async fn store_telemetry(&mut self, telemetry: &TelemetryData) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(telemetry)?;
        self.telemetry.insert_one(doc, None).await?;
        Ok(())
    }
    
    pub async fn get_telemetry(&mut self, id: Uuid) -> Result<TelemetryData, Box<dyn std::error::Error>> {
        let filter = doc!{"id": id};
        let doc = self.telemetry.find_one(filter, None).await?.ok_or("Telemetry not found")?;
        let telemetry: TelemetryData = bson::from_document(doc)?;
        Ok(telemetry)
    }
    
    pub async fn get_truck_telemetry(&mut self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<TelemetryData>, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.telemetry.find(filter, options).await?;
        let mut telemetry = Vec::new();
        while let Some(doc) = cursor.next().await {
            let t: TelemetryData = bson::from_document(doc?)?;
            telemetry.push(t);
        }
        Ok(telemetry)
    }
    
    pub async fn get_latest_telemetry(&mut self, truck_id: Uuid) -> Result<TelemetryData, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().sort(doc!{"timestamp": -1}).limit(1).build();
        let doc = self.telemetry.find_one(filter, options).await?.ok_or("No telemetry found")?;
        let telemetry: TelemetryData = bson::from_document(doc)?;
        Ok(telemetry)
    }
    
    pub async fn get_telemetry_by_time_range(&mut self, truck_id: Uuid, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<TelemetryData>, Box<dyn std::error::Error>> {
        let filter = doc!{
            "truck_id": truck_id,
            "timestamp": {
                "$gte": start_time,
                "$lte": end_time
            }
        };
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.telemetry.find(filter, options).await?;
        let mut telemetry = Vec::new();
        while let Some(doc) = cursor.next().await {
            let t: TelemetryData = bson::from_document(doc?)?;
            telemetry.push(t);
        }
        Ok(telemetry)
    }
    
    pub async fn store_alert(&mut self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(alert)?;
        self.alerts.insert_one(doc, None).await?;
        Ok(())
    }
    
    pub async fn get_alert(&mut self, id: Uuid) -> Result<Alert, Box<dyn std::error::Error>> {
        let filter = doc!{"id": id};
        let doc = self.alerts.find_one(filter, None).await?.ok_or("Alert not found")?;
        let alert: Alert = bson::from_document(doc)?;
        Ok(alert)
    }
    
    pub async fn update_alert(&mut self, alert: &Alert) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(alert)?;
        self.alerts.replace_one(doc!{"id": alert.id}, doc, None).await?;
        Ok(())
    }
    
    pub async fn list_alerts(&mut self, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"triggered_at": -1}).build();
        let mut cursor = self.alerts.find(None, options).await?;
        let mut alerts = Vec::new();
        while let Some(doc) = cursor.next().await {
            let alert: Alert = bson::from_document(doc?)?;
            let summary = AlertSummary {
                id: alert.id,
                alert_id: alert.alert_id,
                truck_id: alert.truck_id,
                alert_type: alert.alert_type,
                severity: alert.severity,
                message: alert.message,
                triggered_at: alert.triggered_at,
                status: alert.status,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            alerts.push(summary);
        }
        Ok(alerts)
    }
    
    pub async fn get_truck_alerts(&mut self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"triggered_at": -1}).build();
        let mut cursor = self.alerts.find(filter, options).await?;
        let mut alerts = Vec::new();
        while let Some(doc) = cursor.next().await {
            let alert: Alert = bson::from_document(doc?)?;
            let summary = AlertSummary {
                id: alert.id,
                alert_id: alert.alert_id,
                truck_id: alert.truck_id,
                alert_type: alert.alert_type,
                severity: alert.severity,
                message: alert.message,
                triggered_at: alert.triggered_at,
                status: alert.status,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            alerts.push(summary);
        }
        Ok(alerts)
    }
    
    pub async fn get_alerts_by_type(&mut self, alert_type: crate::models::alert::AlertType, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"alert_type": format!("{:?}", alert_type)};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"triggered_at": -1}).build();
        let mut cursor = self.alerts.find(filter, options).await?;
        let mut alerts = Vec::new();
        while let Some(doc) = cursor.next().await {
            let alert: Alert = bson::from_document(doc?)?;
            let summary = AlertSummary {
                id: alert.id,
                alert_id: alert.alert_id,
                truck_id: alert.truck_id,
                alert_type: alert.alert_type,
                severity: alert.severity,
                message: alert.message,
                triggered_at: alert.triggered_at,
                status: alert.status,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            alerts.push(summary);
        }
        Ok(alerts)
    }
    
    pub async fn get_alerts_by_severity(&mut self, severity: crate::models::alert::AlertSeverity, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"severity": format!("{:?}", severity)};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"triggered_at": -1}).build();
        let mut cursor = self.alerts.find(filter, options).await?;
        let mut alerts = Vec::new();
        while let Some(doc) = cursor.next().await {
            let alert: Alert = bson::from_document(doc?)?;
            let summary = AlertSummary {
                id: alert.id,
                alert_id: alert.alert_id,
                truck_id: alert.truck_id,
                alert_type: alert.alert_type,
                severity: alert.severity,
                message: alert.message,
                triggered_at: alert.triggered_at,
                status: alert.status,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            alerts.push(summary);
        }
        Ok(alerts)
    }
    
    pub async fn get_alerts_by_status(&mut self, status: crate::models::alert::AlertStatus, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"status": format!("{:?}", status)};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"triggered_at": -1}).build();
        let mut cursor = self.alerts.find(filter, options).await?;
        let mut alerts = Vec::new();
        while let Some(doc) = cursor.next().await {
            let alert: Alert = bson::from_document(doc?)?;
            let summary = AlertSummary {
                id: alert.id,
                alert_id: alert.alert_id,
                truck_id: alert.truck_id,
                alert_type: alert.alert_type,
                severity: alert.severity,
                message: alert.message,
                triggered_at: alert.triggered_at,
                status: alert.status,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            alerts.push(summary);
        }
        Ok(alerts)
    }
    
    pub async fn get_alerts_by_time_range(&mut self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<AlertSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{
            "triggered_at": {
                "$gte": start_time,
                "$lte": end_time
            }
        };
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"triggered_at": -1}).build();
        let mut cursor = self.alerts.find(filter, options).await?;
        let mut alerts = Vec::new();
        while let Some(doc) = cursor.next().await {
            let alert: Alert = bson::from_document(doc?)?;
            let summary = AlertSummary {
                id: alert.id,
                alert_id: alert.alert_id,
                truck_id: alert.truck_id,
                alert_type: alert.alert_type,
                severity: alert.severity,
                message: alert.message,
                triggered_at: alert.triggered_at,
                status: alert.status,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            alerts.push(summary);
        }
        Ok(alerts)
    }
    
    pub async fn get_alert_stats(&mut self) -> Result<crate::models::alert::AlertStats, Box<dyn std::error::Error>> {
        let total_alerts = self.alerts.count_documents(doc!{}, None).await?;
        let active_alerts = self.alerts.count_documents(doc!{"status": "Triggered"}, None).await?;
        let acknowledged_alerts = self.alerts.count_documents(doc!{"status": "Acknowledged"}, None).await?;
        let resolved_alerts = self.alerts.count_documents(doc!{"status": "Resolved"}, None).await?;
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$severity",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.alerts.aggregate(pipeline, None).await?;
        let mut by_severity = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let severity = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_severity.insert(severity, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$alert_type",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.alerts.aggregate(pipeline, None).await?;
        let mut by_type = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let alert_type = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_type.insert(alert_type, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$truck_id",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.alerts.aggregate(pipeline, None).await?;
        let mut by_truck = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let truck_id = result.get("truck_id").unwrap().as_str().unwrap();
            let truck_uuid = Uuid::parse_str(truck_id).unwrap_or(Uuid::nil());
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_truck.insert(truck_uuid, count);
        }
        
        let now = Utc::now();
        let last_24_hours = self.alerts.count_documents(doc!{"triggered_at": {"$gte": now - chrono::Duration::hours(24)}}, None).await?;
        let last_7_days = self.alerts.count_documents(doc!{"triggered_at": {"$gte": now - chrono::Duration::days(7)}}, None).await?;
        let last_30_days = self.alerts.count_documents(doc!{"triggered_at": {"$gte": now - chrono::Duration::days(30)}}, None).await?;
        
        Ok(crate::models::alert::AlertStats {
            total_alerts,
            active_alerts,
            acknowledged_alerts,
            resolved_alerts,
            by_severity,
            by_type,
            by_truck,
            last_24_hours,
            last_7_days,
            last_30_days,
        })
    }
    
    pub async fn get_top_alert_trucks(&mut self, limit: i64) -> Result<Vec<(Uuid, i64)>, Box<dyn std::error::Error>> {
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$truck_id",
                "count": {"$sum": 1}
            }},
            doc!{"$sort": {"count": -1}},
            doc!{"$limit": limit}
        ];
        let mut cursor = self.alerts.aggregate(pipeline, None).await?;
        let mut top_trucks = Vec::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let truck_id = result.get("truck_id").unwrap().as_str().unwrap();
            let truck_uuid = Uuid::parse_str(truck_id).unwrap_or(Uuid::nil());
            let count = result.get_i64("count").unwrap_or(0) as i64;
            top_trucks.push((truck_uuid, count));
        }
        Ok(top_trucks)
    }
    
    pub async fn get_alerts_by_type_grouped(&mut self) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$alert_type",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.alerts.aggregate(pipeline, None).await?;
        let mut alerts_by_type = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let alert_type = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            alerts_by_type.insert(alert_type, count);
        }
        Ok(alerts_by_type)
    }
    
    pub async fn get_alerts_by_severity_grouped(&mut self) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$severity",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.alerts.aggregate(pipeline, None).await?;
        let mut alerts_by_severity = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let severity = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            alerts_by_severity.insert(severity, count);
        }
        Ok(alerts_by_severity)
    }
    
    pub async fn get_active_alerts_count(&mut self, truck_id: Uuid) -> Result<i32, Box<dyn std::error::Error>> {
        let filter = doc!{
            "truck_id": truck_id,
            "status": "Triggered"
        };
        let count = self.alerts.count_documents(filter, None).await? as i32;
        Ok(count)
    }
    
    pub async fn store_ml_event(&mut self, ml_event: &MlEvent) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(ml_event)?;
        self.ml_events.insert_one(doc, None).await?;
        Ok(())
    }
    
    pub async fn get_ml_event(&mut self, id: Uuid) -> Result<MlEvent, Box<dyn std::error::Error>> {
        let filter = doc!{"id": id};
        let doc = self.ml_events.find_one(filter, None).await?.ok_or("ML event not found")?;
        let ml_event: MlEvent = bson::from_document(doc)?;
        Ok(ml_event)
    }
    
    pub async fn list_ml_events(&mut self, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.ml_events.find(None, options).await?;
        let mut ml_events = Vec::new();
        while let Some(doc) = cursor.next().await {
            let event: MlEvent = bson::from_document(doc?)?;
            let summary = MlEventSummary {
                id: event.id,
                event_id: event.event_id,
                truck_id: event.truck_id,
                model_name: event.model_name,
                result_type: format!("{:?}", event.result),
                confidence: event.confidence,
                timestamp: event.timestamp,
                is_alert: event.confidence > 0.8,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            ml_events.push(summary);
        }
        Ok(ml_events)
    }
    
    pub async fn get_truck_ml_events(&mut self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.ml_events.find(filter, options).await?;
        let mut ml_events = Vec::new();
        while let Some(doc) = cursor.next().await {
            let event: MlEvent = bson::from_document(doc?)?;
            let summary = MlEventSummary {
                id: event.id,
                event_id: event.event_id,
                truck_id: event.truck_id,
                model_name: event.model_name,
                result_type: format!("{:?}", event.result),
                confidence: event.confidence,
                timestamp: event.timestamp,
                is_alert: event.confidence > 0.8,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            ml_events.push(summary);
        }
        Ok(ml_events)
    }
    
    pub async fn get_ml_events_by_model(&mut self, model_name: &str, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"model_name": model_name};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.ml_events.find(filter, options).await?;
        let mut ml_events = Vec::new();
        while let Some(doc) = cursor.next().await {
            let event: MlEvent = bson::from_document(doc?)?;
            let summary = MlEventSummary {
                id: event.id,
                event_id: event.event_id,
                truck_id: event.truck_id,
                model_name: event.model_name,
                result_type: format!("{:?}", event.result),
                confidence: event.confidence,
                timestamp: event.timestamp,
                is_alert: event.confidence > 0.8,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            ml_events.push(summary);
        }
        Ok(ml_events)
    }
    
    pub async fn get_ml_events_by_result_type(&mut self, result_type: &str, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"result": {"$regex": result_type, "$options": "i"}};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.ml_events.find(filter, options).await?;
        let mut ml_events = Vec::new();
        while let Some(doc) = cursor.next().await {
            let event: MlEvent = bson::from_document(doc?)?;
            let summary = MlEventSummary {
                id: event.id,
                event_id: event.event_id,
                truck_id: event.truck_id,
                model_name: event.model_name,
                result_type: format!("{:?}", event.result),
                confidence: event.confidence,
                timestamp: event.timestamp,
                is_alert: event.confidence > 0.8,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            ml_events.push(summary);
        }
        Ok(ml_events)
    }
    
    pub async fn get_ml_events_by_time_range(&mut self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<MlEventSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{
            "timestamp": {
                "$gte": start_time,
                "$lte": end_time
            }
        };
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.ml_events.find(filter, options).await?;
        let mut ml_events = Vec::new();
        while let Some(doc) = cursor.next().await {
            let event: MlEvent = bson::from_document(doc?)?;
            let summary = MlEventSummary {
                id: event.id,
                event_id: event.event_id,
                truck_id: event.truck_id,
                model_name: event.model_name,
                result_type: format!("{:?}", event.result),
                confidence: event.confidence,
                timestamp: event.timestamp,
                is_alert: event.confidence > 0.8,
                truck_license_plate: "".to_string(),
                truck_model: "".to_string(),
                truck_make: "".to_string(),
            };
            ml_events.push(summary);
        }
        Ok(ml_events)
    }
    
    pub async fn get_ml_stats(&mut self) -> Result<crate::models::ml::MlStats, Box<dyn std::error::Error>> {
        let total_events = self.ml_events.count_documents(doc!{}, None).await?;
        let alert_events = self.ml_events.count_documents(doc!{"confidence": {"$gt": 0.8}}, None).await?;
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$model_name",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.ml_events.aggregate(pipeline, None).await?;
        let mut by_model = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let model_name = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_model.insert(model_name, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": {"$literal": "result"},
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.ml_events.aggregate(pipeline, None).await?;
        let mut by_result = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let result_type = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_result.insert(result_type, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$truck_id",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.ml_events.aggregate(pipeline, None).await?;
        let mut by_truck = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let truck_id = result.get("truck_id").unwrap().as_str().unwrap();
            let truck_uuid = Uuid::parse_str(truck_id).unwrap_or(Uuid::nil());
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_truck.insert(truck_uuid, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": null,
                "avg_confidence": {"$avg": "$confidence"},
                "avg_latency_ms": {"$avg": "$latency_ms"}
            }}
        ];
        let mut cursor = self.ml_events.aggregate(pipeline, None).await?;
        let mut avg_confidence = 0.0;
        let mut avg_latency_ms = 0.0;
        if let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            avg_confidence = result.get_f64("avg_confidence").unwrap_or(0.0) as f32;
            avg_latency_ms = result.get_f64("avg_latency_ms").unwrap_or(0.0) as f32;
        }
        
        let now = Utc::now();
        let last_24_hours = self.ml_events.count_documents(doc!{"timestamp": {"$gte": now - chrono::Duration::hours(24)}}, None).await?;
        let last_7_days = self.ml_events.count_documents(doc!{"timestamp": {"$gte": now - chrono::Duration::days(7)}}, None).await?;
        let last_30_days = self.ml_events.count_documents(doc!{"timestamp": {"$gte": now - chrono::Duration::days(30)}}, None).await?;
        
        Ok(crate::models::ml::MlStats {
            total_events,
            alert_events,
            by_model,
            by_result,
            by_truck,
            avg_confidence,
            avg_latency_ms,
            last_24_hours,
            last_7_days,
            last_30_days,
        })
    }
    
    pub async fn get_top_ml_models(&mut self, limit: i64) -> Result<Vec<(String, i64)>, Box<dyn std::error::Error>> {
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$model_name",
                "count": {"$sum": 1}
            }},
            doc!{"$sort": {"count": -1}},
            doc!{"$limit": limit}
        ];
        let mut cursor = self.ml_events.aggregate(pipeline, None).await?;
        let mut top_models = Vec::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let model_name = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            top_models.push((model_name, count));
        }
        Ok(top_models)
    }
    
    pub async fn get_ml_events_by_result_grouped(&mut self) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
        let pipeline = vec![
            doc!{"$group": {
                "_id": {"$literal": "result"},
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.ml_events.aggregate(pipeline, None).await?;
        let mut events_by_result = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let result_type = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            events_by_result.insert(result_type, count);
        }
        Ok(events_by_result)
    }
    
    pub async fn get_top_ml_trucks(&mut self, limit: i64) -> Result<Vec<(Uuid, i64)>, Box<dyn std::error::Error>> {
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$truck_id",
                "count": {"$sum": 1}
            }},
            doc!{"$sort": {"count": -1}},
            doc!{"$limit": limit}
        ];
        let mut cursor = self.ml_events.aggregate(pipeline, None).await?;
        let mut top_trucks = Vec::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let truck_id = result.get("truck_id").unwrap().as_str().unwrap();
            let truck_uuid = Uuid::parse_str(truck_id).unwrap_or(Uuid::nil());
            let count = result.get_i64("count").unwrap_or(0) as i64;
            top_trucks.push((truck_uuid, count));
        }
        Ok(top_trucks)
    }
    
    pub async fn store_health_status(&mut self, health_status: &HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(health_status)?;
        self.health_status.insert_one(doc, None).await?;
        Ok(())
    }
    
    pub async fn get_health_status(&mut self, id: Uuid) -> Result<HealthStatus, Box<dyn std::error::Error>> {
        let filter = doc!{"id": id};
        let doc = self.health_status.find_one(filter, None).await?.ok_or("Health status not found")?;
        let health_status: HealthStatus = bson::from_document(doc)?;
        Ok(health_status)
    }
    
    pub async fn list_health_status(&mut self, limit: i64, offset: i64) -> Result<Vec<crate::models::health::HealthSummary>, Box<dyn std::error::Error>> {
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.health_status.find(None, options).await?;
        let mut health_status = Vec::new();
        while let Some(doc) = cursor.next().await {
            let status: HealthStatus = bson::from_document(doc?)?;
            let summary = crate::models::health::HealthSummary {
                truck_id: status.truck_id,
                last_timestamp: status.timestamp,
                status: status.status,
                cpu_percent: status.resources.cpu_percent,
                memory_percent: status.resources.memory_percent,
                disk_percent: status.resources.disk_percent,
                temperature_c: status.resources.temperature_c,
                uptime_sec: status.resources.uptime_sec,
                active_alerts: status.alerts.len() as i32,
                health_score: 100.0,
            };
            health_status.push(summary);
        }
        Ok(health_status)
    }
    
    pub async fn get_truck_health_status(&mut self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<crate::models::health::HealthSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.health_status.find(filter, options).await?;
        let mut health_status = Vec::new();
        while let Some(doc) = cursor.next().await {
            let status: HealthStatus = bson::from_document(doc?)?;
            let summary = crate::models::health::HealthSummary {
                truck_id: status.truck_id,
                last_timestamp: status.timestamp,
                status: status.status,
                cpu_percent: status.resources.cpu_percent,
                memory_percent: status.resources.memory_percent,
                disk_percent: status.resources.disk_percent,
                temperature_c: status.resources.temperature_c,
                uptime_sec: status.resources.uptime_sec,
                active_alerts: status.alerts.len() as i32,
                health_score: 100.0,
            };
            health_status.push(summary);
        }
        Ok(health_status)
    }
    
    pub async fn get_health_status_by_type(&mut self, status_type: crate::models::health::HealthStatusType, limit: i64, offset: i64) -> Result<Vec<crate::models::health::HealthSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"status": format!("{:?}", status_type)};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.health_status.find(filter, options).await?;
        let mut health_status = Vec::new();
        while let Some(doc) = cursor.next().await {
            let status: HealthStatus = bson::from_document(doc?)?;
            let summary = crate::models::health::HealthSummary {
                truck_id: status.truck_id,
                last_timestamp: status.timestamp,
                status: status.status,
                cpu_percent: status.resources.cpu_percent,
                memory_percent: status.resources.memory_percent,
                disk_percent: status.resources.disk_percent,
                temperature_c: status.resources.temperature_c,
                uptime_sec: status.resources.uptime_sec,
                active_alerts: status.alerts.len() as i32,
                health_score: 100.0,
            };
            health_status.push(summary);
        }
        Ok(health_status)
    }
    
    pub async fn get_health_status_by_time_range(&mut self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<crate::models::health::HealthSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{
            "timestamp": {
                "$gte": start_time,
                "$lte": end_time
            }
        };
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"timestamp": -1}).build();
        let mut cursor = self.health_status.find(filter, options).await?;
        let mut health_status = Vec::new();
        while let Some(doc) = cursor.next().await {
            let status: HealthStatus = bson::from_document(doc?)?;
            let summary = crate::models::health::HealthSummary {
                truck_id: status.truck_id,
                last_timestamp: status.timestamp,
                status: status.status,
                cpu_percent: status.resources.cpu_percent,
                memory_percent: status.resources.memory_percent,
                disk_percent: status.resources.disk_percent,
                temperature_c: status.resources.temperature_c,
                uptime_sec: status.resources.uptime_sec,
                active_alerts: status.alerts.len() as i32,
                health_score: 100.0,
            };
            health_status.push(summary);
        }
        Ok(health_status)
    }
    
    pub async fn get_health_stats(&mut self) -> Result<crate::models::health::HealthStats, Box<dyn std::error::Error>> {
        let total_health_events = self.health_status.count_documents(doc!{}, None).await?;
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$status",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.health_status.aggregate(pipeline, None).await?;
        let mut by_status = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let status = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_status.insert(status, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$truck_id",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.health_status.aggregate(pipeline, None).await?;
        let mut by_truck = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let truck_id = result.get("truck_id").unwrap().as_str().unwrap();
            let truck_uuid = Uuid::parse_str(truck_id).unwrap_or(Uuid::nil());
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_truck.insert(truck_uuid, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": null,
                "avg_cpu_percent": {"$avg": "$resources.cpu_percent"},
                "avg_memory_percent": {"$avg": "$resources.memory_percent"},
                "avg_disk_percent": {"$avg": "$resources.disk_percent"},
                "avg_temperature_c": {"$avg": "$resources.temperature_c"}
            }}
        ];
        let mut cursor = self.health_status.aggregate(pipeline, None).await?;
        let mut avg_cpu_percent = 0.0;
        let mut avg_memory_percent = 0.0;
        let mut avg_disk_percent = 0.0;
        let mut avg_temperature_c = 0.0;
        if let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            avg_cpu_percent = result.get_f64("avg_cpu_percent").unwrap_or(0.0) as f32;
            avg_memory_percent = result.get_f64("avg_memory_percent").unwrap_or(0.0) as f32;
            avg_disk_percent = result.get_f64("avg_disk_percent").unwrap_or(0.0) as f32;
            avg_temperature_c = result.get_f64("avg_temperature_c").unwrap_or(0.0) as f32;
        }
        
        let now = Utc::now();
        let last_24_hours = self.health_status.count_documents(doc!{"timestamp": {"$gte": now - chrono::Duration::hours(24)}}, None).await?;
        let last_7_days = self.health_status.count_documents(doc!{"timestamp": {"$gte": now - chrono::Duration::days(7)}}, None).await?;
        let last_30_days = self.health_status.count_documents(doc!{"timestamp": {"$gte": now - chrono::Duration::days(30)}}, None).await?;
        
        Ok(crate::models::health::HealthStats {
            total_health_events,
            by_status,
            by_truck,
            avg_cpu_percent,
            avg_memory_percent,
            avg_disk_percent,
            avg_temperature_c,
            last_24_hours,
            last_7_days,
            last_30_days,
        })
    }
    
    pub async fn get_health_status_by_type_grouped(&mut self) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$status",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.health_status.aggregate(pipeline, None).await?;
        let mut status_by_type = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let status = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            status_by_type.insert(status, count);
        }
        Ok(status_by_type)
    }
    
    pub async fn get_top_health_trucks(&mut self, limit: i64) -> Result<Vec<(Uuid, i64)>, Box<dyn std::error::Error>> {
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$truck_id",
                "count": {"$sum": 1}
            }},
            doc!{"$sort": {"count": -1}},
            doc!{"$limit": limit}
        ];
        let mut cursor = self.health_status.aggregate(pipeline, None).await?;
        let mut top_trucks = Vec::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let truck_id = result.get("truck_id").unwrap().as_str().unwrap();
            let truck_uuid = Uuid::parse_str(truck_id).unwrap_or(Uuid::nil());
            let count = result.get_i64("count").unwrap_or(0) as i64;
            top_trucks.push((truck_uuid, count));
        }
        Ok(top_trucks)
    }
    
    pub async fn get_average_resource_usage(&mut self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>) -> Result<crate::services::health_service::AverageResources, Box<dyn std::error::Error>> {
        let filter = doc!{
            "timestamp": {
                "$gte": start_time,
                "$lte": end_time
            }
        };
        
        let pipeline = vec![
            doc!{"$match": filter},
            doc!{"$group": {
                "_id": null,
                "avg_cpu_percent": {"$avg": "$resources.cpu_percent"},
                "avg_memory_percent": {"$avg": "$resources.memory_percent"},
                "avg_disk_percent": {"$avg": "$resources.disk_percent"},
                "avg_temperature_c": {"$avg": "$resources.temperature_c"}
            }}
        ];
        
        let mut cursor = self.health_status.aggregate(pipeline, None).await?;
        let mut avg_cpu_percent = 0.0;
        let mut avg_memory_percent = 0.0;
        let mut avg_disk_percent = 0.0;
        let mut avg_temperature_c = 0.0;
        
        if let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            avg_cpu_percent = result.get_f64("avg_cpu_percent").unwrap_or(0.0) as f32;
            avg_memory_percent = result.get_f64("avg_memory_percent").unwrap_or(0.0) as f32;
            avg_disk_percent = result.get_f64("avg_disk_percent").unwrap_or(0.0) as f32;
            avg_temperature_c = result.get_f64("avg_temperature_c").unwrap_or(0.0) as f32;
        }
        
        Ok(crate::services::health_service::AverageResources {
            avg_cpu_percent,
            avg_memory_percent,
            avg_disk_percent,
            avg_temperature_c,
        })
    }
    
    pub async fn store_ota_update(&mut self, ota_update: &OtaUpdate) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(ota_update)?;
        self.ota_updates.insert_one(doc, None).await?;
        Ok(())
    }
    
    pub async fn get_ota_update(&mut self, id: Uuid) -> Result<OtaUpdate, Box<dyn std::error::Error>> {
        let filter = doc!{"id": id};
        let doc = self.ota_updates.find_one(filter, None).await?.ok_or("OTA update not found")?;
        let ota_update: OtaUpdate = bson::from_document(doc)?;
        Ok(ota_update)
    }
    
    pub async fn update_ota_update(&mut self, ota_update: &OtaUpdate) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(ota_update)?;
        self.ota_updates.replace_one(doc!{"id": ota_update.id}, doc, None).await?;
        Ok(())
    }
    
    pub async fn list_ota_updates(&mut self, limit: i64, offset: i64) -> Result<Vec<crate::models::ota::OtaSummary>, Box<dyn std::error::Error>> {
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"created_at": -1}).build();
        let mut cursor = self.ota_updates.find(None, options).await?;
        let mut ota_updates = Vec::new();
        while let Some(doc) = cursor.next().await {
            let update: OtaUpdate = bson::from_document(doc?)?;
            let summary = crate::models::ota::OtaSummary {
                id: update.id,
                update_id: update.update_id,
                truck_id: update.truck_id,
                fleet_id: update.fleet_id,
                version: update.version,
                target: update.target,
                priority: update.priority,
                status: update.status,
                progress_percent: update.progress_percent,
                created_at: update.created_at,
                truck_license_plate: None,
                truck_model: None,
                truck_make: None,
            };
            ota_updates.push(summary);
        }
        Ok(ota_updates)
    }
    
    pub async fn get_ota_updates_by_target(&mut self, target: crate::models::ota::UpdateTarget, limit: i64, offset: i64) -> Result<Vec<crate::models::ota::OtaSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"target": format!("{:?}", target)};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"created_at": -1}).build();
        let mut cursor = self.ota_updates.find(filter, options).await?;
        let mut ota_updates = Vec::new();
        while let Some(doc) = cursor.next().await {
            let update: OtaUpdate = bson::from_document(doc?)?;
            let summary = crate::models::ota::OtaSummary {
                id: update.id,
                update_id: update.update_id,
                truck_id: update.truck_id,
                fleet_id: update.fleet_id,
                version: update.version,
                target: update.target,
                priority: update.priority,
                status: update.status,
                progress_percent: update.progress_percent,
                created_at: update.created_at,
                truck_license_plate: None,
                truck_model: None,
                truck_make: None,
            };
            ota_updates.push(summary);
        }
        Ok(ota_updates)
    }
    
    pub async fn get_ota_updates_by_priority(&mut self, priority: crate::models::ota::UpdatePriority, limit: i64, offset: i64) -> Result<Vec<crate::models::ota::OtaSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"priority": format!("{:?}", priority)};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"created_at": -1}).build();
        let mut cursor = self.ota_updates.find(filter, options).await?;
        let mut ota_updates = Vec::new();
        while let Some(doc) = cursor.next().await {
            let update: OtaUpdate = bson::from_document(doc?)?;
            let summary = crate::models::ota::OtaSummary {
                id: update.id,
                update_id: update.update_id,
                truck_id: update.truck_id,
                fleet_id: update.fleet_id,
                version: update.version,
                target: update.target,
                priority: update.priority,
                status: update.status,
                progress_percent: update.progress_percent,
                created_at: update.created_at,
                truck_license_plate: None,
                truck_model: None,
                truck_make: None,
            };
            ota_updates.push(summary);
        }
        Ok(ota_updates)
    }
    
    pub async fn get_ota_updates_by_status(&mut self, status: crate::models::ota::OtaStatus, limit: i64, offset: i64) -> Result<Vec<crate::models::ota::OtaSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"status": format!("{:?}", status)};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"created_at": -1}).build();
        let mut cursor = self.ota_updates.find(filter, options).await?;
        let mut ota_updates = Vec::new();
        while let Some(doc) = cursor.next().await {
            let update: OtaUpdate = bson::from_document(doc?)?;
            let summary = crate::models::ota::OtaSummary {
                id: update.id,
                update_id: update.update_id,
                truck_id: update.truck_id,
                fleet_id: update.fleet_id,
                version: update.version,
                target: update.target,
                priority: update.priority,
                status: update.status,
                progress_percent: update.progress_percent,
                created_at: update.created_at,
                truck_license_plate: None,
                truck_model: None,
                truck_make: None,
            };
            ota_updates.push(summary);
        }
        Ok(ota_updates)
    }
    
    pub async fn get_ota_updates_by_truck(&mut self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<crate::models::ota::OtaSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"created_at": -1}).build();
        let mut cursor = self.ota_updates.find(filter, options).await?;
        let mut ota_updates = Vec::new();
        while let Some(doc) = cursor.next().await {
            let update: OtaUpdate = bson::from_document(doc?)?;
            let summary = crate::models::ota::OtaSummary {
                id: update.id,
                update_id: update.update_id,
                truck_id: update.truck_id,
                fleet_id: update.fleet_id,
                version: update.version,
                target: update.target,
                priority: update.priority,
                status: update.status,
                progress_percent: update.progress_percent,
                created_at: update.created_at,
                truck_license_plate: None,
                truck_model: None,
                truck_make: None,
            };
            ota_updates.push(summary);
        }
        Ok(ota_updates)
    }
    
    pub async fn get_ota_updates_by_time_range(&mut self, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>, limit: i64, offset: i64) -> Result<Vec<crate::models::ota::OtaSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{
            "created_at": {
                "$gte": start_time,
                "$lte": end_time
            }
        };
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"created_at": -1}).build();
        let mut cursor = self.ota_updates.find(filter, options).await?;
        let mut ota_updates = Vec::new();
        while let Some(doc) = cursor.next().await {
            let update: OtaUpdate = bson::from_document(doc?)?;
            let summary = crate::models::ota::OtaSummary {
                id: update.id,
                update_id: update.update_id,
                truck_id: update.truck_id,
                fleet_id: update.fleet_id,
                version: update.version,
                target: update.target,
                priority: update.priority,
                status: update.status,
                progress_percent: update.progress_percent,
                created_at: update.created_at,
                truck_license_plate: None,
                truck_model: None,
                truck_make: None,
            };
            ota_updates.push(summary);
        }
        Ok(ota_updates)
    }
    
    pub async fn get_ota_stats(&mut self) -> Result<crate::models::ota::OtaStats, Box<dyn std::error::Error>> {
        let total_updates = self.ota_updates.count_documents(doc!{}, None).await?;
        let pending_updates = self.ota_updates.count_documents(doc!{"status": "Pending"}, None).await?;
        let in_progress_updates = self.ota_updates.count_documents(doc!{"status": "Downloading"}, None).await? +
                                 self.ota_updates.count_documents(doc!{"status": "Verifying"}, None).await? +
                                 self.ota_updates.count_documents(doc!{"status": "Applying"}, None).await?;
        let successful_updates = self.ota_updates.count_documents(doc!{"status": "Success"}, None).await?;
        let failed_updates = self.ota_updates.count_documents(doc!{"status": "Failed"}, None).await?;
        let rollback_updates = self.ota_updates.count_documents(doc!{"status": "Rollback"}, None).await?;
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$target",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.ota_updates.aggregate(pipeline, None).await?;
        let mut by_target = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let target = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_target.insert(target, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$priority",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.ota_updates.aggregate(pipeline, None).await?;
        let mut by_priority = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let priority = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_priority.insert(priority, count);
        }
        
        let pipeline = vec![
            doc!{"$group": {
                "_id": "$status",
                "count": {"$sum": 1}
            }}
        ];
        let mut cursor = self.ota_updates.aggregate(pipeline, None).await?;
        let mut by_status = HashMap::new();
        while let Some(doc) = cursor.next().await {
            let result: Document = doc?;
            let status = result.get_str("_id").unwrap_or("Unknown").to_string();
            let count = result.get_i64("count").unwrap_or(0) as i64;
            by_status.insert(status, count);
        }
        
        let now = Utc::now();
        let last_24_hours = self.ota_updates.count_documents(doc!{"created_at": {"$gte": now - chrono::Duration::hours(24)}}, None).await?;
        let last_7_days = self.ota_updates.count_documents(doc!{"created_at": {"$gte": now - chrono::Duration::days(7)}}, None).await?;
        let last_30_days = self.ota_updates.count_documents(doc!{"created_at": {"$gte": now - chrono::Duration::days(30)}}, None).await?;
        
        Ok(crate::models::ota::OtaStats {
            total_updates,
            pending_updates,
            in_progress_updates,
            successful_updates,
            failed_updates,
            rollback_updates,
            by_target,
            by_priority,
            by_status,
            last_24_hours,
            last_7_days,
            last_30_days,
        })
    }
    
    pub async fn store_remote_command(&mut self, command: &RemoteCommand) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(command)?;
        self.remote_commands.insert_one(doc, None).await?;
        Ok(())
    }
    
    pub async fn get_remote_command(&mut self, id: Uuid) -> Result<RemoteCommand, Box<dyn std::error::Error>> {
        let filter = doc!{"id": id};
        let doc = self.remote_commands.find_one(filter, None).await?.ok_or("Remote command not found")?;
        let command: RemoteCommand = bson::from_document(doc)?;
        Ok(command)
    }
    
    pub async fn update_remote_command(&mut self, command: &RemoteCommand) -> Result<(), Box<dyn std::error::Error>> {
        let doc = bson::to_document(command)?;
        self.remote_commands.replace_one(doc!{"id": command.id}, doc, None).await?;
        Ok(())
    }
    
    pub async fn list_remote_commands(&mut self, limit: i64, offset: i64) -> Result<Vec<RemoteCommand>, Box<dyn std::error::Error>> {
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"issued_at": -1}).build();
        let mut cursor = self.remote_commands.find(None, options).await?;
        let mut commands = Vec::new();
        while let Some(doc) = cursor.next().await {
            let command: RemoteCommand = bson::from_document(doc?)?;
            commands.push(command);
        }
        Ok(commands)
    }
    
    pub async fn get_recent_trips(&mut self, truck_id: Uuid, limit: i64) -> Result<Vec<TripSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().limit(limit).sort(doc!{"start_time": -1}).build();
        let mut cursor = self.trips.find(filter, options).await?;
        let mut trips = Vec::new();
        while let Some(doc) = cursor.next().await {
            let trip: TripSummary = bson::from_document(doc?)?;
            trips.push(trip);
        }
        Ok(trips)
    }
    
    pub async fn get_trips(&mut self, truck_id: Uuid, limit: i64, offset: i64) -> Result<Vec<TripSummary>, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().limit(limit).skip(offset).sort(doc!{"start_time": -1}).build();
        let mut cursor = self.trips.find(filter, options).await?;
        let mut trips = Vec::new();
        while let Some(doc) = cursor.next().await {
            let trip: TripSummary = bson::from_document(doc?)?;
            trips.push(trip);
        }
        Ok(trips)
    }
    
    pub async fn get_maintenance_history(&mut self, truck_id: Uuid, limit: i64) -> Result<Vec<MaintenanceRecord>, Box<dyn std::error::Error>> {
        let filter = doc!{"truck_id": truck_id};
        let options = FindOptions::builder().limit(limit).sort(doc!{"performed_at": -1}).build();
        let mut cursor = self.maintenance_records.find(filter, options).await?;
        let mut records = Vec::new();
        while let Some(doc) = cursor.next().await {
            let record: MaintenanceRecord = bson::from_document(doc?)?;
            records.push(record);
        }
        Ok(records)
    }
}