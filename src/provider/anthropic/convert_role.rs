//! Role-specific message conversion helpers.
//!
//! Anthropic's Messages API treats roles differently from the crate's generic
//! provider message model. System content is lifted out of the message list into
//! top-level system blocks, assistant content may include tool-use blocks, and
//! tool results are sent back as user-role messages. This module contains the
//! role-aware glue that chooses which individual content-block converter to use
//! for each [`Message`].
//!
//! These helpers do not reorder messages or decide whether empty messages need
//! placeholder content. That responsibility belongs to the higher-level
//! conversion module. The functions here only extract Anthropic-compatible JSON
//! blocks for a single role and append role-derived messages when tool results
//! are present.

use serde_json::Value;

use crate::provider::{ContentPart, Message};

/// Append Anthropic system blocks converted from a generic system message.
///
/// Anthropic expects system instructions as top-level content blocks rather than
/// as ordinary `"role": "system"` messages. This helper extracts text and
/// thinking blocks from `msg` and pushes their JSON representations into
/// `system_blocks`. Other content variants are ignored because they are not
/// valid Anthropic system blocks.
///
/// # Arguments
///
/// * `system_blocks` - Destination collection for converted top-level system
///   blocks.
/// * `msg` - The generic message whose content is being interpreted as system
///   content.
///
/// # Side Effects
///
/// Mutates `system_blocks` by appending one JSON block for each supported
/// content part in `msg`.
pub(crate) fn push_system(system_blocks: &mut Vec<Value>, msg: &Message) {
    for part in &msg.content {
        match part {
            ContentPart::Text { text } => system_blocks.push(super::convert_parts::text(text)),
            ContentPart::Thinking { text, .. } => {
                system_blocks.push(super::convert_parts::thinking(text));
            }
            _ => {}
        }
    }
}

/// Convert user-message content into Anthropic content blocks.
///
/// User messages can carry plain text and thinking blocks in this conversion
/// path. Tool results are intentionally excluded here because they are handled
/// separately by [`push_tool_results`], which wraps them in a user-role message
/// only when actual tool-result blocks are present.
///
/// # Arguments
///
/// * `msg` - The generic user message to convert.
///
/// # Returns
///
/// A vector of Anthropic JSON content blocks for supported user content parts.
/// Unsupported content variants are skipped.
pub(crate) fn user_parts(msg: &Message) -> Vec<Value> {
    msg.content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(super::convert_parts::text(text)),
            ContentPart::Thinking { text, .. } => Some(super::convert_parts::thinking(text)),
            _ => None,
        })
        .collect()
}

/// Convert assistant-message content into Anthropic assistant content blocks.
///
/// Assistant messages may include text, signed thinking, and tool-use requests.
/// The per-block conversion is delegated to
/// [`super::convert_parts::assistant_part`] so this role-level helper only
/// performs iteration and filtering.
///
/// # Arguments
///
/// * `msg` - The generic assistant message to convert.
///
/// # Returns
///
/// A vector of Anthropic JSON content blocks. Content variants that cannot
/// appear in an assistant message are omitted.
pub(crate) fn assistant_parts(msg: &Message) -> Vec<Value> {
    msg.content
        .iter()
        .filter_map(super::convert_parts::assistant_part)
        .collect()
}

/// Append a user-role Anthropic message containing tool results, when present.
///
/// Anthropic represents tool execution results as `"tool_result"` content
/// blocks inside a user message. This helper collects all tool-result parts from
/// `msg` and appends a converted user message to `api_messages` only if at
/// least one result exists.
///
/// # Arguments
///
/// * `api_messages` - Destination list of Anthropic API messages.
/// * `msg` - The generic tool-role message whose content may contain tool
///   results.
///
/// # Side Effects
///
/// Mutates `api_messages` by appending a `"role": "user"` message containing
/// converted tool-result blocks. If `msg` has no tool-result parts, the vector
/// is left unchanged.
pub(crate) fn push_tool_results(
    api_messages: &mut Vec<Value>,
    msg: &Message,
    known_tool_calls: &std::collections::HashSet<String>,
) {
    let results: Vec<Value> = msg
        .content
        .iter()
        .filter(|part| super::sanitize::result_has_call(part, known_tool_calls))
        .filter_map(super::convert_parts::tool_result)
        .collect();
    if !results.is_empty() {
        super::convert::push_message(api_messages, "user", results);
    }
}
