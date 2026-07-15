//! Executable, autonomous tool registries for direct swarm workers.

use crate::provider::Provider;
#[cfg(test)]
use crate::swarm::executor::workspace_registry;
use crate::swarm::result_store::ResultStore;
use crate::tool::{ToolRegistry, swarm_share::SwarmShareTool};
use std::{path::Path, sync::Arc};

#[path = "agent_registry_base.rs"]
mod base;
pub(crate) use base::standard;

pub(super) fn shared(
    read_only: bool,
    verification: bool,
    workspace: &Path,
    store: Arc<ResultStore>,
    producer_id: String,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<ToolRegistry> {
    with_shared(
        base::build(read_only, verification, workspace, provider, model),
        store,
        producer_id,
    )
}

fn with_shared(
    mut registry: ToolRegistry,
    store: Arc<ResultStore>,
    producer_id: String,
) -> Arc<ToolRegistry> {
    registry.register(Arc::new(SwarmShareTool::new(store, producer_id)));
    Arc::new(registry)
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
