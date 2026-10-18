//! Overflow-safe rejection of unbounded stream growth.
use crate::{MAX_STREAM_TEXT_BYTES, MAX_STREAM_TOOL_ARGS_BYTES};
use anyhow::{Result, bail};

/// Bail if appending `delta` to `current` would push the accumulated
/// assistant text past [`MAX_STREAM_TEXT_BYTES`].
/// # Arguments
/// Receives current and incoming byte counts.
/// # Returns
/// Ok when the combined size fits the text accumulator.
/// # Errors
/// Returns an error when the saturating total exceeds the text cap.
/// # Examples
/// ```
/// assert!(codetether_stream_bounds::ensure_text_room(2, 3).is_ok());
/// ```
pub fn ensure_text_room(current_len: usize, delta_len: usize) -> Result<()> {
    if current_len.saturating_add(delta_len) > MAX_STREAM_TEXT_BYTES {
        bail!(
            "assistant text stream exceeded {} MiB cap (runaway provider?); aborting turn",
            MAX_STREAM_TEXT_BYTES / (1024 * 1024),
        );
    }
    Ok(())
}

/// Bail if appending `delta` to a tool call's current argument buffer
/// would push it past [`MAX_STREAM_TOOL_ARGS_BYTES`]. `tool_id` is
/// only used for the error message.
/// # Arguments
/// Receives current/incoming byte counts and the tool-call identifier.
/// # Returns
/// Ok when the combined size fits the tool-arguments accumulator.
/// # Errors
/// Returns an error when the saturating total exceeds the tool-argument cap.
/// # Examples
/// ```
/// assert!(codetether_stream_bounds::ensure_tool_args_room(2, 3, "tool-1").is_ok());
/// ```
pub fn ensure_tool_args_room(current_len: usize, delta_len: usize, tool_id: &str) -> Result<()> {
    if current_len.saturating_add(delta_len) > MAX_STREAM_TOOL_ARGS_BYTES {
        bail!(
            "tool-call arguments exceeded {} MiB cap on tool id={} (runaway provider?); aborting turn",
            MAX_STREAM_TOOL_ARGS_BYTES / (1024 * 1024),
            tool_id,
        );
    }
    Ok(())
}
