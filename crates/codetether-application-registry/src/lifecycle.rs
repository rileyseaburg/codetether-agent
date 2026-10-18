//! Native constructors and weak batch binding for the shared registry lifecycle.
use super::{Application, ToolRegistry};
use codetether_provider_core::Provider;
use std::marker::PhantomData;
use std::sync::{Arc, Weak};

pub(super) struct Policy<H>(PhantomData<fn() -> H>);

impl<H: Application> codetether_tool_registry::lifecycle::Host for Policy<H> {
    type Registry = ToolRegistry<H>;
    type Batch = codetether_batch_tool::BatchTool<ToolRegistry<H>>;

    fn defaults() -> ToolRegistry<H> {
        ToolRegistry::with_defaults()
    }
    fn provider(provider: Arc<dyn Provider>, model: String) -> ToolRegistry<H> {
        ToolRegistry::with_provider(provider, model)
    }
    fn batch() -> Arc<Self::Batch> {
        Arc::new(codetether_batch_tool::BatchTool::new())
    }
    fn register_batch(registry: &mut ToolRegistry<H>, batch: Arc<Self::Batch>) {
        registry.register(batch);
    }
    fn register_search(registry: &mut ToolRegistry<H>, provider: Arc<dyn Provider>) {
        H::register_provider_search(registry, provider);
    }
    fn bind_batch(batch: &Self::Batch, registry: Weak<ToolRegistry<H>>) {
        batch.set_registry(registry);
    }
}
