//! Environment-derived heartbeat sharing settings.

use crate::cognition_env::{env_bool, env_u64, env_usize};

/// Runtime configuration for sending cognition snapshots with worker heartbeats.
/// Heartbeat polling and optional thought-summary sharing settings.
///
/// # Examples
/// ```
/// use codetether_a2a_worker_core::{CognitionHeartbeatConfig};
/// let settings = CognitionHeartbeatConfig::from_env();
/// assert!(settings.interval_secs >= 5);
/// assert!(settings.request_timeout_ms >= 250);
/// ```
#[derive(Clone, Debug)]
pub struct CognitionHeartbeatConfig {
    pub enabled: bool,
    pub source_base_url: String,
    pub token: Option<String>,
    pub provider_name: String,
    pub interval_secs: u64,
    pub include_thought_summary: bool,
    pub summary_max_chars: usize,
    pub request_timeout_ms: u64,
}

impl CognitionHeartbeatConfig {
    /// Load cognition heartbeat configuration from environment variables.
    ///
    /// # Examples
    /// ```
    /// use codetether_a2a_worker_core::{CognitionHeartbeatConfig};
    /// let settings = CognitionHeartbeatConfig::from_env();
    /// assert!(settings.interval_secs >= 5);
    /// assert!(settings.request_timeout_ms >= 250);
    /// ```
    pub fn from_env() -> Self {
        Self {
            enabled: env_bool("CODETETHER_WORKER_COGNITION_SHARE_ENABLED", true),
            source_base_url: cognition_source_url(),
            token: std::env::var("CODETETHER_WORKER_COGNITION_TOKEN").ok(),
            provider_name: cognition_provider(),
            interval_secs: env_u64("CODETETHER_WORKER_COGNITION_INTERVAL_SECS", 30).max(5),
            include_thought_summary: env_bool("CODETETHER_WORKER_COGNITION_INCLUDE_THOUGHTS", true),
            summary_max_chars: env_usize("CODETETHER_WORKER_COGNITION_THOUGHT_MAX_CHARS", 480),
            request_timeout_ms: env_u64("CODETETHER_WORKER_COGNITION_TIMEOUT_MS", 2_500).max(250),
        }
    }
}

fn cognition_source_url() -> String {
    std::env::var("CODETETHER_WORKER_COGNITION_SOURCE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:4096".to_string())
        .trim_end_matches('/')
        .to_string()
}

fn cognition_provider() -> String {
    std::env::var("CODETETHER_WORKER_COGNITION_PROVIDER")
        .unwrap_or_else(|_| "cognition".to_string())
}
