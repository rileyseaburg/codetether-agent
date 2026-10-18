//! Vault initialization and reuse of the application's process-wide cache.
use super::*;
impl<H: Application> ProviderRegistry<H> {
    /// Load providers using the existing Vault and fallback policy.
    /// # Errors
    /// Propagates Vault provider discovery failures.
    /// # Arguments
    /// The application binding is selected by `H`.
    /// # Returns
    /// A registry populated from Vault and permitted fallbacks.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
    /// let _registry = codetether_application_providers::ProviderRegistry::<H>::from_vault().await?;
    /// # Ok(()) }
    /// ```
    pub async fn from_vault() -> Result<Self> {
        Ok(Self::from(
            codetether_provider_registry::ProviderRegistry::from_vault::<H::Factory>().await?,
        ))
    }
    /// Load the registry once and reuse the application's cache for subsequent calls.
    /// Failed initialization leaves the cache empty so a later call can retry.
    /// # Errors
    /// Propagates the underlying initialization error.
    /// # Arguments
    /// The application supplies a process-wide cache cell.
    /// # Returns
    /// The shared cached registry, initialized once after a successful load.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
    /// let _registry = codetether_application_providers::ProviderRegistry::<H>::shared_from_vault().await?;
    /// # Ok(()) }
    /// ```
    pub async fn shared_from_vault() -> Result<Arc<Self>> {
        H::cache()
            .get_or_try_init(|| async { Self::from_vault().await.map(Arc::new) })
            .await
            .cloned()
    }
}
