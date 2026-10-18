//! Lazily compiled matching for tool-call JSON markup.
use regex::Regex;
use std::sync::OnceLock;

/// # Arguments
/// Takes no arguments.
/// # Returns
/// The cached compiled markup expression.
/// # Examples
/// ```
/// assert!(codetether_tool_call_parser::tool_call_markup_re().is_match("<tool_call>{}</tool_call>"));
/// ```
pub fn tool_call_markup_re() -> &'static Regex {
    static TOOL_CALL_RE: OnceLock<Regex> = OnceLock::new();
    TOOL_CALL_RE.get_or_init(|| {
        Regex::new(r"(?s)<tool_call>\s*(?:```(?:json)?\s*)?(\{.*?\})(?:\s*```)?\s*</tool_call>")
            .expect("tool_call regex must compile")
    })
}
