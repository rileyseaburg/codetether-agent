//! Associate image siblings with their preceding, known Anthropic tool result.
use crate::provider::{ContentPart, Message};
use serde_json::Value;
use std::collections::HashSet;

/// Append known tool results as a user turn, nesting each following image.
///
/// # Arguments
///
/// * `api_messages` - Destination native messages.
/// * `msg` - Tool message with results followed by their image siblings.
/// * `known_tool_calls` - Assistant-declared IDs allowed by sanitation.
///
/// # Returns
/// Nothing; orphan results and their images never enter the destination.
pub(crate) fn push_tool_results(
    api_messages: &mut Vec<Value>,
    msg: &Message,
    known_tool_calls: &HashSet<String>,
) {
    let mut results = Vec::new();
    let mut current: Option<Value> = None;
    for part in &msg.content {
        match part {
            ContentPart::ToolResult { .. } => {
                if let Some(result) = current.take() {
                    results.push(result);
                }
                if super::super::sanitize::result_has_call(part, known_tool_calls) {
                    current = super::super::convert_parts::tool_result(part);
                }
            }
            ContentPart::Image { url, mime_type } => {
                if let Some(result) = current.as_mut() {
                    if let Some(text) = result["content"].as_str() {
                        let blocks = if text.is_empty() {
                            Vec::new()
                        } else {
                            vec![super::super::convert_parts::text(text)]
                        };
                        result["content"] = Value::Array(blocks);
                    }
                    if let Some(blocks) = result["content"].as_array_mut() {
                        blocks.push(super::image::block(url, mime_type.as_deref()));
                    }
                }
            }
            _ => {}
        }
    }
    if let Some(result) = current {
        results.push(result);
    }
    if !results.is_empty() {
        super::super::convert::push_message(api_messages, "user", results);
    }
}
