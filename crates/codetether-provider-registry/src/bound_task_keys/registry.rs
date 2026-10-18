//! Construct providers with the bound factory and convert registry ownership.
use super::{BoundTaskKeys, ProviderRegistry};
impl<H: codetether_provider_factory::Host, R: From<ProviderRegistry>> BoundTaskKeys<H, R> {
    /// Construct application providers from the owned credentials.
    /// # Arguments
    /// Borrows the credentials; `H` supplies provider construction and `R` the registry type.
    /// # Returns
    /// The constructed registry with the original credential owner retained by `self`.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_provider_factory::Host>(keys: &codetether_provider_registry::BoundTaskKeys<H, codetether_provider_registry::ProviderRegistry>) {
    /// let _registry = keys.build_registry();
    /// # }
    /// ```
    pub fn build_registry(&self) -> R {
        self.inner.build_registry::<H>().into()
    }
}
