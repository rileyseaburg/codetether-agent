//! Native catalog and coordination constructors for shared composition.
use super::host::Native;
use crate::{
    provider::{Provider, ProviderRegistry, ToolDefinition},
    tool::{ToolRegistry, batch::BatchTool},
};
use std::sync::{Arc, Weak};
impl codetether_workspace_tools::composition::Factory for Native {
    type Batch = BatchTool;
    fn defaults() -> ToolRegistry {
        ToolRegistry::with_defaults()
    }
    fn provider(provider: Arc<dyn Provider>, model: String) -> ToolRegistry {
        ToolRegistry::with_provider(provider, model)
    }
    fn batch() -> Arc<BatchTool> {
        Arc::new(BatchTool::new())
    }
    fn bind_batch(batch: &BatchTool, registry: Weak<ToolRegistry>) {
        batch.set_registry(registry);
    }
    fn search(registry: &mut ToolRegistry, provider: Arc<dyn Provider>) {
        let mut providers = ProviderRegistry::new();
        providers.register(provider);
        registry.register_search(Arc::new(providers));
    }
    fn definitions(registry: &ToolRegistry) -> Vec<ToolDefinition> {
        registry.definitions()
    }
}
