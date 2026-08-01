//! Conversion of Gemini's mixed text slot into typed content parts.

use super::{split_reasoning, strip_ui_markup};
use crate::provider::ContentPart;

/// Separates leaked reasoning from answer text and removes web-client markup.
///
/// Reasoning is retained as [`ContentPart::Thinking`], not discarded. This
/// preserves useful model telemetry and lets the session watchdog observe
/// progress during a long thinking phase.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::response_text::content_parts;
/// use codetether_agent::provider::ContentPart;
///
/// let parts = content_parts(
///     "I will inspect the files.Here is the answer.\n<FollowUp label=\"x\"/>",
/// );
/// assert!(matches!(&parts[0], ContentPart::Thinking { text, .. }
///     if text == "I will inspect the files."));
/// assert!(matches!(&parts[1], ContentPart::Text { text }
///     if text == "Here is the answer."));
/// ```
pub fn content_parts(text: &str) -> Vec<ContentPart> {
    let visible = strip_ui_markup(text);
    let (reasoning, answer) = split_reasoning(&visible);
    let mut parts = Vec::new();
    if !reasoning.is_empty() {
        parts.push(ContentPart::Thinking {
            text: reasoning,
            signature: None,
        });
    }
    if !answer.trim().is_empty() {
        parts.push(ContentPart::Text {
            text: answer.trim().to_string(),
        });
    }
    parts
}
