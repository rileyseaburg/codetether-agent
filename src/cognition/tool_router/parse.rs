//! Parsing FunctionGemma output into structured tool calls.

use super::parse_blocks::blocks;
use super::parsed_call::ParsedToolCall;

/// Parse FunctionGemma output into zero or more structured tool calls.
///
/// Expected format:
/// ```text
/// <tool_call>
/// {"name": "read_file", "arguments": {"path": "/tmp/foo.rs"}}
/// </tool_call>
/// ```
///
/// Handles multiple `<tool_call>` blocks in a single response. Unparseable or
/// unnamed blocks are logged and skipped.
pub(super) fn parse_functiongemma_response(text: &str) -> Vec<ParsedToolCall> {
    blocks(text).into_iter().filter_map(parse_block).collect()
}

/// Parse one JSON block into a named call, if it has a name.
fn parse_block(block: &str) -> Option<ParsedToolCall> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(block) else {
        tracing::warn!(
            block = %block,
            "FunctionGemma produced unparseable tool_call block"
        );
        return None;
    };
    let name = value.get("name").and_then(|n| n.as_str()).unwrap_or("");
    if name.is_empty() {
        return None;
    }
    Some(ParsedToolCall {
        name: name.to_string(),
        arguments: value
            .get("arguments")
            .map(|a| serde_json::to_string(a).unwrap_or_else(|_| "{}".to_string()))
            .unwrap_or_else(|| "{}".to_string()),
    })
}
