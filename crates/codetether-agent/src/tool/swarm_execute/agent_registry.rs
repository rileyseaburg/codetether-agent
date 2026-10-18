//! Native registry composition with concrete-catalog integration tests.
#[cfg(test)]
use crate::swarm::{executor::workspace_registry, result_store::ResultStore};
#[cfg(test)]
use crate::tool::ToolRegistry;
#[cfg(test)]
use std::{path::Path, sync::Arc};
#[cfg(test)]
fn with_shared(
    registry: ToolRegistry,
    store: Arc<ResultStore>,
    producer_id: String,
) -> Arc<ToolRegistry> {
    codetether_workspace_tools::composition::with_shared(registry, store, producer_id)
}

#[cfg(test)]
pub(super) fn standard_for_test(
    workspace: &Path,
    capability: workspace_registry::Capability,
) -> Arc<ToolRegistry> {
    Arc::new(workspace_registry::with_defaults(workspace, capability))
}

#[cfg(test)]
#[path = "agent_registry_tests.rs"]
mod tests;
