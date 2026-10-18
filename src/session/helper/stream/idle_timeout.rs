//! Native event binding for shared stream draining.
use crate::{provider::StreamChunk, session::SessionEvent};
pub(super) use codetether_stream_outcome::IDLE_TIMEOUT;
use futures::stream::BoxStream;
pub(super) async fn drain(
    stream: BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> codetether_stream_outcome::DrainOutcome {
    codetether_stream_drain::drain(stream, event_tx).await
}
