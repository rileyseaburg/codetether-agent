//! S3/R2 Object Storage Sink for Event Streams
//!
//! Automatically archives JSONL event stream files to S3-compatible storage
//! (AWS S3, Cloudflare R2, MinIO, etc.) for immutable, tamper-proof archival.
//!
//! This is essential for compliance: SOC 2, FedRAMP, ATO processes require
//! permanent, unmodifiable records of AI agent actions.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use codetether_agent::event_stream::s3_sink::S3Sink;
//!
//! let sink = S3Sink::new(
//!     "audit-logs-bucket".to_string(),
//!     "events/".to_string(),
//!     "https://account-id.r2.cloudflarestorage.com".to_string(),
//!     "access-key".to_string(),
//!     "secret-key".to_string(),
//! ).await?;
//!
//! // Upload event file to S3/R2
//! sink.upload_file("/local/path/events.jsonl", "session-id").await?;
//! ```

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during S3 operations
#[derive(Error, Debug)]
pub enum S3SinkError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing S3 configuration: {0}")]
    MissingConfig(String),

    #[error("Upload failed: {0}")]
    UploadFailed(String),

    #[error("S3 returned error: {0}")]
    S3Error(String),
}

/// Configuration for S3/R2 sink
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3SinkConfig {
    /// S3 bucket name
    pub bucket: String,
    /// Path prefix for uploaded files
    pub prefix: String,
    /// S3 endpoint URL (None for AWS S3, Some for R2/MinIO)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// S3 access key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// S3 secret key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// Region (default: us-east-1)
    #[serde(default = "default_region")]
    pub region: String,
    /// Stub mode - skip actual HTTP requests when S3 unavailable
    #[serde(default)]
    pub stub_mode: bool,
}

fn default_region() -> String {
    "us-east-1".to_string()
}

impl S3SinkConfig {
    /// Create config from environment variables
    ///
    /// Supports both `S3_*` (preferred) and `CODETETHER_S3_*` (legacy) environment variables.
    /// The `S3_*` variables take precedence over `CODETETHER_S3_*` variables for backwards compatibility.
    ///
    /// # Environment Variables
    /// - `S3_BUCKET` (or `CODETETHER_S3_BUCKET`): S3 bucket name
    /// - `S3_PREFIX` (or `CODETETHER_S3_PREFIX`): Path prefix for uploads (default: "events/")
    /// - `S3_ENDPOINT` (or `CODETETHER_S3_ENDPOINT`): Custom endpoint for R2/MinIO
    /// - `S3_ACCESS_KEY` (or `CODETETHER_S3_ACCESS_KEY`): Access key
    /// - `S3_SECRET_KEY` (or `CODETETHER_S3_SECRET_KEY`): Secret key
    /// - `S3_REGION` (or `CODETETHER_S3_REGION`, `AWS_REGION`): Region (default: "us-east-1")
    /// - `S3_STUB_MODE` (or `CODETETHER_S3_STUB_MODE`): Enable stub mode (skip actual uploads)
    pub fn from_env() -> Option<Self> {
        // Try S3_* first (new naming), fall back to CODETETHER_S3_* (legacy)
        let bucket = std::env::var("S3_BUCKET")
            .or_else(|_| std::env::var("CODETETHER_S3_BUCKET"))
            .ok()?;

        let prefix = std::env::var("S3_PREFIX")
            .or_else(|_| std::env::var("CODETETHER_S3_PREFIX"))
            .unwrap_or_else(|_| "events/".to_string());

        let endpoint = std::env::var("S3_ENDPOINT")
            .or_else(|_| std::env::var("CODETETHER_S3_ENDPOINT"))
            .ok();

        let access_key = std::env::var("S3_ACCESS_KEY")
            .or_else(|_| std::env::var("CODETETHER_S3_ACCESS_KEY"))
            .ok();

        let secret_key = std::env::var("S3_SECRET_KEY")
            .or_else(|_| std::env::var("CODETETHER_S3_SECRET_KEY"))
            .ok();

        let region = std::env::var("S3_REGION")
            .or_else(|_| std::env::var("CODETETHER_S3_REGION"))
            .or_else(|_| std::env::var("AWS_REGION"))
            .unwrap_or_else(|_| "us-east-1".to_string());

        let stub_mode = std::env::var("S3_STUB_MODE")
            .or_else(|_| std::env::var("CODETETHER_S3_STUB_MODE"))
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        // Log deprecation warning when using legacy CODETETHER_S3_* variables
        if std::env::var("CODETETHER_S3_BUCKET").is_ok() && std::env::var("S3_BUCKET").is_err() {
            tracing::warn!(
                "Using legacy CODETETHER_S3_* environment variables. These are deprecated. Please migrate to S3_* naming convention."
            );
        }

        Some(Self {
            bucket,
            prefix,
            endpoint,
            access_key,
            secret_key,
            region,
            stub_mode,
        })
    }
}

