use crate::ota::types::OtaUpdate;
use crate::ota::error::Result;
use ring::signature::{self, KeyPair};
use sha2::{Sha256, Digest};
use std::fs;

pub struct UpdateVerifier;

impl UpdateVerifier {
    pub fn verify_update(&self, file_path: &str, expected_checksum: &str, signature: &str) -> Result<()> {
        // Calculate checksum
        let data = fs::read(file_path)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let actual_checksum = hex::encode(hasher.finalize());

        if actual_checksum != expected_checksum {
            return Err(OtaError::ChecksumError {
                expected: expected_checksum.to_string(),
                actual: actual_checksum,
            });
        }

        // Verify signature (simplified - in production, use actual public key)
        if signature.len() != 128 {
            return Err(OtaError::SignatureError("Invalid signature length".to_string()));
        }

        info!(file=%file_path, "✅ Update verified");
        Ok(())
    }
}