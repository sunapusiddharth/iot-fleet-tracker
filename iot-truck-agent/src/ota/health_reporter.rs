use crate::health::types::HealthEvent;
use crate::ota::error::Result;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::{info, warn};

pub struct HealthReporter {
    client: Client,
    upload_url: String,
}

impl HealthReporter {
    pub fn new(upload_url: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .unwrap();

        Self {
            client,
            upload_url: upload_url.to_string(),
        }
    }

    pub async fn upload_health_snapshot(&self, health_event: &HealthEvent) -> Result<String> {
        info!("📤 Uploading health snapshot");

        let json = serde_json::to_vec(health_event)?;
        let snapshot_id = format!("snapshot-{}", chrono::Utc::now().timestamp_nanos());

        let response = self.client
            .post(&self.upload_url)
            .header("Content-Type", "application/json")
            .header("X-Snapshot-ID", &snapshot_id)
            .body(json)
            .send()
            .await?;

        if response.status().is_success() {
            info!(snapshot_id=%snapshot_id, "✅ Health snapshot uploaded");
            Ok(snapshot_id)
        } else {
            let error = response.text().await?;
            Err(OtaError::DownloadError(reqwest::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Upload failed: {}", error),
            ))))
        }
    }

    pub async fn upload_system_snapshot(&self, snapshot_path: &str) -> Result<String> {
        info!(path=%snapshot_path, "📤 Uploading system snapshot");

        let mut file = File::open(snapshot_path).await?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).await?;

        let snapshot_id = format!("system-snapshot-{}", chrono::Utc::now().timestamp_nanos());

        let response = self.client
            .post(&self.upload_url)
            .header("Content-Type", "application/gzip")
            .header("X-Snapshot-ID", &snapshot_id)
            .body(data)
            .send()
            .await?;

        if response.status().is_success() {
            info!(snapshot_id=%snapshot_id, "✅ System snapshot uploaded");
            Ok(snapshot_id)
        } else {
            let error = response.text().await?;
            Err(OtaError::DownloadError(reqwest::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Upload failed: {}", error),
            ))))
        }
    }
}