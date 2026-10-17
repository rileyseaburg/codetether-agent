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
///
/// # Arguments
/// * `text` - Raw model text containing closed tool-call blocks.
/// * `warn` - Host callback for each malformed JSON block.
///
/// # Returns
/// Named calls in block order, skipping malformed and unnamed blocks.
///
/// # Examples
/// ```
/// use codetether_cognition_core::tool_routing::parse_functiongemma_response;
/// let calls = parse_functiongemma_response(r#"<tool_call>{"name":"read","arguments":{}}</tool_call>"#, |_| {});
/// assert_eq!(calls[0].name, "read");
/// ```
pub fn parse_functiongemma_response(text: &str, mut warn: impl FnMut(&str)) -> Vec<ParsedToolCall> {
    blocks(text)
        .into_iter()
        .filter_map(|block| parse_block(block, &mut warn))
        .collect()
}

/// Parse one JSON block into a named call, if it has a name.
fn parse_block(block: &str, warn: &mut impl FnMut(&str)) -> Option<ParsedToolCall> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(block) else {
        warn(block);
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
