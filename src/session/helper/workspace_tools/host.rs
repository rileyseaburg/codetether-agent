//! Session-specific callbacks on the existing application registry factory.
use crate::{swarm::executor::workspace_registry::RegistryHost, tool::ToolRegistry};
use std::path::Path;
impl codetether_workspace_tools::session::Factory for RegistryHost {
    fn application_mcp(registry: &mut ToolRegistry) {
        crate::tool::application_mcp::register(registry);
    }
    fn commands(registry: &mut ToolRegistry, cwd: &Path) {
        crate::tool::command_session::register(registry, Some(cwd.to_path_buf()));
    }
    fn remove_prior_context(registry: &mut ToolRegistry) {
        crate::session::helper::runtime::remove_prior_context_tools(registry);
    }
}
