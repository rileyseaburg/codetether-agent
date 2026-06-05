//! Message conversion into Anthropic Messages API format.
//!
//! This module translates CodeTether's provider-neutral [`Message`] values into
//! the JSON shape expected by Anthropic's Messages API. System messages are
//! collected separately because Anthropic accepts them through the top-level
//! `system` field, while user, assistant, and tool-result messages are emitted
//! as entries in the ordered `messages` array.
//!
//! Conversion preserves input ordering for non-system conversation turns and can
//! optionally annotate eligible blocks for Anthropic prompt caching.

use serde_json::{Value, json};

use crate::provider::{Message, Role};

/// Convert generic provider messages into Anthropic system and message blocks.
///
/// System-role messages are converted into Anthropic system blocks and returned
/// separately from the conversation message list. User and assistant messages
/// are converted into Anthropic content arrays using the role-specific
/// conversion helpers, and tool-role messages are converted into user-facing
/// tool result blocks so they can be sent back to Anthropic after tool calls.
///
/// When `enable_cache` is `true`, cache-control metadata is applied to the
/// converted system and message blocks before they are returned.
///
/// # Arguments
///
/// * `messages` - Provider-neutral messages in conversation order.
/// * `enable_cache` - Whether to add Anthropic cache-control metadata to
///   eligible converted blocks.
///
/// # Returns
///
/// A tuple containing:
/// * `Some(system_blocks)` when at least one system block was produced, or
///   `None` when there are no system messages.
/// * The Anthropic `messages` array containing converted user, assistant, and
///   tool-result turns.
///
/// # Side Effects
///
/// This function does not mutate the input slice. It allocates new JSON values
/// for the Anthropic request payload.
pub(crate) fn messages(
    messages: &[Message],
    enable_cache: bool,
) -> (Option<Vec<Value>>, Vec<Value>) {
    let mut system_blocks = Vec::new();
    let mut api_messages = Vec::new();
    for msg in messages {
        match msg.role {
            Role::System => super::convert_role::push_system(&mut system_blocks, msg),
            Role::User => push_message(
                &mut api_messages,
                "user",
                super::convert_role::user_parts(msg),
            ),
            Role::Assistant => push_message(
                &mut api_messages,
                "assistant",
                super::convert_role::assistant_parts(msg),
            ),
            Role::Tool => super::convert_role::push_tool_results(&mut api_messages, msg),
        }
    }
    if enable_cache {
        super::cache::apply_to_messages(&mut system_blocks, &mut api_messages);
    }
    (optional(system_blocks), api_messages)
}

/// Append one Anthropic message object to the converted message list.
///
/// Anthropic does not accept an empty `content` array. If the converted content
/// parts are empty, this helper inserts a single blank text block so the message
/// remains valid while preserving the conversation turn and role.
///
/// # Arguments
///
/// * `api_messages` - Destination Anthropic `messages` array to append to.
/// * `role` - Anthropic message role, typically `"user"` or `"assistant"`.
/// * `parts` - Anthropic content blocks for the message.
///
/// # Side Effects
///
/// Mutates `api_messages` by pushing one JSON object with `role` and `content`
/// fields.
pub(crate) fn push_message(api_messages: &mut Vec<Value>, role: &str, mut parts: Vec<Value>) {
    if parts.is_empty() {
        parts.push(json!({"type": "text", "text": " "}));
    }
    api_messages.push(json!({"role": role, "content": parts}));
}

/// Return system blocks only when Anthropic's top-level system field is needed.
///
/// Anthropic requests can omit the `system` field entirely when there are no
/// system blocks. This helper converts an empty vector into `None` and preserves
/// non-empty vectors as `Some`.
///
/// # Arguments
///
/// * `blocks` - Converted Anthropic system content blocks.
///
/// # Returns
///
/// `Some(blocks)` when `blocks` is non-empty; otherwise `None`.
fn optional(blocks: Vec<Value>) -> Option<Vec<Value>> {
    (!blocks.is_empty()).then_some(blocks)
}
