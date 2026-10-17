//! Provider constructors with caller-owned application identity.
use crate::*;
impl CopilotProvider {
    /// Construct a provider with the supplied endpoint credentials and User-Agent.
    ///
    /// # Arguments
    /// * `token` - Bearer credential.
    /// * `user_agent` - Calling application name and version.
    ///
    /// # Returns
    /// A provider using the shared HTTP connection pool.
    ///
    /// # Errors
    /// Currently returns success; the result permits fallible configuration.
    ///
    /// # Examples
    /// See [CopilotProvider] for construction.
    pub fn new(token: String, user_agent: String) -> Result<Self> {
        Self::with_base_url(
            token,
            DEFAULT_BASE_URL.to_string(),
            COPILOT_PROVIDER,
            user_agent,
        )
    }
    /// Construct a provider with the supplied endpoint credentials and User-Agent.
    ///
    /// # Arguments
    /// * `token` - Bearer credential.
    /// * `enterprise_url` - Enterprise domain or URL.
    /// * `user_agent` - Calling application name and version.
    ///
    /// # Returns
    /// A provider using the shared HTTP connection pool.
    ///
    /// # Errors
    /// Currently returns success; the result permits fallible configuration.
    ///
    /// # Examples
    /// See [CopilotProvider] for construction.
    pub fn enterprise(token: String, enterprise_url: String, user_agent: String) -> Result<Self> {
        let base_url = enterprise_base_url(&enterprise_url);
        Self::with_base_url(token, base_url, COPILOT_ENTERPRISE_PROVIDER, user_agent)
    }
    /// Construct a provider with the supplied endpoint credentials and User-Agent.
    ///
    /// # Arguments
    /// * `token` - Bearer credential.
    /// * `base_url` - API base URL.
    /// * `provider_name` - Provider identity exposed by the trait.
    /// * `user_agent` - Calling application name and version.
    ///
    /// # Returns
    /// A provider using the shared HTTP connection pool.
    ///
    /// # Errors
    /// Currently returns success; the result permits fallible configuration.
    ///
    /// # Examples
    /// See [CopilotProvider] for construction.
    pub fn with_base_url(
        token: String,
        base_url: String,
        provider_name: &str,
        user_agent: String,
    ) -> Result<Self> {
        Ok(Self {
            client: codetether_provider_core::shared_http::shared_client().clone(),
            token,
            user_agent,
            base_url: base_url.trim_end_matches('/').to_string(),
            provider_name: provider_name.to_string(),
        })
    }
}
