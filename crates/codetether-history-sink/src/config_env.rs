//! Parse the existing optional environment configuration.
use crate::HistorySinkConfig;
use crate::config::default_prefix;
use anyhow::Result;
impl HistorySinkConfig {
    /// Load the config from environment variables.
    ///
    /// Returns `Ok(None)` when any required variable is unset — callers
    /// treat that as *sink disabled* and skip the upload. Returns
    /// `Err(_)` only when an env variable is present but malformed.
    ///
    /// # Errors
    ///
    /// Never in the current implementation; reserved for future URL
    /// / region validation.
    pub fn from_env() -> Result<Option<Self>> {
        let endpoint = match std::env::var("CODETETHER_HISTORY_S3_ENDPOINT") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => return Ok(None),
        };
        let bucket = match std::env::var("CODETETHER_HISTORY_S3_BUCKET") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => return Ok(None),
        };
        let access_key = match std::env::var("CODETETHER_HISTORY_S3_ACCESS_KEY") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => return Ok(None),
        };
        let secret_key = match std::env::var("CODETETHER_HISTORY_S3_SECRET_KEY") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => return Ok(None),
        };
        let prefix = std::env::var("CODETETHER_HISTORY_S3_PREFIX")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| if s.ends_with('/') { s } else { format!("{s}/") })
            .unwrap_or_else(default_prefix);

        Ok(Some(Self {
            endpoint,
            access_key,
            secret_key,
            bucket,
            prefix,
        }))
    }
}
