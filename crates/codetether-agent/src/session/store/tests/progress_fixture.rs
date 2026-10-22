//! Repeated-edit call/result fixtures and the shared persisted guard lookup.
use crate::{
    provider::{ContentPart, Message, Role},
    session::Session,
};
pub(super) fn call(id: usize) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: id.to_string(),
            name: "edit".into(),
            arguments: "{}".into(),
            thought_signature: None,
        }],
    }
}
pub(super) fn result(id: usize) -> Message {
    codetether_tool_core::messages::tool_result_with_status(
        id.to_string(),
        "edit",
        false,
        "not found".into(),
    )
}
pub(super) fn blocked(session: &Session) -> bool {
    session
        .metadata
        .progress
        .blocked_edit("edit", &serde_json::json!({}))
        .is_some()
}
