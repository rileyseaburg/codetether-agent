//! Provider registry create operations.
use super::*;
impl ProviderRegistry {
    /// Create an empty registry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_provider_registry::ProviderRegistry;
    /// let registry = ProviderRegistry::new();
    /// assert!(registry.list().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }
}
