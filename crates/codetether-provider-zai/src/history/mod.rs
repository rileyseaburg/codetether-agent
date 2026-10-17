//! History serialization shared by application and injected-client adapters.
//!
//! Use [HistoryFormat] to preserve the caller's text/tool-reply conventions.
use crate::{Message, Role};
use serde_json::Value;
mod assistant;
mod calls;
mod format;
mod ordinary;
mod text;
mod tools;
pub use format::HistoryFormat;
#[cfg(test)]
mod tests;

/// Serialize conversation history using explicit wire conventions.
///
/// # Arguments
/// * `messages` - Conversation in stored order.
/// * `include_reasoning` - Whether to include historical assistant reasoning.
/// * `format` - Existing adapter convention to retain.
/// * `normalize_arguments` - Caller-owned tool argument normalization.
///
/// # Returns
/// Serialized messages following the selected format.
///
/// # Examples
/// ```
/// use codetether_provider_zai::history::{convert, HistoryFormat};
/// assert!(convert(&[], false, HistoryFormat::SingleReply, str::to_owned).is_empty());
/// ```
pub fn convert(
    messages: &[Message],
    include_reasoning: bool,
    format: HistoryFormat,
    normalize_arguments: impl Fn(&str) -> String,
) -> Vec<Value> {
    messages
        .iter()
        .flat_map(|message| match message.role {
            Role::Assistant => vec![assistant::convert(
                message,
                include_reasoning,
                &normalize_arguments,
            )],
            Role::Tool => tools::convert(message, format),
            Role::System | Role::Developer | Role::User => vec![ordinary::convert(message, format)],
        })
        .collect()
}
