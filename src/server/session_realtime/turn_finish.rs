//! Ordered draining and terminal framing for a completed prompt task.

use crate::session::thread_events::ThreadEventMapper;
use crate::session::{SessionEvent, SessionResult};
use tokio::sync::mpsc;
use tokio::task::JoinError;

use super::event_forward;
use super::frames::ServerFrame;
use super::send;
use super::socket::SocketSink;

/// Drain pending events before sending the terminal result or error.
pub(super) async fn finish(
    sink: &mut SocketSink,
    mapper: &mut ThreadEventMapper,
    events: &mut mpsc::Receiver<SessionEvent>,
    outcome: Result<Result<SessionResult, String>, JoinError>,
) -> Result<(), String> {
    while let Some(event) = events.recv().await {
        event_forward::session_event(sink, mapper, &event).await?;
    }
    match outcome {
        Ok(Ok(result)) => {
            let completed = mapper.turn_completed(&result.text);
            event_forward::thread_event(sink, completed).await?;
            send::frame(sink, &ServerFrame::Result { result }).await
        }
        Ok(Err(message)) => send_error(sink, message).await,
        Err(error) => send_error(sink, error.to_string()).await,
    }
}

/// Send one terminal failure frame.
pub(super) async fn send_error(sink: &mut SocketSink, message: String) -> Result<(), String> {
    send::frame(sink, &ServerFrame::Error { message }).await
}
