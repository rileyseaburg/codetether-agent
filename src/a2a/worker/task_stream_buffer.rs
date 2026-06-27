//! Bounded frame ingestion and cursor-path helpers for the task stream.

use crate::a2a::stream::cursor::Cursor;
use crate::a2a::stream::frame::parse_frame;
use crate::a2a::stream::staging::{StagingBuffer, StagingError};

use super::{WorkerTaskRuntime, frame_handler::handle_frame};

/// Why a worker task stream connection ended.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum StreamDisconnectReason {
    Ended,
    ReadError(String),
}

/// Cap on the partial-frame staging buffer (1 MiB): a single SSE frame larger
/// than this is treated as a protocol violation rather than grown unbounded.
pub(super) const STAGING_CAP_BYTES: usize = 1024 * 1024;

/// Resume-cursor file path for a given worker id.
pub(super) fn cursor_path(worker_id: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("codetether-stream-cursor-{worker_id}"))
}

/// Append a chunk to the bounded staging buffer, then drain complete
/// `\n\n`-delimited SSE frames and handle each.
///
/// # Errors
///
/// Returns [`StagingError::Overflow`] when an un-delimited frame exceeds the
/// staging cap; the caller disconnects and reconnects on this signal.
pub(super) async fn ingest_chunk(
    buffer: &mut StagingBuffer,
    chunk: &[u8],
    runtime: &WorkerTaskRuntime,
    cursor: &mut Cursor,
) -> Result<(), StagingError> {
    buffer.extend(chunk)?;
    let bytes = buffer.bytes_mut();
    while let Some(pos) = bytes.windows(2).position(|w| w == b"\n\n") {
        let event_bytes = bytes[..pos].to_vec();
        bytes.drain(..pos + 2);
        let Ok(event_str) = std::str::from_utf8(&event_bytes) else {
            continue;
        };
        if let Some(frame) = parse_frame(event_str) {
            handle_frame(&frame, runtime, cursor).await;
        }
    }
    Ok(())
}
