//! Conversion helpers for individual Anthropic message content blocks.
//!
//! The Anthropic Messages API represents assistant output, user input, tool
//! calls, and tool results as typed JSON content blocks. This module converts
//! the crate's provider-neutral [`ContentPart`] values into those Anthropic
//! block shapes one block at a time.
//!
//! Higher-level message conversion code is responsible for choosing the
//! surrounding message role and preserving message order. The helpers here only
//! produce the JSON object for a single content block, or return `None` when a
//! generic content part has no valid Anthropic representation in the current
//! conversion context.

use serde_json::{Value, json};

use crate::provider::ContentPart;

/// Build an Anthropic text content block.
///
/// # Arguments
///
/// * `text` - The UTF-8 text payload to include in the block.
///
/// # Returns
///
/// A JSON object with `"type": "text"` and the provided text. The function does
/// not trim or otherwise normalize the text, so callers can preserve exact
/// message content when required.
pub(crate) fn text(text: &str) -> Value {
    json!({"type": "text", "text": text})
}

/// Build an Anthropic thinking content block.
///
/// # Arguments
///
/// * `text` - The model reasoning text to place in the `"thinking"` field.
///
/// # Returns
///
/// A JSON object with `"type": "thinking"`. This helper does not attach a
/// signature; assistant thinking blocks that may need a signature are handled by
/// [`thinking_with_signature`].
pub(crate) fn thinking(text: &str) -> Value {
    json!({"type": "thinking", "thinking": text})
}

/// Convert an assistant-side [`ContentPart`] into an Anthropic content block.
///
/// Assistant messages may contain text, thinking blocks, or tool calls. Tool
/// results are intentionally ignored here because Anthropic expects them in a
/// user-role message as `"tool_result"` blocks.
///
/// # Arguments
///
/// * `part` - The provider-neutral content part from an assistant message.
///
/// # Returns
///
/// `Some(Value)` when the part can be represented in an Anthropic assistant
/// message, or `None` for content variants that are not valid assistant blocks.
pub(crate) fn assistant_part(part: &ContentPart) -> Option<Value> {
    match part {
        ContentPart::Text { text } => Some(text_block(text)),
        ContentPart::Thinking { text, signature } => Some(thinking_with_signature(text, signature)),
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            ..
        } => Some(tool_use(id, name, arguments)),
        _ => None,
    }
}

/// Convert a tool-result [`ContentPart`] into an Anthropic `"tool_result"` block.
///
/// Anthropic requires tool execution results to be sent as content blocks inside
/// a user-role message, keyed by the original tool-use identifier. Non-tool
/// result content is ignored by returning `None`.
///
/// # Arguments
///
/// * `part` - The provider-neutral content part to inspect.
///
/// # Returns
///
/// `Some(Value)` containing `"type": "tool_result"`, `"tool_use_id"`, and
/// `"content"` for tool results, or `None` for all other content variants.
pub(crate) fn tool_result(part: &ContentPart) -> Option<Value> {
    if let ContentPart::ToolResult {
        tool_call_id,
        content,
    } = part
    {
        return Some(json!({
            "type": "tool_result",
            "tool_use_id": tool_call_id,
            "content": content
        }));
    }
    None
}

/// Build a text block for assistant-message conversion.
///
/// This private helper keeps assistant conversion readable while producing the
/// same Anthropic JSON shape as [`text`].
fn text_block(text: &str) -> Value {
    json!({"type": "text", "text": text})
}

/// Build a thinking block and attach Anthropic's signature when available.
///
/// # Arguments
///
/// * `text` - The reasoning text to include in the block.
/// * `signature` - Optional Anthropic thinking signature to round-trip.
///
/// # Returns
///
/// A thinking JSON object. When `signature` is `Some`, the returned object also
/// includes a `"signature"` field so future requests can preserve signed
/// thinking state.
fn thinking_with_signature(text: &str, signature: &Option<String>) -> Value {
    let mut block = thinking(text);
    if let Some(sig) = signature {
        block["signature"] = json!(sig);
    }
    block
}

/// Build an Anthropic `"tool_use"` block from a generic tool call.
///
/// # Arguments
///
/// * `id` - Provider-level identifier for the tool call.
/// * `name` - Tool name requested by the assistant.
/// * `arguments` - JSON string containing the tool input arguments.
///
/// # Returns
///
/// A JSON object with `"type": "tool_use"`. If `arguments` is valid JSON, it is
/// decoded into the `"input"` field. If parsing fails, the original argument
/// string is preserved under `"input": {"raw": ...}` so malformed arguments are
/// not silently discarded.
fn tool_use(id: &str, name: &str, arguments: &str) -> Value {
    let input = serde_json::from_str(arguments).unwrap_or_else(|_| json!({"raw": arguments}));
    json!({"type": "tool_use", "id": id, "name": name, "input": input})
}
