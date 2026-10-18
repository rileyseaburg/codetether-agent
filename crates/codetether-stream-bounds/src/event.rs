//! Bounded tool payloads for live session and UI events.
use codetether_util::truncate::truncate_bytes_safe;

pub(crate) const TOOL_OUTPUT_EVENT_MAX_BYTES: usize = 8 * 1024;

pub(crate) const TOOL_ARGUMENTS_EVENT_MAX_BYTES: usize = 4 * 1024;

const TRUNCATED_OUTPUT_MARKER: &str =
    "\n\n[truncated for live TUI event; full output remains in session history]";

const TRUNCATED_ARGUMENTS_MARKER: &str = "\n\n[truncated for live TUI event: tool arguments]";

/// # Arguments
/// Receives the full tool output.
/// # Returns
/// An 8 KiB-or-smaller live preview, with a marker when truncated.
/// # Examples
/// ```
/// assert_eq!(codetether_stream_bounds::bounded_tool_output("ok"), "ok");
/// ```
pub fn bounded_tool_output(output: &str) -> String {
    bounded(output, TOOL_OUTPUT_EVENT_MAX_BYTES, TRUNCATED_OUTPUT_MARKER)
}

/// # Arguments
/// Receives serialized tool arguments.
/// # Returns
/// A 4 KiB-or-smaller preview, with a marker when truncated.
/// # Examples
/// ```
/// assert_eq!(codetether_stream_bounds::bounded_tool_arguments("{}"), "{}");
/// ```
pub fn bounded_tool_arguments(arguments: &str) -> String {
    bounded(
        arguments,
        TOOL_ARGUMENTS_EVENT_MAX_BYTES,
        TRUNCATED_ARGUMENTS_MARKER,
    )
}

fn bounded(input: &str, max_bytes: usize, marker: &str) -> String {
    if input.len() <= max_bytes {
        return input.to_string();
    }
    let content_bytes = max_bytes.saturating_sub(marker.len());
    let mut bounded = truncate_bytes_safe(input, content_bytes).to_string();
    bounded.push_str(marker);
    bounded
}
