//! Native constructors and weak batch binding for the shared registry lifecycle.
use super::{ToolRegistry, batch, search_router};
use crate::provider::{Provider, ProviderRegistry};
use std::sync::{Arc, Weak};

pub(super) struct Native;

impl codetether_tool_registry::lifecycle::Host for Native {
    type Registry = ToolRegistry;
    type Batch = batch::BatchTool;

    fn defaults() -> ToolRegistry {
        ToolRegistry::with_defaults()
    }
    fn provider(provider: Arc<dyn Provider>, model: String) -> ToolRegistry {
        ToolRegistry::with_provider(provider, model)
    }
    fn batch() -> Arc<Self::Batch> {
        Arc::new(batch::BatchTool::new())
    }
    fn register_batch(registry: &mut ToolRegistry, batch: Arc<Self::Batch>) {
        registry.register(batch);
    }
    fn register_search(registry: &mut ToolRegistry, provider: Arc<dyn Provider>) {
        let mut providers = ProviderRegistry::new();
        providers.register(Arc::clone(&provider));
        registry.register(Arc::new(search_router::SearchTool::new(Arc::new(
            providers,
        ))));
    }
    fn bind_batch(batch: &Self::Batch, registry: Weak<ToolRegistry>) {
        batch.set_registry(registry);
    }
}
