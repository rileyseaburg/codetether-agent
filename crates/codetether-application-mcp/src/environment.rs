//! Build an optional connection from trusted process environment settings.
use super::{ApplicationMcpTool, config};
/// Load the configured authenticated application MCP connection.
///
/// Reads `CODETETHER_APPLICATION_MCP_URL` or its legacy URL alias, connection
/// identity/name overrides, and the configured bearer token or its fallback.
///
/// # Arguments
/// None; configuration comes from the process environment.
/// # Returns
/// `None` when no endpoint is configured, otherwise a ready-to-register tool.
/// # Errors
/// Returns invalid identifier/URL, missing token, or HTTP client construction errors.
/// # Examples
/// ```rust,no_run
/// if let Some(tool)=codetether_application_mcp::from_env()? {
///     use codetether_tool_core::Tool;
///     assert!(!tool.id().is_empty());
/// }
/// # Ok::<(),anyhow::Error>(())
/// ```
pub fn from_env() -> anyhow::Result<Option<ApplicationMcpTool>> {
    config::load().and_then(|value| value.map(ApplicationMcpTool::new).transpose())
}
