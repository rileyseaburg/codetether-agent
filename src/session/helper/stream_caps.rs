//! Hard size caps for the per-turn streaming accumulators in
//! [`super::stream`]. A runaway provider that streams without
//! terminating cannot grow these buffers into OOM territory; the
//! offending turn fails with a descriptive error instead.

use anyhow::{Result, bail};

/// Cap on the total accumulated assistant text per turn. Real replies
/// top out around a few hundred KiB; 64 MiB is two-plus orders of
/// magnitude of headroom.
pub const MAX_STREAM_TEXT_BYTES: usize = 64 * 1024 * 1024;

/// Cap on a single tool call's argument JSON. Realistic tool calls are
/// well under 1 MiB; anything past 16 MiB is a broken provider.
pub const MAX_STREAM_TOOL_ARGS_BYTES: usize = 16 * 1024 * 1024;

/// Cap on a single forwarded `TextChunk` snapshot. Bounds the per-event
/// preview payload at 256 KiB regardless of how long the reply has
/// grown; the final response is **not** truncated by this.
pub const MAX_STREAM_SNAPSHOT_BYTES: usize = 256 * 1024;

/// Build the snapshot string to send over the UI channel. Truncates
/// with a trailing marker when the accumulated text exceeds
/// [`MAX_STREAM_SNAPSHOT_BYTES`].
pub fn snapshot_for_send(text: &str) -> String {
    if text.len() > MAX_STREAM_SNAPSHOT_BYTES {
        let mut t = crate::util::truncate_bytes_safe(text, MAX_STREAM_SNAPSHOT_BYTES).to_string();
        t.push_str(" …[truncated]");
        t
    } else {
        text.to_string()
    }
}

/// Bail if appending `delta` to `current` would push the accumulated
/// assistant text past [`MAX_STREAM_TEXT_BYTES`].
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
