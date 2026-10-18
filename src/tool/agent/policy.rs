//! Bind shared spawn model-cost policy to existing initialized providers.
use crate::provider::ProviderRegistry;
pub(super) async fn model_warning(model: &str, registry: &ProviderRegistry) -> Option<String> {
    codetether_spawn_policy::model_warning(model, registry).await
}
