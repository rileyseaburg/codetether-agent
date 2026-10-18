//! Owned environment-driven history upload configuration.
use serde::{Deserialize, Serialize};

/// Default object-key prefix when
/// [`HistorySinkConfig::CODETETHER_HISTORY_S3_PREFIX`] is unset.
///
/// [`HistorySinkConfig::CODETETHER_HISTORY_S3_PREFIX`]: HistorySinkConfig
const DEFAULT_PREFIX: &str = "history/";

/// Durable history-sink configuration.
///
/// Intentionally owned (not borrowed) so the config can be handed to a
/// `tokio::spawn` fire-and-forget upload without tying its lifetime to
/// the `Session`.
///
/// # Examples
///
/// ```rust
/// use codetether_history_sink::HistorySinkConfig;
///
/// let cfg = HistorySinkConfig {
///     endpoint: "http://localhost:9000".to_string(),
///     access_key: "minioadmin".to_string(),
///     secret_key: "minioadmin".to_string(),
///     bucket: "codetether".to_string(),
///     prefix: "history/".to_string(),
/// };
/// assert_eq!(cfg.bucket, "codetether");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistorySinkConfig {
    /// MinIO / S3 endpoint URL, e.g. `http://localhost:9000`.
    pub endpoint: String,
    /// S3 access key.
    pub access_key: String,
    /// S3 secret key.
    pub secret_key: String,
    /// Destination bucket.
    pub bucket: String,
    /// Key prefix under the bucket. Always ends with `/`.
    #[serde(default = "default_prefix")]
    pub prefix: String,
}

pub(crate) fn default_prefix() -> String {
    DEFAULT_PREFIX.to_string()
}
