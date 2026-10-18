//! Native entry points for shared invocation configuration and policy flow.
use crate::tool::ToolResult;
use serde_json::Value;
use std::path::Path;
#[path = "invocation_configured.rs"]
mod configured;
#[path = "invocation_host.rs"]
mod host;
pub use configured::evaluate_tool_invocation_with_config;
pub async fn evaluate_tool_invocation(tool_name: &str, args: &Value) -> Option<ToolResult> {
    codetether_runtime_policy::evaluation::evaluate::<host::Native>(tool_name, args).await
}
pub async fn evaluate_tool_invocation_for_workspace(
    tool_name: &str,
    args: &Value,
    workspace: &Path,
) -> Option<ToolResult> {
    codetether_runtime_policy::evaluation::for_workspace::<host::Native>(tool_name, args, workspace)
        .await
}

#[cfg(test)]
#[path = "prior_context_tests.rs"]
mod prior_context_tests;
#[cfg(test)]
#[path = "invocation_tests.rs"]
mod tests;
