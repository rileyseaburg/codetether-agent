//! Client pool and transport timeouts.
use crate::*;
impl OpenRouterProvider {
    /// Construct a provider with the existing OpenRouter transport configuration.
    ///
    /// # Arguments
    /// * api_key - Bearer credential used by requests.
    ///
    /// # Returns
    /// A configured provider.
    ///
    /// # Errors
    /// Returns the HTTP client construction error if configuration fails.
    ///
    /// # Examples
    ///
    ///     assert!(codetether_provider_openrouter::OpenRouterProvider::new(String::new()).is_ok());
    pub fn new(api_key: String) -> Result<Self> {
        let client = codetether_provider_core::socket_opts::apply_socket_opts(Client::builder())
            .connect_timeout(std::time::Duration::from_secs(15))
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .context("Failed to build reqwest client")?;
        Ok(Self {
            client,
            api_key,
            base_url: "https://openrouter.ai/api/v1".to_string(),
        })
    }
}
