//! Register the environment-configured authenticated application MCP connection.
use crate::{Application, ToolRegistry};
use std::sync::Arc;
/// Install the application MCP tool when configured and valid.
/// # Arguments
/// `registry` receives the configured tool.
/// # Returns
/// Completes without registration when absent, or logs a configuration error.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_registry::Application>(registry: &mut codetether_application_registry::ToolRegistry<H>) {
/// codetether_application_registry::session_tools::register_mcp(registry);
/// # }
/// ```
pub fn register_mcp<H: Application>(registry: &mut ToolRegistry<H>) {
    match codetether_application_mcp::from_env() {
        Ok(Some(tool)) => registry.register(Arc::new(tool)),
        Ok(None) => {}
        Err(error) => {
            tracing::warn!(target: "codetether_agent::tool::application_mcp", error = %error, "Application MCP connection disabled")
        }
    }
}
