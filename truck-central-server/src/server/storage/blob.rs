use rusoto_s3::{S3Client, S3, PutObjectRequest, GetObjectRequest};
use rusoto_core::Region;
use uuid::Uuid;

pub struct BlobStore {
    client: S3Client,
    bucket: String,
}

impl BlobStore {
    pub fn new(client: S3Client, bucket: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client,
            bucket: bucket.to_string(),
        })
    }
    
    pub async fn store_camera_frame(&mut self, truck_id: Uuid, frame_data: &[u8], format: &str) -> Result<String, Box<dyn std::error::Error>> {
        let key = format!("trucks/{}/frames/{}/{}.{}", 
            truck_id,
            chrono::Utc::now().format("%Y/%m/%d"),
            Uuid::new_v4(),
            format
        );
        
        let request = PutObjectRequest {
            bucket: self.bucket.clone(),
            key: key.clone(),
            body: Some(frame_data.to_vec().into()),
            content_type: Some(match format {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "h264" => "video/h264",
                _ => "application/octet-stream",
            }.to_string()),
            ..Default::default()
        };
        
        self.client.put_object(request).await?;
        Ok(key)
    }
    
    pub async fn get_camera_frame(&mut self, key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let request = GetObjectRequest {
            bucket: self.bucket.clone(),
            key: key.to_string(),
            ..Default::default()
        };
        
        let result = self.client.get_object(request).await?;
        let data = result.body.unwrap().collect().await?;
        Ok(data)
    }
    
    pub async fn generate_presigned_url(&mut self, key: &str, expires_in: i64) -> Result<String, Box<dyn std::error::Error>> {
        // In production, use S3 presigned URL generation
        // For now, return a dummy URL
        Ok(format!("https://{}/{}?expires={}", self.bucket, key, expires_in))
    }
}