//! Base direct-swarm registry construction.

use crate::{provider::Provider, swarm::executor::workspace_registry, tool::ToolRegistry};
use std::{path::Path, sync::Arc};

pub(crate) fn standard(
    read_only: bool,
    verification: bool,
    workspace: &Path,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<ToolRegistry> {
    codetether_workspace_tools::composition::direct::standard::<workspace_registry::RegistryHost>(
        read_only,
        verification,
        workspace,
        provider,
        model,
    )
}
