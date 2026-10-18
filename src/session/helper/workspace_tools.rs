//! Native bindings for shared session workspace composition.
use crate::{provider::Provider, tool::ToolRegistry};
#[cfg(test)]
use codetether_workspace_tools::session::autonomous;
use std::{path::Path, sync::Arc};
#[cfg(test)]
#[path = "workspace_tools/autonomous_tests.rs"]
mod autonomous_tests;
#[path = "workspace_tools/host.rs"]
mod host;
pub(crate) fn registry_for_cwd(
    provider: Arc<dyn Provider>,
    model: &str,
    cwd: &Path,
    prior_context_allowed: bool,
    autonomous: bool,
) -> Arc<ToolRegistry> {
    codetether_workspace_tools::session::registry_for_cwd::<
        crate::swarm::executor::workspace_registry::RegistryHost,
    >(provider, model, cwd, prior_context_allowed, autonomous)
}
