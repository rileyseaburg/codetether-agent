//! Chat sync configuration: types and constants.

pub const CHAT_SYNC_DEFAULT_BUCKET: &str = "codetether-chat-archive";
pub const CHAT_SYNC_DEFAULT_PREFIX: &str = "chat-events";
pub const CHAT_SYNC_DEFAULT_INTERVAL_SECS: u64 = 30;
pub const CHAT_SYNC_MAX_INTERVAL_SECS: u64 = 600;

#[derive(Debug, Clone)]
pub struct ChatSyncConfig {
    pub endpoint: String,
    pub fallback_endpoint: Option<String>,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub prefix: String,
    pub interval_secs: u64,
    pub ignore_cert_check: bool,
}

pub fn normalize_minio_endpoint(endpoint: &str) -> String {
    let mut n = endpoint.trim().trim_end_matches('/').to_string();
    if let Some(s) = n.strip_suffix("/login") { n = s.trim_end_matches('/').to_string(); }
    if !n.starts_with("http://") && !n.starts_with("https://") { n = format!("http://{n}"); }
    n
}

pub fn minio_fallback_endpoint(endpoint: &str) -> Option<String> {
    if endpoint.contains(":9001") { Some(endpoint.replacen(":9001", ":9000", 1)) } else { None }
}
