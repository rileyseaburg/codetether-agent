//! Managed-agent tool request construction and dispatch.

use std::path::Path;

use serde_json::{Value, json};

use codetether_tool_core::ToolResult;
use std::future::Future;
/// Native executor of canonical agent-tool requests.
/// # Implementors
/// Forward requests to the application agent tool without changing its policy.
pub trait Host {
    /// Execute a request with the existing tool.
    /// # Errors
    /// Returns the native tool execution error.
    fn execute(params: Value) -> impl Future<Output = anyhow::Result<ToolResult>> + Send;
}

async fn execute<H: Host>(params: Value) -> ToolResult {
    H::execute(params)
        .await
        .unwrap_or_else(|error| ToolResult::error(error.to_string()))
}

/// Build and execute an asynchronous child-spawn request.
/// # Arguments
/// Receives parent ID, optional model, workspace, child name and instructions.
/// # Returns
/// Tool output, or a model-selection error before dispatch.
/// # Examples
/// ```no_run
/// async fn spawn<H: codetether_agent_command::request::Host>() {
///     codetether_agent_command::request::spawn::<H>("parent", Some("model"), std::path::Path::new("."), "worker", "inspect").await;
/// }
/// ```
pub async fn spawn<H: Host>(
    session_id: &str,
    model: Option<&str>,
    cwd: &Path,
    name: &str,
    instructions: &str,
) -> ToolResult {
    let Some(model) = model else {
        return ToolResult::error("Select a model before spawning a sub-agent");
    };
    execute::<H>(json!({
        "action": "spawn", "name": name, "instructions": instructions,
        "model": model, "detach": true, "__ct_session_id": session_id,
        "__ct_parent_workspace": cwd,
    }))
    .await
}

/// Dispatch a detached child message carrying both parent context fields.
/// # Arguments
/// Receives the parent ID, child name and message text.
/// # Returns
/// The native tool result, converting execution errors to error output.
/// # Examples
/// ```no_run
/// async fn message<H: codetether_agent_command::request::Host>() {
///     codetether_agent_command::request::message::<H>("parent", "worker", "inspect").await;
/// }
/// ```
pub async fn message<H: Host>(parent: &str, name: &str, message: &str) -> ToolResult {
    execute::<H>(json!({
        "action": "message", "name": name, "message": message, "detach": true,
        "context_id": parent, "__ct_session_id": parent,
    }))
    .await
}

/// Dispatch a parent-scoped child removal request.
/// # Arguments
/// Receives the parent ID and child name.
/// # Returns
/// The native tool result, converting execution errors to error output.
/// # Examples
/// ```no_run
/// async fn kill<H: codetether_agent_command::request::Host>() {
///     codetether_agent_command::request::kill::<H>("parent", "worker").await;
/// }
/// ```
pub async fn kill<H: Host>(parent: &str, name: &str) -> ToolResult {
    execute::<H>(json!({ "action": "kill", "name": name, "__ct_session_id": parent })).await
}
