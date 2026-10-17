//! Construct an application provider without normalizing its endpoint.
use super::ZaiProvider;
use anyhow::Result;
impl ZaiProvider {
    /// Construct using the shared HTTP pool and an unmodified endpoint.
    ///
    /// # Arguments
    /// * api_key - Credential used by future provider requests.
    /// * base_url - Endpoint prefix, retained exactly as supplied.
    ///
    /// # Returns
    /// The application-compatible provider.
    ///
    /// # Errors
    /// This constructor currently always returns Ok.
    ///
    /// # Examples
    ///
    ///     use codetether_provider_zai::application::ZaiProvider;
    ///     let provider = ZaiProvider::with_base_url(String::new(), "https://example.test/v4".into()).unwrap();
    ///     assert!(format!("{provider:?}").contains("example.test"));
    pub fn with_base_url(api_key: String, base_url: String) -> Result<Self> {
        tracing::debug!(
            provider = "zai",
            base_url = %base_url,
            api_key_len = api_key.len(),
            "Creating Z.AI provider with custom base URL"
        );
        Ok(Self {
            client: codetether_provider_core::shared_http::shared_client().clone(),
            api_key,
            base_url,
        })
    }
}
