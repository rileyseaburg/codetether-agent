//! Main-swarm registry composition with safe coordination extensions.

use super::{Capability, with_provider};
use crate::{
    provider::{Provider, ProviderRegistry, ToolDefinition},
    swarm::result_store::ResultStore,
    tool::{ToolRegistry, batch::BatchTool, swarm_share::SwarmShareTool},
};
use std::{path::Path, sync::Arc};

pub(crate) fn for_agent(
    provider: Arc<dyn Provider>,
    model: String,
    root: &Path,
    capability: Capability,
    store: Arc<ResultStore>,
    producer_id: String,
) -> (Arc<ToolRegistry>, Vec<ToolDefinition>) {
    let mut registry = with_provider(Arc::clone(&provider), model, root, capability);
    registry.register(Arc::new(SwarmShareTool::new(store, producer_id)));
    let batch = Arc::new(BatchTool::new());
    if capability.is_mutating() {
        registry.register(batch.clone());
        let mut providers = ProviderRegistry::new();
        providers.register(provider);
        registry.register_search(Arc::new(providers));
    }
    let registry = Arc::new(registry);
    batch.set_registry(Arc::downgrade(&registry));
    let tools = registry.definitions();
    (registry, tools)
}
