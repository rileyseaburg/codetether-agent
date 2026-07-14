//! Workspace-scoped tool registries for swarm participants.

use crate::{provider::Provider, tool::ToolRegistry};
use std::{path::Path, sync::Arc};

mod aliases;
mod capability;
mod file_uri;
mod local;
mod policy;
mod root;
mod scoped_bash;
mod scoped_git;
mod scoped_tools;

pub(crate) use capability::Capability;
pub(crate) use local::for_agent;
pub(crate) use root::resolve_root;

pub(crate) fn with_provider(
    provider: Arc<dyn Provider>,
    model: String,
    root: &Path,
    capability: Capability,
) -> ToolRegistry {
    finish(
        ToolRegistry::with_provider(provider, model),
        root,
        capability,
    )
}

#[cfg(test)]
pub(crate) fn with_defaults(root: &Path, capability: Capability) -> ToolRegistry {
    finish(ToolRegistry::with_defaults(), root, capability)
}

fn finish(mut registry: ToolRegistry, root: &Path, capability: Capability) -> ToolRegistry {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    scoped_tools::install(&mut registry, &root, capability);
    policy::apply(&mut registry, capability);
    registry
}

#[cfg(test)]
#[path = "../workspace_registry_tests.rs"]
mod tests;
