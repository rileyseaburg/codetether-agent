use super::{Factory, with_provider};
use crate::{Capability, Registry};
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::result_store::ResultStore;
use codetether_swarm_share::SwarmShareTool;
use std::{path::Path, sync::Arc};
/// Compose the main swarm registry with capability-appropriate coordination.
/// # Arguments
/// * `provider`, `model` — Completion backend and selected model.
/// * `root`, `capability` — Assigned workspace and allowed operations.
/// * `store`, `producer_id` — Shared result storage and publisher identity.
/// # Returns
/// The completed shared registry and its provider-facing definitions.
/// Batch and routed search are registered only for mutating capability.
/// # Examples
/// ```no_run
/// # fn make<H: codetether_workspace_tools::composition::Factory>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let (_registry, _tools) = codetether_workspace_tools::composition::for_agent::<H>(provider, "model".into(), std::path::Path::new("."), codetether_workspace_tools::Capability::ReadOnly, codetether_swarm_core::result_store::ResultStore::new_arc(), "worker".into());
/// # }
/// ```
pub fn for_agent<H: Factory>(
    provider: Arc<dyn Provider>,
    model: String,
    root: &Path,
    capability: Capability,
    store: Arc<ResultStore>,
    producer_id: String,
) -> (Arc<H::Registry>, Vec<ToolDefinition>) {
    let mut registry = with_provider::<H>(Arc::clone(&provider), model, root, capability);
    registry.register(Arc::new(SwarmShareTool::new(store, producer_id)));
    let batch = H::batch();
    if capability.is_mutating() {
        registry.register(batch.clone());
        H::search(&mut registry, provider);
    }
    let registry = Arc::new(registry);
    H::bind_batch(&batch, Arc::downgrade(&registry));
    let tools = H::definitions(&registry);
    (registry, tools)
}
