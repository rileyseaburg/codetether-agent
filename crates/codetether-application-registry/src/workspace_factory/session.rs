//! Session workspace composition implemented by the owning factory crate.
use super::WorkspaceFactory;
use crate::{Application, ToolRegistry, commands, session_tools};
use codetether_workspace_tools::{Host, session::Factory};
use std::path::Path;
impl<A: Application, W: Host<Registry = ToolRegistry<A>>> Factory for WorkspaceFactory<A, W> {
    fn application_mcp(registry: &mut ToolRegistry<A>) {
        session_tools::register_mcp(registry);
    }
    fn commands(registry: &mut ToolRegistry<A>, cwd: &Path) {
        commands::register(registry, Some(cwd.to_path_buf()));
    }
    fn remove_prior_context(registry: &mut ToolRegistry<A>) {
        session_tools::remove_prior_context(registry);
    }
}
