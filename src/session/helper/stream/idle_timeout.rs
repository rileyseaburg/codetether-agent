//! Idle-timeout guard + stop classification for provider stream consumption.
//!
//! Wraps `stream.next().await` so a stalled HTTP connection (TCP open,
//! zero SSE events) cannot block the agent loop forever, and classifies *why*
//! the stream stopped into a [`StreamStop`] so the restart engine can decide
//! whether re-opening a fresh stream is sound.

use crate::provider::StreamChunk;
use crate::session::SessionEvent;
use futures::stream::BoxStream;
use std::time::Duration;

use super::drain_state::DrainState;
use super::idle_drain::apply;
use super::idle_fault::fault_from;
use super::idle_keepalive::{Next, next_with_keepalive};
use super::outcome::{DrainOutcome, StreamStop};

/// Maximum gap between consecutive stream chunks before the stream is
/// considered stalled. GLM-5.2 can think 60-90 s before the first token,
/// so 3 min gives generous headroom.
pub(super) const IDLE_TIMEOUT: Duration = Duration::from_secs(180);

/// Drain `stream`, classifying the stop reason. Never errors on its own; a
/// terminal error chunk is reported as [`StreamStop::Fault`] for the caller.
///
/// A provider stream that ends (`None`) *without* first emitting a
/// [`StreamChunk::Done`] terminated prematurely: the HTTP body closed before
/// the model signalled completion. Every streaming provider emits `Done` on a
/// real finish, so a missing `Done` is a reliable "connection dropped
/// mid-turn" signal. Rather than mis-report that as [`StreamStop::Clean`]
/// (which silently accepts a truncated reply), it is classified as
/// [`StreamStop::PrematureEnd`] so the restart engine re-requests a fresh
/// stream and discards the partial.
pub(super) async fn drain(
    mut stream: BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> DrainOutcome {
    let mut state = DrainState::new();
    let mut got_chunk = false;
    let mut saw_done = false;
    let stop = loop {
        match next_with_keepalive(&mut stream, event_tx, IDLE_TIMEOUT).await {
            Next::Chunk(Some(StreamChunk::Error(msg))) => break fault_from(&msg),
            Next::Chunk(Some(c)) => {
                got_chunk = true;
                saw_done |= matches!(c, StreamChunk::Done { .. });
                if let Err(e) = apply(&mut state, c, event_tx).await {
                    break fault_from(&e.to_string());
                }
            }
            Next::Chunk(None) if saw_done => break StreamStop::Clean,
            Next::Chunk(None) => {
                tracing::warn!("Stream ended before Done; premature termination");
                break StreamStop::PrematureEnd;
            }
            Next::IdleTimeout if !got_chunk => {
                tracing::warn!("Cold idle timeout; retryable");
                break StreamStop::ColdStall;
            }
            Next::IdleTimeout => {
                tracing::warn!("Idle timeout; returning partial");
                break StreamStop::MidStreamStall;
            }
        }
    };
    let response = state.finish();
    DrainOutcome {
        response: Some(response),
        stop,
    }
}
