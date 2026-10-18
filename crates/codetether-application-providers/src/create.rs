//! Empty and credential-backed provider registry construction.
use super::*;
impl<H: Application> ProviderRegistry<H> {
    /// Construct an empty registry.
    /// # Arguments
    /// The application binding is selected by `H`.
    /// # Returns
    /// An empty registry.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>() {
    /// assert!(codetether_application_providers::ProviderRegistry::<H>::new().list().is_empty());
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            application: PhantomData,
            inner: codetether_provider_registry::ProviderRegistry::new(),
        }
    }
    /// Build providers from the supplied secret records.
    /// # Arguments
    /// `secrets` maps provider identifiers to the existing secret records.
    /// # Returns
    /// A registry constructed with the application factory.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>() {
    /// let secrets = std::collections::HashMap::new();
    /// let _registry = codetether_application_providers::ProviderRegistry::<H>::from_provider_secrets_map(&secrets);
    /// # }
    /// ```
    pub fn from_provider_secrets_map(secrets: &HashMap<String, ProviderSecrets>) -> Self {
        Self {
            application: PhantomData,
            inner: codetether_provider_registry::ProviderRegistry::from_provider_secrets_map::<
                H::Factory,
            >(secrets),
        }
    }
}
impl<H: Application> Default for ProviderRegistry<H> {
    fn default() -> Self {
        Self::new()
    }
}
