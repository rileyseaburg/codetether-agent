//! Configuration and Vault construction with application-supplied providers.
use super::*;
impl ProviderRegistry {
    /// Initialize providers from configuration and permitted fallbacks.
    /// # Arguments
    /// * `config` — Application configuration for provider initialization.
    /// # Returns
    /// An instrumented registry populated under the shared fallback policy.
    /// # Errors
    /// Returns the first configured provider constructor failure.
    /// # Examples
    /// ```no_run
    /// # async fn load<H: codetether_provider_factory::Host>(config: &codetether_config::Config) -> anyhow::Result<()> {
    /// let _registry = codetether_provider_registry::ProviderRegistry::from_config::<H>(config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn from_config<H: codetether_provider_factory::Host>(
        config: &codetether_config::Config,
    ) -> Result<Self> {
        let mut registry = Self::new();
        codetether_provider_factory::config::populate_from_config::<H>(&mut registry, config)
            .await?;
        Ok(registry)
    }
    /// Initialize providers from Vault and permitted fallbacks.
    /// # Returns
    /// An instrumented registry populated from Vault and permitted fallbacks.
    /// # Errors
    /// Returns Vault provider discovery failures.
    /// # Examples
    /// ```no_run
    /// # async fn load<H: codetether_provider_factory::Host>() -> anyhow::Result<()> {
    /// let _registry = codetether_provider_registry::ProviderRegistry::from_vault::<H>().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn from_vault<H: codetether_provider_factory::Host>() -> Result<Self> {
        let mut registry = Self::new();
        codetether_provider_factory::vault::populate_from_vault::<H>(&mut registry).await?;
        Ok(registry)
    }
}
