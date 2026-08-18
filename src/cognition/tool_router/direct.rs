//! Direct `<tool_call>` extraction from assistant text.

use super::parse;
use super::parsed_call::ParsedToolCall;
use crate::provider::ToolDefinition;

/// Parse tool calls already present in the text, keeping only known tools.
///
/// This is zero-cost and succeeds when the system prompt already instructed the
/// model to emit structured tool calls.
pub(super) fn direct_calls(assistant_text: &str, tools: &[ToolDefinition]) -> Vec<ParsedToolCall> {
    parse::parse_functiongemma_response(assistant_text)
        .into_iter()
        .filter(|c| tools.iter().any(|t| t.name == c.name))
        .collect()
}
