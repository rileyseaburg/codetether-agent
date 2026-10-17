//! Construct a bounded-time HTTP client from environment or explicit settings.
use super::*;
impl HttpStore {
    /// Create from environment variables.
    ///
    /// - `CODETETHER_API_URL` — base URL (default: `http://localhost:8080`)
    /// - `CODETETHER_API_KEY` — optional Bearer token
    ///
    /// # Returns
    /// A store using the configured endpoint and an optional Bearer token.
    /// # Examples
    /// ```no_run
    /// let store = codetether_ralph_core::store_http::HttpStore::from_env();
    /// let _ = store;
    /// ```
    pub fn from_env() -> Self {
        let base_url = std::env::var("CODETETHER_API_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());
        let api_key = std::env::var("CODETETHER_API_KEY")
            .ok()
            .filter(|s| !s.is_empty());

        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
        }
    }

    /// Create with explicit base URL and optional API key
    ///
    /// # Arguments
    /// * `base_url` - Server URL; trailing slashes are removed.
    /// * `api_key` - Optional Bearer token.
    /// # Returns
    /// A store using a ten-second client timeout, with the original default-client fallback.
    /// # Examples
    /// ```
    /// let store = codetether_ralph_core::store_http::HttpStore::new("http://localhost:8080", None);
    /// let _ = store;
    /// ```
    pub fn new(base_url: &str, api_key: Option<String>) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
        }
    }
}
