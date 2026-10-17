//! Task-claim metadata and deterministic provider names.
use super::*;
impl TenantProviderKeyPayload {
    /// Return the supplied source label or the default BYOK label.
    pub fn source(&self) -> &str {
        self.source.as_deref().unwrap_or("byok")
    }

    /// Return provider identifiers in sorted order.
    pub fn provider_names(&self) -> Vec<String> {
        let mut names = self.providers.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }
}
