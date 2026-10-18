//! Convert a tool result to a provider tool-role message.
use crate::payloads::CodexFunctionCallOutputPayload;
use anyhow::Result;
use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::Value;

pub(super) fn parse_function_output_item(payload: Value) -> Result<Option<Message>> {
    let payload: CodexFunctionCallOutputPayload = serde_json::from_value(payload)?;
    let content = super::value::value_to_string(payload.output).unwrap_or_default();
    Ok(Some(Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: payload.call_id,
            content,
        }],
    }))
}
