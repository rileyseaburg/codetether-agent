//! Bound retained tool payloads before placing them in chat history.
//!
//! [tool_output] strips a feedback envelope and bounds output;
//! [tool_arguments] bounds argument text while preserving UTF-8 boundaries.
mod bounded;
mod feedback;
use bounded::bounded;
use feedback::strip_tool_feedback;

const TOOL_OUTPUT_RETAINED_BYTES: usize = 8 * 1024;

const TOOL_ARGUMENTS_RETAINED_BYTES: usize = 4 * 1024;

/// Bound tool output retained in the TUI message list.
/// # Arguments
/// * `output` - Raw tool payload.
/// # Returns
/// Bounded UTF-8 text, including a truncation marker if required.
/// # Examples
/// ```
/// assert_eq!(codetether_chat_model::payload::tool_output("ok"), "ok");
/// ```
pub fn tool_output(output: &str) -> String {
    bounded(
        strip_tool_feedback(output),
        TOOL_OUTPUT_RETAINED_BYTES,
        "tool output",
    )
}

/// Bound tool call arguments retained in the TUI message list.
/// # Arguments
/// * `arguments` - Raw tool payload.
/// # Returns
/// Bounded UTF-8 text, including a truncation marker if required.
/// # Examples
/// ```
/// assert_eq!(codetether_chat_model::payload::tool_arguments("ok"), "ok");
/// ```
pub fn tool_arguments(arguments: &str) -> String {
    bounded(arguments, TOOL_ARGUMENTS_RETAINED_BYTES, "tool arguments")
}
#[cfg(test)]
mod tests;
