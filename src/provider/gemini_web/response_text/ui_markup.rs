//! Removal of Gemini Web chat-UI affordance markup.
//!
//! Gemini's web transport appends tags meant for its own chat client into the
//! same text slot the provider reads. Captured live from `gemini-web-fast`:
//!
//! ```text
//! ...executes its own suite of MCP tools to accomplish the objective.
//!
//! <FollowUp label="Would you like an example implementation of an MCP server
//! or an A2A Agent Card?" query="Show me an example implementation..."/>
//! ```
//!
//! Left in place, that tag surfaces to the user as part of the assistant's
//! answer.

use regex::Regex;
use std::sync::OnceLock;

/// Matches self-closing affordance tags such as `<FollowUp ... />`.
///
/// Rust's `regex` crate has no backreferences, so paired forms are handled by a
/// separate pattern per tag name rather than with `\1`.
fn self_closing_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE.get_or_init(|| {
        Regex::new(r"(?is)<\s*(?:followup|follow_up|suggestion|suggestions|relatedcontent)\b[^>]*>")
            .unwrap()
    })
}

/// Matches closing affordance tags such as `</FollowUp>`.
fn closing_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE.get_or_init(|| {
        Regex::new(r"(?is)<\s*/\s*(?:followup|follow_up|suggestion|suggestions|relatedcontent)\s*>")
            .unwrap()
    })
}

/// Removes Gemini chat-UI affordance markup from `text`.
///
/// Opening and closing tags are stripped independently so a malformed or
/// unbalanced tag cannot swallow surrounding answer text.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::response_text::strip_ui_markup;
///
/// let cleaned = strip_ui_markup("Answer.\n<FollowUp label=\"a\" query=\"b\"/>");
/// assert_eq!(cleaned.trim(), "Answer.");
///
/// // Unrelated markup is preserved.
/// assert_eq!(strip_ui_markup("plain"), "plain");
/// assert!(strip_ui_markup("<tool_call>{}</tool_call>").contains("tool_call"));
/// ```
pub fn strip_ui_markup(text: &str) -> String {
    let without_open = self_closing_regex().replace_all(text, "");
    closing_regex().replace_all(&without_open, "").to_string()
}
