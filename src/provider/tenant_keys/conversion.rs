//! Transfer shared registry ownership into the native application wrapper.
use super::ProviderRegistry;
impl From<codetether_provider_registry::ProviderRegistry> for ProviderRegistry {
    fn from(inner: codetether_provider_registry::ProviderRegistry) -> Self {
        Self { inner }
    }
}
