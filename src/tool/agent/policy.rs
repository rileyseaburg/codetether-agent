//! Bind shared spawn model-cost policy to existing initialized providers.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_spawn_policy::ModelRegistry for ProviderRegistry {
    fn get(&self, provider: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, provider)
    }
}
pub(super) async fn model_warning(model: &str, registry: &ProviderRegistry) -> Option<String> {
    codetether_spawn_policy::model_warning(model, registry).await
}
