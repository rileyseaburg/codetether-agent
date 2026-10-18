//! Native registry construction and process-wide caching for Vault providers.
use super::registry::ProviderRegistry;
use anyhow::Result;
use std::sync::Arc;
impl ProviderRegistry {
    /// Load providers with shared Vault and fallback policy.
    /// # Errors
    /// Returns Vault provider discovery failures.
    pub async fn from_vault() -> Result<Self> {
        Ok(Self {
            inner: codetether_provider_registry::ProviderRegistry::from_vault::<
                super::init_dispatch::host::Native,
            >()
            .await?,
        })
    }

    /// Process-wide cached [`from_vault`](Self::from_vault) registry.
    ///
    /// `from_vault` performs Vault fetches plus environment and AWS probing on
    /// every call. Runtime prompt, server, TUI, and tool paths should use this
    /// accessor so provider discovery happens only once per process.
    ///
    /// The cache is process-global. Restart the binary to pick up
    /// re-keyed providers.
    ///
    /// # Errors
    ///
    /// Propagates the underlying [`from_vault`](Self::from_vault) error
    /// on the first call. Subsequent calls reuse the cached value and
    /// cannot fail.
    pub async fn shared_from_vault() -> Result<Arc<Self>> {
        use tokio::sync::OnceCell;
        static CACHED: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();
        CACHED
            .get_or_try_init(|| async { Self::from_vault().await.map(Arc::new) })
            .await
            .cloned()
    }
}
