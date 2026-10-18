//! Bounded live bus payload helpers.

pub const LIVE_BUS_MAX_BYTES: usize = 16 * 1024;

pub fn bounded(input: &str, label: &str) -> String {
    let mut out =
        codetether_util::truncate::truncate_bytes_safe(input, LIVE_BUS_MAX_BYTES).to_string();
    if input.len() > LIVE_BUS_MAX_BYTES {
        out.push_str(label);
    }
    out
}

/// Format a bounded live payload with the established truncation label.
/// # Arguments
/// Receives the source text and custom label.
/// # Returns
/// A UTF-8-safe preview, appending a label only when truncated.
/// # Examples
/// ```
/// assert_eq!(codetether_bus_core::payload::compact("ok", "truncated"), "ok");
/// ```
pub fn compact(input: &str, label: &str) -> String {
    bounded(input, label)
}

/// Format a bounded live payload with the established truncation label.
/// # Arguments
/// Receives the source text.
/// # Returns
/// A UTF-8-safe preview, appending a label only when truncated.
/// # Examples
/// ```
/// assert_eq!(codetether_bus_core::payload::compact_tool("ok"), "ok");
/// ```
pub fn compact_tool(input: &str) -> String {
    compact(input, "\n\n[truncated in session bus payload: tool output]")
}

/// Format a bounded live payload with the established truncation label.
/// # Arguments
/// Receives the source text.
/// # Returns
/// A UTF-8-safe preview, appending a label only when truncated.
/// # Examples
/// ```
/// assert_eq!(codetether_bus_core::payload::compact_thinking("ok"), "ok");
/// ```
pub fn compact_thinking(input: &str) -> String {
    compact(input, "\n\n[truncated in session bus payload: thinking]")
}
