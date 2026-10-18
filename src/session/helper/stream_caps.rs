//! Shared stream accumulator limits and preview truncation.
pub use codetether_stream_bounds::{
    MAX_STREAM_SNAPSHOT_BYTES, MAX_STREAM_TEXT_BYTES, MAX_STREAM_TOOL_ARGS_BYTES, ensure_text_room,
    ensure_tool_args_room, snapshot_for_send,
};
