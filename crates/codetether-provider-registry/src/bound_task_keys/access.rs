//! Read credential metadata through the shared credential owner.
use super::BoundTaskKeys;
impl<H, R> BoundTaskKeys<H, R> {
    /// Return the claim source or its default label.
    /// # Examples
    /// ```no_run
    /// # fn example(keys: &codetether_provider_registry::BoundTaskKeys<(), ()>) {
    /// let _source = keys.source();
    /// # }
    /// ```
    pub fn source(&self) -> &str {
        self.inner.source()
    }
    /// Return deterministic provider identifiers.
    /// # Examples
    /// ```no_run
    /// # fn example(keys: &codetether_provider_registry::BoundTaskKeys<(), ()>) {
    /// let _names = keys.provider_names();
    /// # }
    /// ```
    pub fn provider_names(&self) -> Vec<String> {
        self.inner.provider_names()
    }
}
