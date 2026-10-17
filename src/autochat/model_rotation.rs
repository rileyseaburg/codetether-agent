//! Application registry adapter for shared auto-chat model selection.
use crate::provider::{Provider, ProviderRegistry};
pub use codetether_autochat_core::model_rotation::RelayModelRotation;
use codetether_autochat_core::model_rotation::{self, ModelRegistry};
use std::sync::Arc;
impl ModelRegistry for ProviderRegistry {
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
}
/// Build rotation using the existing initialized application registry.
pub async fn build_round_robin_model_rotation(
    registry: &Arc<ProviderRegistry>,
    requested_model_ref: &str,
) -> RelayModelRotation {
    model_rotation::build_round_robin_model_rotation(registry.as_ref(), requested_model_ref).await
}
/// Resolve a provider handle using the shared selection policy.
pub fn resolve_provider_for_model_autochat(
    registry: &Arc<ProviderRegistry>,
    model_ref: &str,
) -> Option<(Arc<dyn Provider>, String)> {
    model_rotation::resolve_provider_for_model_autochat(registry.as_ref(), model_ref)
}
