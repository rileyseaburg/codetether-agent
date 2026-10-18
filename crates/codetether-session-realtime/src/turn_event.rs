//! One-step forwarding from the prompt event channel.

use codetether_session_events::SessionEvent;
use codetether_thread_events::ThreadEventMapper;
use tokio::sync::mpsc;

use super::event_forward;
use super::socket::SocketSink;

/// State change caused by receiving one prompt event.
/// # Examples
/// ```
/// let _frame = codetether_session_realtime::turn_event::EventAction::Continue;
/// ```
pub enum EventAction {
    Continue,
    Closed,
    Failed,
}

/// Receive and forward one event without owning the turn loop.
/// # Arguments
/// * `sink` — Sending half.
/// * `mapper` — Per-turn event mapper.
/// * `events` — Live prompt events.
/// # Returns
/// Continue, Closed or Failed according to event receipt and forwarding.
/// # Examples
/// ```no_run
/// # async fn example(sink: &mut codetether_session_realtime::socket::SocketSink, mapper: &mut codetether_thread_events::ThreadEventMapper, events: &mut tokio::sync::mpsc::Receiver<codetether_session_events::SessionEvent<()>>) {
/// let _ = codetether_session_realtime::turn_event::next(sink, mapper, events).await;
/// # }
/// ```
pub async fn next<S>(
    sink: &mut SocketSink,
    mapper: &mut ThreadEventMapper,
    events: &mut mpsc::Receiver<SessionEvent<S>>,
) -> EventAction {
    let Some(event) = events.recv().await else {
        return EventAction::Closed;
    };
    match event_forward::session_event(sink, mapper, &event).await {
        Ok(()) => EventAction::Continue,
        Err(_) => EventAction::Failed,
    }
}
