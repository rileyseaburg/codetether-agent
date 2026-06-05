//! Prompt-cache annotations for Anthropic-compatible request blocks.
//!
//! Anthropic-compatible Messages API providers can accept a `cache_control`
//! marker on selected request blocks. This module centralizes the JSON mutation
//! used to mark the final reusable prompt segments as ephemeral-cacheable.
//!
//! The helpers operate directly on `serde_json::Value` blocks that have already
//! been converted into Anthropic request shape. They intentionally ignore values
//! that are not JSON objects or do not contain message content arrays, because
//! cache annotations are optional request hints rather than required structure.

use serde_json::{Value, json};

/// Mark a content or tool block as ephemeral-cacheable.
///
/// Adds a `cache_control` object with Anthropic's `"ephemeral"` cache type to
/// the provided JSON object. If `block` is not a JSON object, the function does
/// nothing.
///
/// # Arguments
///
/// * `block` — A mutable Anthropic request block, typically a text, tool, or
///   system block represented as a JSON object.
///
/// # Side effects
///
/// Mutates `block` in place by inserting or replacing its `cache_control`
/// field when the value is an object.
pub(crate) fn add_ephemeral_cache_control(block: &mut Value) {
    if let Some(obj) = block.as_object_mut() {
        obj.insert("cache_control".to_string(), json!({ "type": "ephemeral" }));
    }
}

/// Apply cache annotations to the last system block and last message block.
///
/// Anthropic prompt caching is most useful on the most recent stable prompt
/// boundaries. This helper marks the final content part from the latest API
/// message and the final system block, when those blocks exist.
///
/// # Arguments
///
/// * `system_blocks` — Mutable system prompt blocks converted for the
///   Anthropic `system` field.
/// * `api_messages` — Mutable Anthropic message objects whose `content` fields
///   may contain text, tool-use, or tool-result blocks.
///
/// # Side effects
///
/// Mutates the last eligible system block and message content block in place by
/// inserting Anthropic `cache_control` metadata. Empty slices and malformed
/// message values are ignored.
pub(crate) fn apply_to_messages(system_blocks: &mut [Value], api_messages: &mut [Value]) {
    if let Some(last_block) = api_messages.iter_mut().rev().find_map(last_content_part) {
        add_ephemeral_cache_control(last_block);
    }
    if let Some(last_system) = system_blocks.last_mut() {
        add_ephemeral_cache_control(last_system);
    }
}

/// Return the final content part from an Anthropic message value.
///
/// The expected input shape is a JSON object with a `content` array, matching
/// the Messages API request format. Values without that shape return `None`.
///
/// # Arguments
///
/// * `message` — A mutable JSON message object to inspect.
///
/// # Returns
///
/// A mutable reference to the last element of the message's `content` array, or
/// `None` when the message has no content array or the array is empty.
fn last_content_part(message: &mut Value) -> Option<&mut Value> {
    message
        .get_mut("content")
        .and_then(Value::as_array_mut)
        .and_then(|parts| parts.last_mut())
}
