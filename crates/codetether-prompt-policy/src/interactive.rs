//! Classify tools that require user interaction.
/// Return whether the tool requires interactive user input.
///
/// # Examples
///
/// ```rust
/// use codetether_prompt_policy::is_interactive_tool;
///
/// assert!(is_interactive_tool("question"));
/// assert!(!is_interactive_tool("read"));
/// ```
/// # Arguments
/// Receives a tool identifier.
/// # Returns
/// True when the loop must allow interactive user input.
pub fn is_interactive_tool(tool_name: &str) -> bool {
    matches!(tool_name, "question")
}
