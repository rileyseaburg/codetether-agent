use crate::provider::{ContentPart, Message, Role};
use anyhow::Result;
use serde_json::Value;

use crate::session::codex_import::payloads::{
    CodexFunctionCallOutputPayload, CodexFunctionCallPayload,
};

pub(super) fn parse_function_call_item(payload: Value) -> Result<Option<Message>> {
    let payload: CodexFunctionCallPayload = serde_json::from_value(payload)?;
    let Some(id) = payload.call_id.or(payload.id) else {
        return Ok(None);
    };
    let Some(arguments) = value_to_string(payload.arguments) else {
        return Ok(None);
    };
    Ok(Some(Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id,
            name: payload.name,
            arguments,
            thought_signature: None,
        }],
    }))
}

pub(super) fn parse_function_output_item(payload: Value) -> Result<Option<Message>> {
    let payload: CodexFunctionCallOutputPayload = serde_json::from_value(payload)?;
    let content = value_to_string(payload.output).unwrap_or_default();
    Ok(Some(Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: payload.call_id,
            content,
        }],
    }))
}

fn value_to_string(value: Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(text) => Some(text),
        other => Some(other.to_string()),
    }
}
