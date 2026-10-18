//! Workspace-scoped tool registries for swarm participants.

#[cfg(test)]
#[cfg(not(test))]
use crate::tool::ToolRegistry;
#[cfg(test)]
#[cfg(not(test))]
use std::path::Path;

mod host;
#[cfg(test)]
mod local;
pub(crate) use host::Native as RegistryHost;

#[cfg(test)]
pub(crate) use codetether_workspace_tools::Capability;

#[cfg(test)]
pub(crate) fn with_defaults(root: &Path, capability: Capability) -> ToolRegistry {
    codetether_workspace_tools::composition::with_defaults::<host::Native>(root, capability)
}

#[cfg(test)]
#[path = "../workspace_registry_tests.rs"]
mod tests;

#[cfg(test)]
include!("mod_test_imports.rs");
