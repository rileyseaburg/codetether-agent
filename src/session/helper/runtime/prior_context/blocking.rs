//! Runtime rejection of forbidden prior-context tool calls.

#[cfg(test)]
use crate::provider::Message;
use crate::session::Session;
use crate::tool::ToolResult;
use serde_json::Value;

const MESSAGE: &str = "The user disabled prior memory/session/history access. Do not retry this tool or substitute another prior-context source. Use the active conversation and user-designated repository files or documentation. Only a later explicit opt-in restores access.";

/// Block a prior-context request using transcript-derived policy.
#[cfg(test)]
pub(crate) fn for_messages(messages: &[Message], name: &str, args: &Value) -> Option<ToolResult> {
    blocked(!super::allowed(messages), name, args)
}

/// Parse and block serialized arguments using transcript-derived policy.
#[cfg(test)]
pub(crate) fn serialized_messages(
    messages: &[Message],
    name: &str,
    args: &str,
) -> Option<ToolResult> {
    for_messages(messages, name, &parse(args))
}

/// Parse and block serialized arguments using durable session policy.
pub(crate) fn serialized_session(session: &Session, name: &str, args: &str) -> Option<ToolResult> {
    blocked(!super::session_state::allowed(session), name, &parse(args))
}

/// Block using the authoritative ceiling embedded by a session runtime.
pub(crate) fn runtime_context(name: &str, args: &Value) -> Option<ToolResult> {
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
