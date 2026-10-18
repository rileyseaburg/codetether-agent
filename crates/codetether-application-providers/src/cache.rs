//! Call-site-owned caches over the application's shared provider registry.
//!
//! [`RegistryCache`] keeps an independent one-time binding for a consumer,
//! loading from the application's shared Vault registry on first access.
use crate::{Application, ProviderRegistry};
use std::sync::Arc;
use tokio::sync::OnceCell;

/// One-time provider binding owned by an application consumer.
/// # Examples
/// ```
/// # fn example<H: codetether_application_providers::Application>() {
/// let cache = codetether_application_providers::cache::RegistryCache::<H>::new();
/// assert!(cache.initialize(std::sync::Arc::new(
///     codetether_application_providers::ProviderRegistry::<H>::new()
/// )));
/// # }
/// ```
pub struct RegistryCache<H: Application>(OnceCell<Arc<ProviderRegistry<H>>>);

impl<H: Application> RegistryCache<H> {
    /// Construct an empty cache suitable for a static consumer binding.
    pub const fn new() -> Self {
        Self(OnceCell::const_new())
    }

    /// Load or clone the cached shared provider registry.
    /// # Arguments
    /// Uses the cache and provider application selected by `Self`.
    /// # Returns
    /// The cached registry after successful initialization.
    /// # Errors
    /// Propagates provider initialization errors; failures leave the cache empty.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
    /// let cache = codetether_application_providers::cache::RegistryCache::<H>::new();
    /// let _registry = cache.get().await?;
    /// # Ok(()) }
    /// ```
    pub async fn get(&self) -> anyhow::Result<Arc<ProviderRegistry<H>>> {
        self.0
            .get_or_try_init(ProviderRegistry::<H>::shared_from_vault)
            .await
            .cloned()
    }

    /// Seed an empty cache, returning false if it is already bound or initializing.
    pub fn initialize(&self, registry: Arc<ProviderRegistry<H>>) -> bool {
        self.0.set(registry).is_ok()
    }
}

impl<H: Application> Default for RegistryCache<H> {
    fn default() -> Self {
        Self::new()
    }
}
