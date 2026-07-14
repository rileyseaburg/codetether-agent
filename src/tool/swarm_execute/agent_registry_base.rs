//! Base direct-swarm registry construction.

use crate::{provider::Provider, swarm::executor::workspace_registry, tool::ToolRegistry};
use std::{path::Path, sync::Arc};

pub(super) fn build(
    read_only: bool,
    verification: bool,
    workspace: &Path,
    provider: Arc<dyn Provider>,
    model: String,
) -> ToolRegistry {
    let capability = workspace_registry::Capability::from_flags(read_only, verification);
    workspace_registry::with_provider(provider, model, workspace, capability)
}

pub(crate) fn standard(
    read_only: bool,
    verification: bool,
    workspace: &Path,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<ToolRegistry> {
    Arc::new(build(read_only, verification, workspace, provider, model))
}
