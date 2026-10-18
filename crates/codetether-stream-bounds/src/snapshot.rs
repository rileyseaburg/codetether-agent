//! Bounded assistant snapshots preserve full-response accumulation elsewhere.
use crate::MAX_STREAM_SNAPSHOT_BYTES;

/// Build the snapshot string to send over the UI channel. Truncates
/// with a trailing marker when the accumulated text exceeds
/// [`MAX_STREAM_SNAPSHOT_BYTES`].
/// # Arguments
/// Receives accumulated assistant text.
/// # Returns
/// Text capped at the snapshot limit, with a trailing marker if truncated.
/// # Examples
/// ```
/// assert_eq!(codetether_stream_bounds::snapshot_for_send("ok"), "ok");
/// ```
pub fn snapshot_for_send(text: &str) -> String {
    if text.len() > MAX_STREAM_SNAPSHOT_BYTES {
        let mut t = codetether_util::truncate::truncate_bytes_safe(text, MAX_STREAM_SNAPSHOT_BYTES)
            .to_string();
        t.push_str(" …[truncated]");
        t
    } else {
        text.to_string()
    }
}
