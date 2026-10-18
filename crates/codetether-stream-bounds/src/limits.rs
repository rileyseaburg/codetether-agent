//! Byte budgets for full stream accumulators and assistant previews.

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
