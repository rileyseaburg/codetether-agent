//! Provider configuration using the shared HTTP pool.
use crate::*;
impl Glm5Provider {
    /// Construct with the default model.
    ///
    /// # Arguments
    /// * `api_key` - Bearer token for the endpoint.
    /// * `base_url` - API base URL, including `/v1` where required.
    ///
    /// # Returns
    /// A configured provider using the shared HTTP connection pool.
    ///
    /// # Errors
    /// This constructor currently returns success; the result supports fallible configuration.
    ///
    /// # Examples
    /// See [Glm5Provider] for a construction example.
    #[allow(dead_code)]
    pub fn new(api_key: String, base_url: String) -> Result<Self> {
        Self::with_model(api_key, base_url, DEFAULT_MODEL.to_string())
    }
    /// Construct with an explicit model name.
    ///
    /// # Arguments
    /// * `api_key` - Bearer token for the endpoint.
    /// * `base_url` - API base URL, including `/v1` where required.
    /// * `model_name` - Configured model name retained in provider diagnostics.
    ///
    /// # Returns
    /// A configured provider using the shared HTTP connection pool.
    ///
    /// # Errors
    /// This constructor currently returns success; the result supports fallible configuration.
    ///
    /// # Examples
    /// See [Glm5Provider] for a construction example.
    pub fn with_model(api_key: String, base_url: String, model_name: String) -> Result<Self> {
        let base_url = base_url.trim_end_matches('/').to_string();
        tracing::debug!(
            provider = "glm5",
            base_url = %base_url,
            model = %model_name,
            "Creating GLM-5 FP8 provider"
        );
        Ok(Self {
            client: codetether_provider_core::shared_http::shared_client().clone(),
            api_key,
            base_url,
            model_name,
        })
    }
}
