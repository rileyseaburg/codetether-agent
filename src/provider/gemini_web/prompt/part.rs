//! Gemini Web content-part protocol encoding.

use crate::provider::ContentPart;
use serde_json::{Value, json};

/// Encode one history part without teaching the model fake transcript syntax.
pub(super) fn render(part: &ContentPart) -> Option<String> {
    match part {
        ContentPart::Text { text } => Some(text.clone()),
        ContentPart::ToolCall {
            name, arguments, ..
        } => Some(render_call(name, arguments)),
        ContentPart::ToolResult {
            tool_call_id,
            content,
        } => Some(render_result(tool_call_id, content)),
        ContentPart::Thinking { text, .. } => Some(format!("<thinking>{text}</thinking>")),
        ContentPart::Image { .. } | ContentPart::File { .. } => None,
    }
}

fn render_call(name: &str, arguments: &str) -> String {
    let arguments = serde_json::from_str::<Value>(arguments)
        .unwrap_or_else(|_| Value::String(arguments.to_string()));
    let payload = safe_json(&json!({"name": name, "arguments": arguments}));
    format!("<tool_call>{payload}</tool_call>")
}

fn render_result(tool_call_id: &str, content: &str) -> String {
    let payload = safe_json(&json!({"tool_call_id": tool_call_id, "content": content}));
    format!("<tool_result>{payload}</tool_result>")
}

fn safe_json(value: &Value) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "{}".to_string())
        .replace("</", "<\\/")
}
