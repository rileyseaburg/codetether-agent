//! Shared-client provider construction.
use crate::*;
impl MoonshotProvider {
    /// Construct a provider for the existing Moonshot API endpoint.
    ///
    /// # Arguments
    /// * api_key - Bearer credential for future requests.
    ///
    /// # Returns
    /// A provider using the process-wide shared HTTP client.
    ///
    /// # Errors
    /// This constructor currently always returns Ok.
    ///
    /// # Examples
    ///
    ///     assert!(codetether_provider_moonshot::MoonshotProvider::new(String::new()).is_ok());
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            client: codetether_provider_core::shared_http::shared_client().clone(),
            api_key,
            base_url: "https://api.moonshot.ai/v1".to_string(),
        })
    }
}
