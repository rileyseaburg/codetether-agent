use super::Host;
use crate::{
    cursor::Cursor,
    frame::ParsedFrame,
    staging::{StagingBuffer, StagingError},
};
struct Consumer<'a, H>(&'a H);
#[async_trait::async_trait]
impl<H: Host> crate::ingest::FrameConsumer for Consumer<'_, H> {
    async fn handle(&mut self, frame: &ParsedFrame, cursor: &mut Cursor) {
        crate::dispatch::handle_frame(frame, self.0, cursor).await;
    }
}
pub(super) async fn ingest_chunk<H: Host>(
    buffer: &mut StagingBuffer,
    chunk: &[u8],
    host: &H,
    cursor: &mut Cursor,
) -> Result<(), StagingError> {
    let mut consumer = Consumer(host);
    crate::ingest::ingest_chunk(buffer, chunk, &mut consumer, cursor).await
}
