//! Shared HTTP client construction and key validation.
use crate::*;
impl GoogleProvider {
    /// Construct a provider with the existing Google endpoint.
    ///
    /// # Arguments
    /// * api_key - Bearer credential used by future requests.
    ///
    /// # Returns
    /// A provider using the shared HTTP client.
    ///
    /// # Errors
    /// This constructor currently always returns Ok.
    ///
    /// # Examples
    ///
    ///     assert!(codetether_provider_google::GoogleProvider::new(String::new()).is_ok());
    pub fn new(api_key: String) -> Result<Self> {
        tracing::debug!(
            provider = "google",
            api_key_len = api_key.len(),
            "Creating Google Gemini provider"
        );
        Ok(Self {
            client: codetether_provider_core::shared_http::shared_client().clone(),
            api_key,
        })
    }
    pub(crate) fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("Google API key is empty");
        }
        Ok(())
    }
}
