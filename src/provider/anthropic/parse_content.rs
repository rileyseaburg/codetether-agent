//! Content-block conversion for Anthropic responses.
//!
//! This module translates Anthropic Messages API response content blocks into
//! the provider-neutral [`ContentPart`] representation used by the rest of the
//! application. It preserves visible assistant text, reasoning blocks, and tool
//! calls while dropping empty text blocks and unknown content variants.

use crate::provider::ContentPart;

use super::response::AnthropicContent;

/// Convert one Anthropic response content block into an internal content part.
///
/// Text blocks become [`ContentPart::Text`] only when they contain non-empty
/// text. Thinking blocks are normalized by [`thinking_part`]. Tool-use blocks
/// become [`ContentPart::ToolCall`] values and set `has_tool_calls` to `true`
/// so the caller can distinguish ordinary assistant text from responses that
/// require tool execution.
///
/// # Parameters
///
/// * `part` - The Anthropic response content block to convert.
/// * `has_tool_calls` - Mutable flag updated to `true` when `part` is a tool
///   call. Existing `true` values are preserved.
///
/// # Returns
///
/// `Some(ContentPart)` when the block contains supported, non-empty content;
/// `None` for empty text blocks or unknown content variants.
///
/// # Side Effects
///
/// Mutates `has_tool_calls` when a tool-use block is encountered. The function
/// does not perform I/O or modify the input content block.
pub(crate) fn content_part(
    part: &AnthropicContent,
    has_tool_calls: &mut bool,
) -> Option<ContentPart> {
    match part {
        AnthropicContent::Text { text } if !text.is_empty() => {
            Some(ContentPart::Text { text: text.clone() })
        }
        AnthropicContent::Thinking {
            thinking,
            text,
            signature,
        } => thinking_part(thinking, text, signature),
        AnthropicContent::ToolUse { id, name, input } => {
            *has_tool_calls = true;
            Some(ContentPart::ToolCall {
                id: id.clone(),
                name: name.clone(),
                arguments: serde_json::to_string(input).unwrap_or_default(),
                thought_signature: None,
            })
        }
        AnthropicContent::Text { .. } | AnthropicContent::Unknown => None,
    }
}

/// Convert Anthropic reasoning fields into a provider-neutral thinking part.
///
/// Anthropic-compatible providers may populate either `thinking` or `text` for
/// reasoning content. This helper prefers `thinking`, falls back to `text`,
/// trims surrounding whitespace, and preserves the optional provider signature
/// used to authenticate or continue extended thinking flows.
///
/// # Parameters
///
/// * `thinking` - Primary reasoning text supplied by the provider.
/// * `text` - Fallback reasoning text used when `thinking` is absent.
/// * `signature` - Optional provider signature associated with the reasoning
///   block.
///
/// # Returns
///
/// `Some(ContentPart::Thinking)` when the normalized reasoning text is not
/// empty; otherwise `None`.
fn thinking_part(
    thinking: &Option<String>,
    text: &Option<String>,
    signature: &Option<String>,
) -> Option<ContentPart> {
    let reasoning = thinking
        .as_deref()
        .or(text.as_deref())
        .unwrap_or_default()
        .trim()
        .to_string();
    (!reasoning.is_empty()).then(|| ContentPart::Thinking {
        text: reasoning,
        signature: signature.clone(),
    })
}
