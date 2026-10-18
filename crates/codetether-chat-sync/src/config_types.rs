//! Chat sync configuration: types and constants.

pub const CHAT_SYNC_DEFAULT_BUCKET: &str = "codetether-chat-archive";
pub const CHAT_SYNC_DEFAULT_PREFIX: &str = "chat-events";
pub const CHAT_SYNC_DEFAULT_INTERVAL_SECS: u64 = 30;
pub const CHAT_SYNC_MAX_INTERVAL_SECS: u64 = 600;

#[derive(Debug, Clone)]
/// Connection and scheduling settings for archive synchronization.
/// # Examples
/// ```
/// let config = codetether_chat_sync::config_types::ChatSyncConfig { endpoint: "http://localhost:9000".into(), fallback_endpoint: None, access_key: "example".into(), secret_key: "example".into(), bucket: "archive".into(), prefix: "events".into(), interval_secs: 30, ignore_cert_check: false };
/// assert_eq!(config.interval_secs, 30);
/// ```
pub struct ChatSyncConfig {
    /// Primary S3 endpoint.
    pub endpoint: String,
    /// Optional alternate endpoint.
    pub fallback_endpoint: Option<String>,
    /// S3 access-key identifier.
    pub access_key: String,
    /// S3 secret key.
    pub secret_key: String,
    /// Destination bucket.
    pub bucket: String,
    /// Object-key prefix.
    pub prefix: String,
    /// Seconds between upload attempts.
    pub interval_secs: u64,
    /// Whether the client skips certificate verification.
    pub ignore_cert_check: bool,
}

/// Normalize an endpoint URL and remove console login suffixes.
/// # Arguments
/// Configured endpoint.
/// # Returns
/// Normalized URL with an explicit scheme.
/// # Examples
/// ```text
/// Configure the archive destination, then use normalize_minio_endpoint through the chat-sync API.
/// ```
pub fn normalize_minio_endpoint(endpoint: &str) -> String {
    let mut n = endpoint.trim().trim_end_matches('/').to_string();
    if let Some(s) = n.strip_suffix("/login") {
        n = s.trim_end_matches('/').to_string();
    }
    if !n.starts_with("http://") && !n.starts_with("https://") {
        n = format!("http://{n}");
    }
    n
}

/// Derive an API endpoint from the conventional console port.
/// # Arguments
/// Primary endpoint.
/// # Returns
/// An alternate port-9000 endpoint when the source contains port 9001.
/// # Examples
/// ```text
/// Configure the archive destination, then use minio_fallback_endpoint through the chat-sync API.
/// ```
pub fn minio_fallback_endpoint(endpoint: &str) -> Option<String> {
    if endpoint.contains(":9001") {
        Some(endpoint.replacen(":9001", ":9000", 1))
    } else {
        None
    }
}
