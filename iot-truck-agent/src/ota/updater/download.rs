use crate::ota::types::OtaUpdate;
use crate::ota::error::Result;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{info, warn};

pub struct BandwidthAwareDownloader {
    client: Client,
    max_bandwidth_kbps: u32,
    temp_dir: String,
}

impl BandwidthAwareDownloader {
    pub fn new(max_bandwidth_kbps: u32, temp_dir: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .unwrap();

        Self {
            client,
            max_bandwidth_kbps,
            temp_dir: temp_dir.to_string(),
        }
    }

    pub async fn download_update(&self, update: &OtaUpdate) -> Result<String> {
        info!(update_id=%update.update_id, "📥 Starting download");

        let response = self.client
            .get(&update.url)
            .send()
            .await?;

        let total_size = response.content_length().unwrap_or(0);
        let mut file = tempfile::NamedTempFile::new_in(&self.temp_dir)?;
        let mut file_path = file.path().to_path_buf();

        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;
        let start_time = std::time::Instant::now();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            downloaded += chunk.len() as u64;

            // Bandwidth throttling
            if self.max_bandwidth_kbps > 0 {
                let elapsed_ms = start_time.elapsed().as_millis();
                let target_bytes = (self.max_bandwidth_kbps as u64 * elapsed_ms * 1000) / 8;
                if downloaded > target_bytes {
                    let delay_ms = ((downloaded * 8 / self.max_bandwidth_kbps as u64) - elapsed_ms as u64) as u64;
                    if delay_ms > 0 {
                        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    }
                }
            }

            file.write_all(&chunk).await?;
            let progress = (downloaded * 100) / total_size.max(1);
            if progress % 10 == 0 {
                info!(update_id=%update.update_id, progress=%progress, "⬇️  Download progress");
            }
        }

        file.flush().await?;
        let final_path = file.into_temp_path();
        info!(update_id=%update.update_id, path=%final_path.display(), "✅ Download completed");
        Ok(final_path.to_string_lossy().to_string())
    }
}