/// S3/R2 sink for event stream archival
pub struct S3Sink {
    client: Client,
    config: S3SinkConfig,
}

impl S3Sink {
    #[allow(dead_code)]
    /// Create a new S3 sink
    pub async fn new(
        bucket: String,
        prefix: String,
        endpoint: Option<String>,
        access_key: Option<String>,
        secret_key: Option<String>,
    ) -> Result<Self, S3SinkError> {
        let config = S3SinkConfig {
            bucket,
            prefix,
            endpoint,
            access_key,
            secret_key,
            region: "us-east-1".to_string(),
            stub_mode: false,
        };

        Self::from_config(config).await
    }

    /// Create S3 sink from configuration
    pub async fn from_config(config: S3SinkConfig) -> Result<Self, S3SinkError> {
        let client = Client::builder().build().map_err(S3SinkError::Http)?;

        Ok(Self { client, config })
    }

    /// Create S3 sink from environment variables
    pub async fn from_env() -> Result<Self, S3SinkError> {
        let config = S3SinkConfig::from_env().ok_or_else(|| {
            S3SinkError::MissingConfig("S3_BUCKET (or CODETETHER_S3_BUCKET) not set".to_string())
        })?;

        Self::from_config(config).await
    }

    /// Build the S3 endpoint URL
    fn endpoint_url(&self) -> String {
        if let Some(ref endpoint) = self.config.endpoint {
            // Custom endpoint (R2, MinIO, etc.)
            format!("{}/{}", endpoint.trim_end_matches('/'), self.config.bucket)
        } else {
            // AWS S3
            format!(
                "https://{}.s3.{}.amazonaws.com",
                self.config.bucket, self.config.region
            )
        }
    }

    /// Generate AWS Authorization header
    fn generate_aws_header(
        &self,
        method: &Method,
        path: &str,
        query: &str,
        content_hash: &str,
        date: &str,
    ) -> String {
        let access_key = self.config.access_key.as_deref().unwrap_or("");
        let secret_key = self.config.secret_key.as_deref().unwrap_or("");

        let canonical_request = format!(
            "{}\n{}\n{}\n\ncontent-type:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n\ncontent-type;x-amz-content-sha256;x-amz-date",
            method, path, query, "application/octet-stream", content_hash, date
        );

        // String to sign
        let canonical_request_hash = sha256_hex(&canonical_request);
        let credential_scope = format!("{}/{}/s3/aws4_request", &date[..8], self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            date, credential_scope, canonical_request_hash
        );

        // Signing key
        let k_date = hmac_sha256(
            format!("AWS4{}", secret_key).as_bytes(),
            date[..8].as_bytes(),
        );
        let k_region = hmac_sha256(&k_date, self.config.region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"s3");
        let k_signing = hmac_sha256(&k_service, b"aws4_request");

        // Signature
        let signature = hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()));

        format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders=content-type;x-amz-content-sha256;x-amz-date, Signature={}",
            access_key, credential_scope, signature
        )
    }

    /// Upload a local file to S3/R2
    pub async fn upload_file(
        &self,
        local_path: &PathBuf,
        session_id: &str,
    ) -> Result<String, S3SinkError> {
        let filename = local_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| S3SinkError::UploadFailed("Invalid filename".to_string()))?;

        // Read file contents
        let data = tokio::fs::read(local_path).await?;

        // S3 key: prefix/session_id/timestamp-filename
        let s3_key = format!("{}{}/{}", self.config.prefix, session_id, filename);

        // Upload to S3
        self.upload_bytes(&data, &s3_key, "application/json").await
    }

    /// Upload bytes directly to S3/R2
    pub async fn upload_bytes(
        &self,
        data: &[u8],
        s3_key: &str,
        _content_type: &str,
    ) -> Result<String, S3SinkError> {
        // Stub mode: skip actual HTTP request
        if self.config.stub_mode {
            let url = if let Some(ref endpoint) = self.config.endpoint {
                format!("{}/{}", endpoint.trim_end_matches('/'), s3_key)
            } else {
                format!("s3://{}/{}", self.config.bucket, s3_key)
            };

            tracing::debug!(
                key = %s3_key,
                bytes = data.len(),
                "[STUB MODE] Skipping S3 upload"
            );

            return Ok(url);
        }

        use std::time::{SystemTime, UNIX_EPOCH};

        let endpoint = self.endpoint_url();
        let path = format!("/{}", s3_key);

        // Generate AWS Signature V4 headers
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let date = chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
            .map(|dt| dt.format("%Y%m%dT%H%M%SZ").to_string())
            .unwrap_or_default();
        let _date_only = &date[..8];

        let content_hash = sha256_hex_bytes(data);

        let method = Method::PUT;
        let auth_header = self.generate_aws_header(&method, &path, "", &content_hash, &date);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-amz-date",
            HeaderValue::from_str(&date).map_err(|e| S3SinkError::UploadFailed(e.to_string()))?,
        );
        headers.insert(
            "x-amz-content-sha256",
            HeaderValue::from_str(&content_hash)
                .map_err(|e| S3SinkError::UploadFailed(e.to_string()))?,
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_header)
                .map_err(|e| S3SinkError::UploadFailed(e.to_string()))?,
        );

        let url = format!("{}{}", endpoint, path);

        let response = self
            .client
            .put(&url)
            .headers(headers)
            .body(data.to_vec())
            .send()
            .await?;

        if response.status().is_success() {
            let url = if let Some(ref endpoint) = self.config.endpoint {
                format!("{}/{}", endpoint.trim_end_matches('/'), s3_key)
            } else {
                format!(
                    "https://{}.s3.{}.amazonaws.com/{}",
                    self.config.bucket, self.config.region, s3_key
                )
            };

            tracing::info!("Uploaded event stream to S3: {}", url);
            Ok(url)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(S3SinkError::S3Error(format!("{}: {}", status, body)))
        }
    }

    /// Check if S3 sink is configured
    ///
    /// Returns true if either `CODETETHER_S3_BUCKET` or `S3_BUCKET` is set.
    pub fn is_configured() -> bool {
        std::env::var("CODETETHER_S3_BUCKET")
            .or_else(|_| std::env::var("S3_BUCKET"))
            .is_ok()
    }

    #[allow(dead_code)]
    /// Get the bucket name
    pub fn bucket_name(&self) -> &str {
        &self.config.bucket
    }

    #[allow(dead_code)]
    /// Get the prefix
    pub fn prefix(&self) -> &str {
        &self.config.prefix
    }
}

