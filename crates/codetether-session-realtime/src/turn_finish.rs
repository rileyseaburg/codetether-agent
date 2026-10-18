//! Ordered draining and terminal framing for a completed prompt task.

use codetether_session_events::{SessionEvent, SessionResult};
use codetether_thread_events::ThreadEventMapper;
use tokio::sync::mpsc;
use tokio::task::JoinError;

use super::event_forward;
use super::frames::ServerFrame;
use super::send;
use super::socket::SocketSink;

/// Drain pending events before sending the terminal result or error.
/// # Arguments
/// * `sink` — Sending half.
/// * `mapper` — Per-turn event mapper.
/// * `events` — Events to drain first.
/// * `outcome` — Joined prompt result.
/// # Returns
/// Unit after ordered terminal framing.
/// # Errors
/// Returns event or terminal-frame send failure.
/// # Examples
/// ```no_run
/// # async fn example(sink: &mut codetether_session_realtime::socket::SocketSink, mapper: &mut codetether_thread_events::ThreadEventMapper, events: &mut tokio::sync::mpsc::Receiver<codetether_session_events::SessionEvent<()>>, outcome: Result<Result<codetether_session_events::SessionResult, String>, tokio::task::JoinError>) {
/// let _ = codetether_session_realtime::turn_finish::finish(sink, mapper, events, outcome).await;
/// # }
/// ```
pub async fn finish<S>(
    sink: &mut SocketSink,
    mapper: &mut ThreadEventMapper,
    events: &mut mpsc::Receiver<SessionEvent<S>>,
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
/// # Arguments
/// * `sink` — Sending half.
/// * `message` — Failure text.
/// # Returns
/// Unit after the terminal error is sent.
/// # Errors
/// Returns serialization or send failure text.
/// # Examples
/// ```no_run
/// # async fn example(sink: &mut codetether_session_realtime::socket::SocketSink, message: String) {
/// let _ = codetether_session_realtime::turn_finish::send_error(sink, message).await;
/// # }
/// ```
pub async fn send_error(sink: &mut SocketSink, message: String) -> Result<(), String> {
    send::frame(sink, &ServerFrame::Error { message }).await
}
