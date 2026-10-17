//! Serializable settings for S3-compatible archival.

use serde::{Deserialize, Serialize};

/// Serializable bucket, endpoint, credentials, region, and stub-mode settings.
///
/// # Examples
/// ```
/// use codetether_event_stream::s3_sink::S3SinkConfig;
/// let config: S3SinkConfig = serde_json::from_str(r#"{"bucket":"logs","prefix":"events/"}"#).unwrap();
/// assert_eq!(config.region, "us-east-1");
/// assert!(!config.stub_mode);
/// ```
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
