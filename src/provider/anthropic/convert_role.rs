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
pub(super) mod image;
#[cfg(test)]
mod image_test_support;
#[cfg(test)]
mod image_tests;
mod tool;
pub(crate) use tool::push_tool_results;

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
/// User messages can carry images, plain text and thinking blocks in this conversion
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
            ContentPart::Image { url, mime_type } => Some(image::block(url, mime_type.as_deref())),
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
