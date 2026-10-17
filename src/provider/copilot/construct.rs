//! Constructors preserving the application's User-Agent identity.
use super::CopilotProvider;
use anyhow::Result;
use codetether_provider_copilot::CopilotProvider as Inner;
impl CopilotProvider {
    /// Create a provider for the public Copilot endpoint.
    ///
    /// # Arguments
    /// * `token` - Copilot bearer credential.
    /// # Returns
    /// A provider using the application's User-Agent.
    /// # Errors
    /// Propagates provider construction errors.
    /// # Examples
    /// See [CopilotProvider] for construction.
    pub fn new(token: String) -> Result<Self> {
        Inner::new(token, Self::user_agent()).map(Self)
    }
    /// Create a provider for an enterprise domain.
    ///
    /// # Arguments
    /// * `token` - Copilot bearer credential.
    /// * `enterprise_url` - Enterprise domain or URL.
    /// # Returns
    /// A provider for that enterprise.
    /// # Errors
    /// Propagates provider construction errors.
    /// # Examples
    /// ```no_run
    /// let provider = codetether_agent::provider::copilot::CopilotProvider::enterprise(
    ///     "token".into(), "example.ghe.com".into())?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn enterprise(token: String, enterprise_url: String) -> Result<Self> {
        Inner::enterprise(token, enterprise_url, Self::user_agent()).map(Self)
    }
    /// Create a provider with an explicit API URL and registry name.
    ///
    /// # Arguments
    /// * `token` - Copilot bearer credential.
    /// * `base_url` - API base URL.
    /// * `provider_name` - Registry name exposed by the provider.
    /// # Returns
    /// A provider with the requested endpoint and application identity.
    /// # Errors
    /// Propagates provider construction errors.
    /// # Examples
    /// ```no_run
    /// let provider = codetether_agent::provider::copilot::CopilotProvider::with_base_url(
    ///     "token".into(), "https://example.invalid".into(), "custom-copilot")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn with_base_url(token: String, base_url: String, provider_name: &str) -> Result<Self> {
        Inner::with_base_url(token, base_url, provider_name, Self::user_agent()).map(Self)
    }
    fn user_agent() -> String {
        format!("codetether-agent/{}", env!("CARGO_PKG_VERSION"))
    }
}
