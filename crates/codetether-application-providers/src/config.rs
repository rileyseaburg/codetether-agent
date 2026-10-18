//! Configuration-backed provider construction.
use super::*;
impl<H: Application> ProviderRegistry<H> {
    /// Initialize providers from configuration and allowed fallbacks.
    /// # Errors
    /// Returns the configured provider construction failure.
    /// # Arguments
    /// `config` supplies provider configuration and fallback policy.
    /// # Returns
    /// A registry populated from the configuration.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_application_providers::Application>(config: &codetether_config::Config) -> anyhow::Result<()> {
    /// let _registry = codetether_application_providers::ProviderRegistry::<H>::from_config(config).await?;
    /// # Ok(()) }
    /// ```
    pub async fn from_config(config: &codetether_config::Config) -> Result<Self> {
        Ok(Self::from(
            codetether_provider_registry::ProviderRegistry::from_config::<H::Factory>(config)
                .await?,
        ))
    }
}
