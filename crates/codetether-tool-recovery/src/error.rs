//! Tool-result error text for incomplete provider argument JSON.
use super::rules::retry_rule;

/// Build the tool-result error text shown to the model for a truncated call.
/// # Arguments
/// Receives the truncated tool name.
/// # Returns
/// The existing corrective tool-result text.
/// # Examples
/// ```
/// assert!(codetether_tool_recovery::error_content("write").contains("first chunk"));
/// ```
pub fn error_content(tool: &str) -> String {
    format!(
        "Error: Your tool call to `{tool}` was truncated because the provider cut off the \
         arguments JSON before it could be parsed (the response hit the output-token limit). \
         Recovery required: {} Do not \
         provide a final answer until the smaller tool call succeeds or you have tried a \
         different small diagnostic step.",
        retry_rule(tool)
    )
}
