//! Use the same catalog, batch binding and routed search as registry construction.
use super::WorkspaceFactory;
use crate::{Application, ToolRegistry, lifecycle::Policy};
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_tool_registry::lifecycle::Host as Lifecycle;
use codetether_workspace_tools::{Host, composition::Factory};
use std::sync::{Arc, Weak};
impl<A: Application, W: Host<Registry = ToolRegistry<A>>> Factory for WorkspaceFactory<A, W> {
    type Batch = codetether_batch_tool::BatchTool<ToolRegistry<A>>;
    fn defaults() -> ToolRegistry<A> {
        Policy::<A>::defaults()
    }
    fn provider(provider: Arc<dyn Provider>, model: String) -> ToolRegistry<A> {
        Policy::<A>::provider(provider, model)
    }
    fn batch() -> Arc<Self::Batch> {
        Policy::<A>::batch()
    }
    fn bind_batch(batch: &Self::Batch, registry: Weak<ToolRegistry<A>>) {
        Policy::<A>::bind_batch(batch, registry);
    }
    fn search(registry: &mut ToolRegistry<A>, provider: Arc<dyn Provider>) {
        Policy::<A>::register_search(registry, provider);
    }
    fn definitions(registry: &ToolRegistry<A>) -> Vec<ToolDefinition> {
        registry.definitions()
    }
}