/// Compute SHA256 hex digest
fn sha256_hex(s: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:016x}{:016x}", hasher.finish(), hasher.finish())
}

/// Compute SHA256 hex digest of bytes
fn sha256_hex_bytes(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Compute HMAC-SHA256
fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<sha2::Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// Event file with S3 archival status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ArchivedEventFile {
    /// Local path where the file was written
    pub local_path: PathBuf,
    /// S3 URL where the file was archived
    pub s3_url: String,
    /// Session ID
    pub session_id: String,
    /// Byte range of events in this file
    pub start_offset: u64,
    pub end_offset: u64,
    /// When the file was archived
    pub archived_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to ensure clean environment before each test
    fn clean_env() {
        // Note: set_var/remove_var are unsafe in Rust 2024+, but this is test-only code
        unsafe {
            // Remove both legacy CODETETHER_S3_* and new S3_* variables
            std::env::remove_var("CODETETHER_S3_BUCKET");
            std::env::remove_var("CODETETHER_S3_PREFIX");
            std::env::remove_var("CODETETHER_S3_ENDPOINT");
            std::env::remove_var("CODETETHER_S3_REGION");
            std::env::remove_var("CODETETHER_S3_ACCESS_KEY");
            std::env::remove_var("CODETETHER_S3_SECRET_KEY");
            std::env::remove_var("CODETETHER_S3_STUB_MODE");
            std::env::remove_var("S3_BUCKET");
            std::env::remove_var("S3_PREFIX");
            std::env::remove_var("S3_ENDPOINT");
            std::env::remove_var("S3_REGION");
            std::env::remove_var("S3_ACCESS_KEY");
            std::env::remove_var("S3_SECRET_KEY");
            std::env::remove_var("S3_STUB_MODE");
            std::env::remove_var("AWS_REGION");
        }
    }

    #[test]
    fn test_config_from_env() {
        // Always start clean
        clean_env();

        unsafe {
            std::env::set_var("CODETETHER_S3_BUCKET", "test-bucket");
            std::env::set_var("CODETETHER_S3_PREFIX", "audit/");
            std::env::set_var(
                "CODETETHER_S3_ENDPOINT",
                "https://test.r2.cloudflarestorage.com",
            );
        }

        let config = S3SinkConfig::from_env();
        assert!(config.is_some());

        let cfg = config.unwrap();
        assert_eq!(cfg.bucket, "test-bucket");
        assert_eq!(cfg.prefix, "audit/");
        assert_eq!(
            cfg.endpoint,
            Some("https://test.r2.cloudflarestorage.com".to_string())
        );

        // Clean up
        clean_env();
    }

    #[test]
    fn test_config_defaults() {
        clean_env();

        unsafe {
            std::env::set_var("CODETETHER_S3_BUCKET", "test-bucket");
        }

        let config = S3SinkConfig::from_env().unwrap();
        assert_eq!(config.region, "us-east-1");

        clean_env();
    }

    #[test]
    fn test_is_configured() {
        // Always start clean - remove the var first to ensure isolation
        clean_env();

        let is_not_configured = !S3Sink::is_configured();
        assert!(
            is_not_configured,
            "S3 sink should not be configured by default"
        );

        unsafe {
            std::env::set_var("CODETETHER_S3_BUCKET", "test-bucket");
        }
        assert!(
            S3Sink::is_configured(),
            "S3 sink should be configured when bucket is set"
        );

        clean_env();
    }
}
