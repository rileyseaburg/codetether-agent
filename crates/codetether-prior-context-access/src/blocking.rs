//! Runtime rejection of forbidden prior-context tool calls.

use codetether_provider_core::Message;
use codetether_tool_core::ToolResult;
use serde_json::Value;

const MESSAGE: &str = "The user disabled prior memory/session/history access. Do not retry this tool or substitute another prior-context source. Use the active conversation and user-designated repository files or documentation. Only a later explicit opt-in restores access.";

/// Block a prior-context request using transcript-derived policy.
/// # Arguments
/// Receives transcript messages, a tool name and decoded arguments.
/// # Returns
/// A structured rejection when access is denied, otherwise None.
/// # Examples
/// ```
/// assert!(codetether_prior_context_access::for_messages(&[], "read", &serde_json::json!({})).is_none());
/// ```
pub fn for_messages(messages: &[Message], name: &str, args: &Value) -> Option<ToolResult> {
    blocked(
        !codetether_prior_context_policy::resolve(messages),
        name,
        args,
    )
}

/// Parse and block serialized arguments using transcript-derived policy.
/// # Arguments
/// Receives transcript messages, a tool name and serialized arguments.
/// # Returns
/// A structured rejection when access is denied, otherwise None.
/// # Examples
/// ```
/// assert!(codetether_prior_context_access::serialized_messages(&[], "read", "{}").is_none());
/// ```
pub fn serialized_messages(messages: &[Message], name: &str, args: &str) -> Option<ToolResult> {
    for_messages(messages, name, &parse(args))
}

/// Parse and block serialized arguments using durable session policy.
/// # Arguments
/// Receives effective permission, a tool name and serialized arguments.
/// # Returns
/// A structured rejection when access is denied, otherwise None.
/// # Examples
/// ```
/// assert!(codetether_prior_context_access::serialized_policy(false, "session_recall", "{}").is_some());
/// ```
pub fn serialized_policy(allowed: bool, name: &str, args: &str) -> Option<ToolResult> {
    blocked(!allowed, name, &parse(args))
}

/// Block using the authoritative ceiling embedded by a session runtime.
/// # Arguments
/// Receives a tool name and arguments carrying the authoritative runtime ceiling.
/// # Returns
/// A structured rejection when access is denied, otherwise None.
/// # Examples
/// ```
/// let args = serde_json::json!({"__ct_prior_context_allowed": false});
/// assert!(codetether_prior_context_access::runtime_context("session_recall", &args).is_some());
/// ```
pub fn runtime_context(name: &str, args: &Value) -> Option<ToolResult> {
    let denied = args
        .get("__ct_prior_context_allowed")
        .and_then(Value::as_bool)
        == Some(false);
    blocked(denied, name, args)
}

fn blocked(denied: bool, name: &str, args: &Value) -> Option<ToolResult> {
    (denied && super::call::requests_prior_context(name, args)).then(|| {
        ToolResult::structured_error("PRIOR_CONTEXT_DISABLED_BY_USER", name, MESSAGE, None, None)
    })
}

fn parse(args: &str) -> Value {
    serde_json::from_str(args).unwrap_or(Value::Null)
}
