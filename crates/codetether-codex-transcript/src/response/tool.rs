use anyhow::Result;
use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::Value;

use crate::payloads::CodexFunctionCallPayload;

pub(super) fn parse_function_call_item(payload: Value) -> Result<Option<Message>> {
    let payload: CodexFunctionCallPayload = serde_json::from_value(payload)?;
    let Some(id) = payload.call_id.or(payload.id) else {
        return Ok(None);
    };
    let Some(arguments) = super::value::value_to_string(payload.arguments) else {
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
