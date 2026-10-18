//! Registry lookup, policy gating and invalid-tool routing.
use crate::*;
/// Executes a tool from an already parsed JSON value.
///
/// # Examples
///
/// ```ignore
/// let result = agent.execute_tool_value("bash", serde_json::json!({"command":"pwd"})).await;
/// ```
/// Execute a parsed tool input through registry lookup and native policy.
/// # Arguments
/// Tool host, registered name and parsed arguments.
/// # Returns
/// The registered-tool, policy-denial or invalid-tool result.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_tools::*;
/// # async fn example<H: Host>(host: &H, session: &H::Session) {
/// execute_value(host, "tool", serde_json::json!({})).await;
/// # }
/// ```
pub async fn execute_value<H: Host>(host: &H, name: &str, arguments: Value) -> ToolResult {
    log_tool_permission(host, name);
    match host.tool(name) {
        Some(tool) => match H::blocked(name, &arguments).await {
            Some(blocked) => blocked,
            None => codetether_agent_core::execution::registered::execute(tool, arguments).await,
        },
        None => crate::invalid::execute(host, name, arguments).await,
    }
}

fn log_tool_permission<H: Host>(host: &H, name: &str) {
    if let Some(permission) = host.permission(name) {
        tracing::debug!(target: "codetether_agent::agent::tooling::dispatch",agent = %host.name(), tool = %name, permission = ?permission, "Checking tool permission");
    }
}
