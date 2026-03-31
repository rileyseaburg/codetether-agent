mod message;
mod tool;

use super::payloads::CodexReasoningPayload;
use super::title::extract_reasoning_text;
use crate::provider::{ContentPart, Message, Role};
use anyhow::Result;
use serde_json::Value;

pub(crate) fn parse_response_item(
    payload: Value,
    first_user_text: &mut Option<String>,
) -> Result<Option<Message>> {
    let Some(item_type) = payload.get("type").and_then(Value::as_str) else {
        return Ok(None);
    };

    match item_type {
        "message" => message::parse_message_item(payload, first_user_text),
        "function_call" => tool::parse_function_call_item(payload),
        "function_call_output" => tool::parse_function_output_item(payload),
        "reasoning" => parse_reasoning_item(payload),
        _ => Ok(None),
    }
}

fn parse_reasoning_item(payload: Value) -> Result<Option<Message>> {
    let payload: CodexReasoningPayload = serde_json::from_value(payload)?;
    Ok(extract_reasoning_text(&payload).map(|text| Message {
        role: Role::Assistant,
        content: vec![ContentPart::Thinking { text }],
    }))
}
