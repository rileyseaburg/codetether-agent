//! Catalog provider ids.
use super::*;
impl ModelCatalog {
    /// List all provider IDs (all, not filtered)
    #[allow(dead_code)]
    pub fn provider_ids(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }
}
