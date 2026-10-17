//! Bounded SSE ingestion over a caller-supplied frame consumer.
use crate::cursor::Cursor;
use crate::frame::{ParsedFrame, parse_frame};
use crate::staging::{StagingBuffer, StagingError};

/// Why a worker task stream connection ended.
#[derive(Debug, PartialEq, Eq)]
pub enum StreamDisconnectReason {
    Ended,
    ReadError(String),
}

/// Cap on the partial-frame staging buffer (1 MiB): a single SSE frame larger
/// than this is treated as a protocol violation rather than grown unbounded.
pub const STAGING_CAP_BYTES: usize = 1024 * 1024;
/// Application action for each complete, valid frame.
#[async_trait::async_trait]
pub trait FrameConsumer: Send {
    /// Apply one frame before ingestion advances to the next.
    async fn handle(&mut self, frame: &ParsedFrame, cursor: &mut Cursor);
}

/// Append a chunk to the bounded staging buffer, then drain complete
/// `\n\n`-delimited SSE frames and handle each.
///
/// # Errors
///
/// Returns [`StagingError::Overflow`] when an un-delimited frame exceeds the
/// staging cap; the caller disconnects and reconnects on this signal.
pub async fn ingest_chunk<C: FrameConsumer + ?Sized>(
    buffer: &mut StagingBuffer,
    chunk: &[u8],
    consumer: &mut C,
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
            consumer.handle(&frame, cursor).await;
        }
    }
    Ok(())
}
