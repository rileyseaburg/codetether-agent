//! Splitting leaked model reasoning from the answer in Gemini Web responses.
//!
//! Gemini Web has no separate reasoning channel, so a thinking model's narration
//! arrives concatenated with the answer in one text slot. Captured live from
//! `gemini-web-fast`:
//!
//! ```text
//! I will run a glob check to explore the project structure...I am inspecting
//! the workspace to check the repository status and contents.Here is a
//! breakdown of the differences between **A2A** and **MCP**...
//! ```
//!
//! Note there is no separator: sentences are glued directly to the answer.
//!
//! Reasoning is preserved as [`crate::provider::StreamChunk::Thinking`] rather
//! than dropped. It is genuine model output, it explains what the model believed
//! it was doing, and emitting it refreshes the session watchdog during long
//! silent phases.
//!
//! Conservative by design: only first-person narration at the *start* of a
//! response is treated as reasoning, since a false positive would silently
//! delete answer text.

#[path = "reasoning/sentence.rs"]
pub mod sentence;

use sentence::sentence_end;

/// First-person openers that mark narration rather than answer content.
const OPENERS: &[&str] = &[
    "I will ",
    "I am ",
    "I'll ",
    "I need to ",
    "I should ",
    "Let me ",
    "Let's ",
    "First, I ",
    "Now I ",
];

/// Returns `true` when `text` starts with first-person narration.
fn is_narration(text: &str) -> bool {
    OPENERS.iter().any(|opener| text.starts_with(opener))
}

/// Splits `text` into `(reasoning, answer)`.
///
/// Returns an empty reasoning string when no leading narration is detected, so
/// the answer is never altered in the common case. When the response is *only*
/// narration, it is returned as the answer rather than yielding an empty reply.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::response_text::split_reasoning;
///
/// let (thinking, answer) = split_reasoning("I will check the files.Here is the result.");
/// assert_eq!(thinking, "I will check the files.");
/// assert_eq!(answer, "Here is the result.");
///
/// let (thinking, answer) = split_reasoning("The answer is 42.");
/// assert!(thinking.is_empty());
/// assert_eq!(answer, "The answer is 42.");
/// ```
pub fn split_reasoning(text: &str) -> (String, String) {
    let trimmed = text.trim_start();
    if !is_narration(trimmed) {
        return (String::new(), text.to_string());
    }
    let cut = narration_end(trimmed);
    if cut == 0 {
        return (String::new(), text.to_string());
    }
    let answer = trimmed[cut..].trim_start().to_string();
    if answer.is_empty() {
        return (String::new(), text.to_string());
    }
    (trimmed[..cut].trim().to_string(), answer)
}

/// Returns the byte index where leading narration stops.
fn narration_end(trimmed: &str) -> usize {
    let mut cut = 0usize;
    let mut cursor = 0usize;
    while let Some(end) = sentence_end(&trimmed[cursor..]) {
        let absolute = cursor + end;
        if cursor > 0 && !is_narration(trimmed[cursor..absolute].trim_start()) {
            break;
        }
        cut = absolute;
        cursor = absolute;
    }
    cut
}
