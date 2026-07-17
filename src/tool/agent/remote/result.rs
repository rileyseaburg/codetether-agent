//! Conversion of A2A task/message responses into first-party tool results.

use crate::a2a::types::SendMessageResponse;
use crate::tool::ToolResult;
use serde_json::json;

pub(super) fn render(name: &str, response: SendMessageResponse) -> ToolResult {
    let (text, failed) = super::text::response(&response);
    let text = match text.trim() {
        "" => "Peer completed without a text response".to_string(),
        _ => text,
    };
    let output = json!({
        "agent": name,
        "response": text,
        "transport": "a2a-mdns"
    });
    let rendered = serde_json::to_string_pretty(&output).unwrap_or(text);
    match failed {
        true => ToolResult::error(rendered),
        false => ToolResult::success(rendered),
    }
}
