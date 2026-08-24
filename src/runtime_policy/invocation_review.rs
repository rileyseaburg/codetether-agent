//! Side-effect-free invocation review used before live approval and preflight.

use crate::config::Config;
use crate::tool::ToolResult;
use serde_json::Value;

pub(crate) async fn evaluate(tool_name: &str, args: &Value) -> Option<ToolResult> {
    let config = match super::super::workspace::from_args(args) {
        Some(path) => Config::load_for_workspace(path).await,
        None => Config::load().await,
    }
    .unwrap_or_default();
    let policy = super::super::RuntimeToolPolicy::from_config(&config);
    let decision = super::super::invocation_decision::decide(&policy, tool_name, args);
    let scope = super::super::invocation_scope::for_tool(tool_name, args);
    if matches!(
        decision.outcome,
        super::super::ToolPolicyOutcome::RequireApproval
    ) && super::super::approval_gate::review_allowed(
        tool_name,
        args,
        scope.action,
        &scope.resource,
    ) {
        return None;
    }
    super::super::result::blocking_result_with_approval_request_for_args(
        tool_name,
        decision,
        scope.action,
        &scope.resource,
        args,
    )
}
