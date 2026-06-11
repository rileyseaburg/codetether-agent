use super::{RuntimeToolPolicy, ToolPolicyOutcome};
use crate::config::Config;
use crate::tool::ToolResult;
use serde_json::Value;
use std::path::Path;

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

pub fn evaluate_tool_invocation_with_config(
    config: &Config,
    tool_name: &str,
    args: &Value,
) -> Option<ToolResult> {
    let policy = RuntimeToolPolicy::from_config(config);
    let decision = super::invocation_decision::decide(&policy, tool_name, args);
    let scope = super::invocation_scope::for_tool(tool_name, args);
    if matches!(decision.outcome, ToolPolicyOutcome::RequireApproval)
        && super::approval_gate::allowed(tool_name, args, scope.action, &scope.resource)
    {
        return None;
    }
    super::result::blocking_result_with_approval_request_for_args(
        tool_name,
        decision,
        scope.action,
        &scope.resource,
        args,
    )
}

#[cfg(test)]
#[path = "invocation_tests.rs"]
mod tests;
