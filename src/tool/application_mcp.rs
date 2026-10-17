//! Register the environment-configured authenticated application MCP connection.
use crate::tool::ToolRegistry;
use std::sync::Arc;

pub(crate) fn register(registry: &mut ToolRegistry) {
    match codetether_application_mcp::from_env() {
        Ok(Some(tool)) => registry.register(Arc::new(tool)),
        Ok(None) => {}
        Err(error) => tracing::warn!(error = %error, "Application MCP connection disabled"),
    }
}
