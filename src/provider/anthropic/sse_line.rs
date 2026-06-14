//! SSE event line parsing for Anthropic-compatible streaming responses.
//!
//! Anthropic's Messages API streams responses as Server-Sent Events (SSE).
//! Each event has an optional `event:` line and a `data:` line. This module
//! extracts `(event_type, data)` pairs from raw byte lines read off the HTTP
//! response body.

/// Parse one SSE line into its optional event type and data payload.
///
/// Returns `None` for blank lines (event boundaries) and comment lines (`:`).
///
/// # Parameters
///
/// * `line` - A single UTF-8 line from the SSE stream (with trailing `\n`
///   already stripped by the caller).
///
/// # Returns
///
/// * `None` — blank line, comment, or non-SSE content.
/// * `Some((event, None))` — `data:` line without a preceding `event:`.
/// * `Some((event, Some(data)))` — paired event + data.
pub(crate) fn parse_sse_line(line: &str) -> Option<(Option<String>, Option<String>)> {
    let trimmed = line.trim_end();
    if trimmed.is_empty() || trimmed.starts_with(':') {
        return None;
    }
    if let Some(data) = trimmed.strip_prefix("data:") {
        return Some((None, Some(data.trim().to_string())));
    }
    if let Some(event) = trimmed.strip_prefix("event:") {
        return Some((Some(event.trim().to_string()), None));
    }
    None
}
