//! Executable, autonomous tool registries for direct swarm workers.

use crate::provider::Provider;
#[cfg(test)]
use crate::swarm::executor::workspace_registry;
use crate::swarm::result_store::ResultStore;
use crate::tool::ToolRegistry;
use std::{path::Path, sync::Arc};

#[path = "agent_registry_base.rs"]
mod base;
pub(crate) use base::standard;

pub(super) fn shared(
    read_only: bool,
    verification: bool,
    expects_changes: bool,
    workspace: &Path,
    store: Arc<ResultStore>,
    producer_id: String,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<ToolRegistry> {
    codetether_workspace_tools::composition::direct::shared::<
        crate::swarm::executor::workspace_registry::RegistryHost,
    >(
        read_only,
        verification,
        expects_changes,
        workspace,
        store,
        producer_id,
        provider,
        model,
    )
}

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
