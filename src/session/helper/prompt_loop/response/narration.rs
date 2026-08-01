//! Suppression of pre-tool narration from the accumulated final answer.
//!
//! Text emitted in the same assistant turn as a tool call is a preamble, not an
//! answer. GLM's published chat template makes this explicit: a turn may carry
//! `content` *and* `tool_calls` together, rendered as
//!
//! ```text
//! <|assistant|>
//! <think></think>
//! Now let me read the route config.
//! <tool_call>read
//! <arg_key>path</arg_key>
//! ```
//!
//! (`huggingface.co/zai-org/GLM-4.5/blob/main/chat_template.jinja`)
//!
//! Accumulating that text produced answers made entirely of intent — observed
//! live as "I'll investigate... Let me start with parallel discovery. Now let me
//! read the actual files..." with no findings at all.
//!
//! Narration is still surfaced as a live event for the TUI; it is only withheld
//! from `progress.output`, which becomes the returned answer.

use crate::provider::ContentPart;

/// Returns `true` when this turn's text should be withheld from the answer.
///
/// A turn that also calls a tool is mid-work, so its text is a preamble. Turns
/// with no tool call are terminal and their text is the answer.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::ContentPart;
/// use codetether_agent::session::helper::prompt_loop::response::narration::is_preamble;
///
/// let call = ContentPart::ToolCall {
///     id: "1".into(), name: "read".into(),
///     arguments: "{}".into(), thought_signature: None,
/// };
/// let text = ContentPart::Text { text: "Now let me read it.".into() };
///
/// assert!(is_preamble(&[text.clone(), call]));
/// assert!(!is_preamble(&[text]));
/// ```
pub fn is_preamble(parts: &[ContentPart]) -> bool {
    parts
        .iter()
        .any(|part| matches!(part, ContentPart::ToolCall { .. }))
}

#[cfg(test)]
#[path = "narration_tests.rs"]
mod tests;
