//! Drain provider streams into bounded responses with generic session events.
//!
//! [drain] returns a classified transport outcome for restart orchestration;
//! [collect_stream_completion_with_events] applies final acceptance to one drain.
//! Text, thinking and liveness events retain the shared session-event format.
mod drain_loop;
mod drain_state;
mod idle_drain;
mod idle_keepalive;
mod idle_timeout;
mod reasoning;
mod text_acc;
mod tool_delta;
mod tool_start;
use codetether_provider_core::{CompletionResponse, StreamChunk};
use codetether_session_events::SessionEvent;
use futures::stream::BoxStream;
pub use idle_timeout::drain;
/// Collect one stream and reject incomplete or empty terminal outcomes.
/// # Arguments
/// Receives a provider stream and optional typed session-event sender.
/// # Returns
/// The complete assembled response from one successful drain.
/// # Errors
/// Propagates final acceptance errors for faults, stalls and premature endings.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let stream = Box::pin(futures::stream::iter([
///     codetether_provider_core::StreamChunk::Text("ok".into()),
///     codetether_provider_core::StreamChunk::Done { usage: None },
/// ]));
/// let response = codetether_stream_drain::collect_stream_completion_with_events::<()>(stream, None).await?;
/// assert_eq!(response.message.content.len(), 1);
/// # Ok(())
/// # }
/// ```
pub async fn collect_stream_completion_with_events<S>(
    stream: BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
) -> anyhow::Result<CompletionResponse> {
    let outcome = drain(stream, event_tx).await;
    codetether_stream_outcome::accept(outcome, 0)
}
#[cfg(test)]
mod tests;
