//! Native provider registry adapter for shared model discovery.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_model_catalog::discovery::CatalogRegistry for ProviderRegistry {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}
pub(crate) async fn collect_models(
    registry: &ProviderRegistry,
    created: i64,
) -> Vec<codetether_model_catalog::discovery::types::Model> {
    codetether_model_catalog::discovery::collect_models(registry, created).await
}
