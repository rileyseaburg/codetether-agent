//! Fail-closed lease acquisition immediately before a mutating tool call.

use serde_json::Value;

/// Gate an invocation when an inherited coordinator is active.
/// # Arguments
/// `host` supplies mux access; `tool` and `input` describe the proposed operation.
/// # Returns
/// A structured blocking result, or None when coordination permits continuation.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_workspace_coordination::Host) {
/// let _block = codetether_workspace_coordination::blocked(host, "read", &serde_json::json!({})).await;
/// # }
/// ```
pub async fn blocked(
    host: &impl crate::Host,
    tool: &str,
    input: &Value,
) -> Option<codetether_tool_reporting::ToolTuple> {
    if !host.active() {
        return None;
    }
    blocked_in_mux(host, tool, input).await
}

/// Gate an invocation whose coordinator activity has already been established.
/// # Arguments
/// `host` acquires leases; `tool` and `input` supply mutation paths and trusted context.
/// # Returns
/// None for an unscoped call or acquired lease, otherwise a structured failure.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_workspace_coordination::Host) {
/// let _block = codetether_workspace_coordination::blocked_in_mux(host, "read", &serde_json::json!({})).await;
/// # }
/// ```
pub async fn blocked_in_mux(
    host: &impl crate::Host,
    tool: &str,
    input: &Value,
) -> Option<codetether_tool_reporting::ToolTuple> {
    let paths = super::paths::mutation_paths(tool, input)?;
    let context = match super::context::RuntimeContext::from_input(input) {
        Ok(context) => context,
        Err(error) => return Some(super::gate_failure::unavailable(tool, &error)),
    };
    let scope = super::scope::resolve(&context.workspace, paths);
    let scope = super::scope::scoped_only(scope)?;
    let reply = host
        .acquire(
            &context.owner,
            &context.agent,
            &scope.workspace,
            scope.paths,
        )
        .await;
    match reply {
        Ok(Some(reply)) => super::outcome::classify(tool, &context.agent, reply),
        Ok(None) => Some(super::gate_failure::missing(tool)),
        Err(error) => Some(super::gate_failure::unavailable(tool, &error)),
    }
}
