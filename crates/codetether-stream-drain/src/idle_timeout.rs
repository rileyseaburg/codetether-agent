//! Idle-timeout guard + stop classification for provider stream consumption.
//!
//! Wraps `stream.next().await` so a stalled HTTP connection (TCP open,
//! zero SSE events) cannot block the agent loop forever, and classifies *why*
//! the stream stopped into a [`StreamStop`] so the restart engine can decide
//! whether re-opening a fresh stream is sound.

use codetether_provider_core::StreamChunk;
use codetether_session_events::SessionEvent;
use futures::stream::BoxStream;

use super::drain_state::DrainState;
pub(super) use codetether_stream_outcome::IDLE_TIMEOUT;
use codetether_stream_outcome::{DrainOutcome, StreamStop};

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
/// # Arguments
/// Receives a provider stream and optional typed event sender.
/// # Returns
/// The assembled response, committed content items and terminal stop reason.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let stream = Box::pin(futures::stream::empty());
/// let outcome = codetether_stream_drain::drain::<()>(stream, None).await;
/// assert!(matches!(outcome.stop, codetether_stream_outcome::StreamStop::PrematureEnd));
/// # }
/// ```
pub async fn drain<S>(
    mut stream: BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
) -> DrainOutcome {
    let mut state = DrainState::new();
    let stop = super::drain_loop::run(&mut stream, event_tx, &mut state).await;
    let (response, completed) = state.finish();
    DrainOutcome {
        response: Some(response),
        completed,
        stop,
    }
}
