//! One-step forwarding from the prompt event channel.

use crate::session::SessionEvent;
use crate::session::thread_events::ThreadEventMapper;
use tokio::sync::mpsc;

use super::event_forward;
use super::socket::SocketSink;

/// State change caused by receiving one prompt event.
pub(super) enum EventAction {
    Continue,
    Closed,
    Failed,
}

/// Receive and forward one event without owning the turn loop.
pub(super) async fn next(
    sink: &mut SocketSink,
    mapper: &mut ThreadEventMapper,
    events: &mut mpsc::Receiver<SessionEvent>,
) -> EventAction {
    let Some(event) = events.recv().await else {
        return EventAction::Closed;
    };
    match event_forward::session_event(sink, mapper, &event).await {
        Ok(()) => EventAction::Continue,
        Err(_) => EventAction::Failed,
    }
}
