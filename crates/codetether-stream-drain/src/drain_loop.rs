//! Wait for chunks, dispatch them and classify the first terminal condition.
use super::{
    drain_state::DrainState,
    idle_drain::apply,
    idle_keepalive::{Next, next_with_keepalive},
};
use codetether_provider_core::StreamChunk;
use codetether_session_events::SessionEvent;
use codetether_stream_outcome::{IDLE_TIMEOUT, StreamStop, fault_from};
use futures::stream::BoxStream;
pub(super) async fn run<S>(
    stream: &mut BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
    state: &mut DrainState,
) -> StreamStop {
    let mut got_chunk = false;
    let stop = loop {
        match next_with_keepalive(stream, event_tx, IDLE_TIMEOUT).await {
            Next::Chunk(Some(StreamChunk::Error(msg))) => break fault_from(&msg),
            Next::Chunk(Some(c)) => {
                got_chunk = true;
                let done = matches!(c, StreamChunk::Done { .. });
                if let Err(e) = apply(state, c, event_tx).await {
                    break fault_from(&e.to_string());
                }
                if done {
                    break StreamStop::Clean;
                }
            }
            Next::Chunk(None) => {
                tracing::warn!(target: "codetether_agent::session::helper::stream::idle_timeout","Stream ended before Done; premature termination");
                break StreamStop::PrematureEnd;
            }
            Next::IdleTimeout if !got_chunk => {
                tracing::warn!(target: "codetether_agent::session::helper::stream::idle_timeout","Cold idle timeout; retryable");
                break StreamStop::ColdStall;
            }
            Next::IdleTimeout => {
                tracing::warn!(target: "codetether_agent::session::helper::stream::idle_timeout","Idle timeout; returning partial");
                break StreamStop::MidStreamStall;
            }
        }
    };

    stop
}
