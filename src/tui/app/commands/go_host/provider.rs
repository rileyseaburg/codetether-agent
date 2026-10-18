//! Native provider lookup for interactive goal drafting.
use super::Go;
use crate::provider::{Provider, ProviderRegistry};
use codetether_relay_okr::interactive::DraftHost;
use std::sync::Arc;
impl DraftHost for Go<'_> {
    type Registry = ProviderRegistry;
    async fn load_registry() -> Option<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await.ok()
    }
    fn resolve(
        registry: &Arc<ProviderRegistry>,
        model: &str,
    ) -> Option<(Arc<dyn Provider>, String)> {
        crate::autochat::model_rotation::resolve_provider_for_model_autochat(registry, model)
    }
}
