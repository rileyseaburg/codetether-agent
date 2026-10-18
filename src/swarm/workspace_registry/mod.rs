//! Workspace-scoped tool registries for swarm participants.

use crate::{provider::Provider, tool::ToolRegistry};
use std::{path::Path, sync::Arc};

mod composition_host;
mod host;
pub(crate) use host::Native as RegistryHost;
mod local;

pub(crate) use codetether_workspace_tools::Capability;
pub(crate) use codetether_workspace_tools::resolve_root;
pub(crate) use local::for_agent;

pub(crate) fn with_provider(
    provider: Arc<dyn Provider>,
    model: String,
    root: &Path,
    capability: Capability,
) -> ToolRegistry {
    codetether_workspace_tools::composition::with_provider::<host::Native>(
        provider, model, root, capability,
    )
}

#[cfg(test)]
pub(crate) fn with_defaults(root: &Path, capability: Capability) -> ToolRegistry {
    codetether_workspace_tools::composition::with_defaults::<host::Native>(root, capability)
}

#[cfg(test)]
#[path = "../workspace_registry_tests.rs"]
mod tests;
