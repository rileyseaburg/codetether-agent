//! Tolerance for models echoing our own transcript formatting.
//!
//! We render executed tool output into the prompt as `<tool_result>{...}` — see
//! `prompt::part::render_result`. Gemini is then asked to emit `<tool_call>`
//! blocks in the same visual style, so a model that pattern-matches the
//! transcript sometimes appends a `<tool_result>` of its own after a legitimate
//! call.
//!
//! Rejecting the whole turn for that is self-inflicted: the retry replays the
//! same transcript, so the model repeats the mimicry and the turn dies with
//! "tool protocol retry failed" — observed live in a real session.
//!
//! Forging output *instead of* calling a tool is still a genuine protocol
//! violation and must fail. The distinction is whether a real call is present.

use super::tool_calls;
use anyhow::{Result, bail};

/// Removes mimicked result markup, or fails when no real call accompanies it.
///
/// # Errors
///
/// Returns an error when the response contains fabricated `<tool_result>` output
/// with no genuine `<tool_call>` preceding it.
pub(super) fn tolerate(text: &str) -> Result<String> {
    if !tool_calls::contains_result_markup(text) {
        return Ok(text.to_string());
    }
    let (kept, _) = strip_trailing(text);
    if tool_calls::extract(&kept).1.is_empty() {
        bail!("assistant response contains forged <tool_result> markup");
    }
    tracing::warn!("Gemini Web echoed transcript <tool_result> markup; stripped");
    Ok(kept)
}

/// Splits trailing mimicked `<tool_result>` markup off a response.
///
/// Returns `(kept, stripped)` where `stripped` is true when mimicry was removed.
/// Only content at or after the first `<tool_result>` is dropped, and only when
/// the caller has confirmed a genuine `<tool_call>` precedes it.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::tool_validation::mimicry::strip_trailing;
///
/// let (kept, stripped) = strip_trailing(
///     "<tool_call>{\"name\":\"read\"}</tool_call><tool_result>{}</tool_result>",
/// );
/// assert!(stripped);
/// assert!(!kept.contains("tool_result"));
///
/// // Nothing to strip.
/// let (kept, stripped) = strip_trailing("plain answer");
/// assert!(!stripped);
/// assert_eq!(kept, "plain answer");
/// ```
pub fn strip_trailing(text: &str) -> (String, bool) {
    let Some(index) = text.find("<tool_result") else {
        return (text.to_string(), false);
    };
    (text[..index].trim_end().to_string(), true)
}

#[cfg(test)]
#[path = "mimicry_tests.rs"]
mod tests;
