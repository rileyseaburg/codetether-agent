//! Provider configuration and construction.
use anyhow::Result;
use reqwest::Client;

/// Connection and credential state for the DeepSeek chat-completion adapter.
///
/// # Examples
/// ```no_run
/// use codetether_provider_deepseek::DeepSeekProvider;
/// let provider = DeepSeekProvider::with_base_url(
///     String::new(), "https://api.deepseek.com/v1".into(),
/// ).unwrap();
/// assert!(!provider.base_url.is_empty());
/// ```
pub struct DeepSeekProvider {
    /// Shared HTTP connection pool used by completion requests.
    pub client: Client,
    /// Bearer credential; do not include it in diagnostic output.
    pub api_key: String,
    /// Endpoint prefix to which `/chat/completions` is appended.
    pub base_url: String,
}

impl DeepSeekProvider {
    /// Create a new provider with a custom base URL.
    ///
    /// # Arguments
    /// * `api_key` - Credential used for subsequent requests.
    /// * `base_url` - Endpoint prefix for the completion API.
    ///
    /// # Returns
    /// A provider sharing the process-wide provider HTTP client.
    ///
    /// # Errors
    /// This constructor currently always returns `Ok`.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_provider_deepseek::DeepSeekProvider;
    /// let provider = DeepSeekProvider::with_base_url(String::new(), "https://example.test/v1".into()).unwrap();
    /// assert_eq!(provider.base_url, "https://example.test/v1");
    /// ```
    pub fn with_base_url(api_key: String, base_url: String) -> Result<Self> {
        Ok(Self {
            client: codetether_provider_core::shared_http::shared_client().clone(),
            api_key,
            base_url,
        })
    }
}
