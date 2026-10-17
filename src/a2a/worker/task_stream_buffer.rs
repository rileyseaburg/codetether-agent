//! Bind shared frame ingestion to the worker task dispatcher.
use super::{WorkerTaskRuntime, frame_handler::handle_frame};
use crate::a2a::stream::cursor::Cursor;
use crate::a2a::stream::frame::ParsedFrame;
use crate::a2a::stream::staging::{StagingBuffer, StagingError};
pub(super) use codetether_a2a_stream::ingest::{STAGING_CAP_BYTES, StreamDisconnectReason};

/// Resume-cursor file path for a given worker id.
pub(super) fn cursor_path(worker_id: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("codetether-stream-cursor-{worker_id}"))
}
struct Consumer<'a>(&'a WorkerTaskRuntime);
#[async_trait::async_trait]
impl codetether_a2a_stream::ingest::FrameConsumer for Consumer<'_> {
    async fn handle(&mut self, frame: &ParsedFrame, cursor: &mut Cursor) {
        handle_frame(frame, self.0, cursor).await;
    }
}
pub(super) async fn ingest_chunk(
    buffer: &mut StagingBuffer,
    chunk: &[u8],
    runtime: &WorkerTaskRuntime,
    cursor: &mut Cursor,
) -> Result<(), StagingError> {
    let mut consumer = Consumer(runtime);
    codetether_a2a_stream::ingest::ingest_chunk(buffer, chunk, &mut consumer, cursor).await
}
