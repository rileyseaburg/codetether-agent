//! Move shared registry ownership into or out of the application wrapper.
use super::*;
impl<H: Application> From<codetether_provider_registry::ProviderRegistry> for ProviderRegistry<H> {
    fn from(inner: codetether_provider_registry::ProviderRegistry) -> Self {
        Self {
            inner,
            application: PhantomData,
        }
    }
}
impl<H: Application> ProviderRegistry<H> {
    /// Borrow the shared collection without changing provider ownership.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>(registry: &codetether_application_providers::ProviderRegistry<H>) {
    /// let _shared = registry.as_shared();
    /// # }
    /// ```
    pub fn as_shared(&self) -> &codetether_provider_registry::ProviderRegistry {
        &self.inner
    }
}
