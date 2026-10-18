//! question provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_side_question::Registry for ProviderRegistry<H> {
    fn resolve(registry: &Arc<Self>, model: &str) -> Option<(Arc<dyn Provider>, String)> {
        codetether_autochat_core::model_rotation::resolve_provider_for_model_autochat(
            registry.as_ref(),
            model,
        )
    }
}
