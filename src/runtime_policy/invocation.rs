use crate::config::Config;
use crate::tool::ToolResult;
use serde_json::Value;
use std::path::Path;

#[path = "invocation_configured.rs"]
mod configured;
pub use configured::evaluate_tool_invocation_with_config;

pub async fn evaluate_tool_invocation(tool_name: &str, args: &Value) -> Option<ToolResult> {
    let config = match super::workspace::from_args(args) {
        Some(path) => Config::load_for_workspace(path).await,
        None => Config::load().await,
    }
    .unwrap_or_default();
    evaluate_tool_invocation_with_config(&config, tool_name, args)
}

pub async fn evaluate_tool_invocation_for_workspace(
    tool_name: &str,
    args: &Value,
    workspace: &Path,
) -> Option<ToolResult> {
    let config = Config::load_for_workspace(workspace)
        .await
        .unwrap_or_default();
    evaluate_tool_invocation_with_config(&config, tool_name, args)
}

#[cfg(test)]
#[path = "prior_context_tests.rs"]
mod prior_context_tests;
#[cfg(test)]
#[path = "invocation_tests.rs"]
mod tests;
