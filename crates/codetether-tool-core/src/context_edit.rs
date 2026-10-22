//! Tools that modify the calling session outside its current in-memory owner.
//!
//! Save pending transcript data before these tools, then refresh after success.
/// Whether a tool requires a persisted session and a subsequent state refresh.
/// # Arguments
/// `name` is the canonical registered tool name.
/// # Returns
/// True for context pinning and summary changes, false for ordinary tools.
/// # Examples
/// ```
/// assert!(codetether_tool_core::context_edit::is_context_edit("context_pin"));
/// assert!(!codetether_tool_core::context_edit::is_context_edit("read"));
/// ```
pub fn is_context_edit(name: &str) -> bool {
    matches!(name, "context_pin" | "context_summarize")
}
