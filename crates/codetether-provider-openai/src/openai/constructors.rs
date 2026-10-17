//! Client construction and TCP transport defaults.

use super::*;
impl OpenAIProvider {
    /// Create an OpenAI client with the supplied API key.
    ///
    /// # Errors
    /// Returns an error if HTTP client initialization fails.
    pub fn new(api_key: String) -> Result<Self> {
        tracing::debug!(
            provider = "openai",
            api_key_len = api_key.len(),
            "Creating OpenAI provider"
        );
        let config = OpenAIConfig::new().with_api_key(api_key.clone());
        let api_base = "https://api.openai.com/v1".to_string();
        Ok(Self {
            client: Client::with_config(config),
            provider_name: "openai".to_string(),
            api_key: Some(api_key),
            api_base,
            http: codetether_provider_core::socket_opts::apply_socket_opts(HttpClient::builder())
                .timeout(std::time::Duration::from_secs(45))
                .build()?,
        })
    }
    /// Create with custom base URL (for OpenAI-compatible providers like Moonshot).
    ///
    /// # Errors
    /// Returns an error if HTTP client initialization fails.
    pub fn with_base_url(api_key: String, base_url: String, provider_name: &str) -> Result<Self> {
        Self::with_base_url_optional_key(Some(api_key), base_url, provider_name)
    }
    /// Create with custom base URL and optional API key.
    ///
    /// # Errors
    /// Returns an error if HTTP client initialization fails.
    ///
    /// Useful for private in-cluster OpenAI-compatible endpoints that rely on
    /// network policy instead of bearer authentication.
    pub fn with_base_url_optional_key(
        api_key: Option<String>,
        base_url: String,
        provider_name: &str,
    ) -> Result<Self> {
        let api_key = api_key.filter(|key| !key.trim().is_empty());
        tracing::debug!(
            provider = provider_name,
            base_url = %base_url,
            api_key_len = api_key.as_ref().map(|key| key.len()).unwrap_or(0),
            "Creating OpenAI-compatible provider"
        );
        let config = OpenAIConfig::new()
            .with_api_key(api_key.clone().unwrap_or_default())
            .with_api_base(base_url.clone());
        let api_base = base_url.trim_end_matches('/').to_string();
        Ok(Self {
            client: Client::with_config(config),
            provider_name: provider_name.to_string(),
            api_key,
            api_base,
            http: codetether_provider_core::socket_opts::apply_socket_opts(HttpClient::builder())
                .timeout(std::time::Duration::from_secs(45))
                .build()?,
        })
    }
}
