//! Mapping and forwarding of live session events.

use codetether_session_events::SessionEvent;
use codetether_thread_events::ThreadEventMapper;
use codetether_thread_store::ThreadEvent;

use super::frames::ServerFrame;
use super::send;
use super::socket::SocketSink;

/// Map and send every thread event produced by one session event.
/// # Arguments
/// * `sink` — Sending half.
/// * `mapper` — Per-turn event mapper.
/// * `event` — Shared session event.
/// # Returns
/// Unit after all mapped events have been sent.
/// # Errors
/// Returns the first send failure.
/// # Examples
/// ```no_run
/// # async fn example(sink: &mut codetether_session_realtime::socket::SocketSink, mapper: &mut codetether_thread_events::ThreadEventMapper, event: &codetether_session_events::SessionEvent<()>) {
/// let _ = codetether_session_realtime::event_forward::session_event(sink, mapper, event).await;
/// # }
/// ```
pub async fn session_event<S>(
    sink: &mut SocketSink,
    mapper: &mut ThreadEventMapper,
    event: &SessionEvent<S>,
) -> Result<(), String> {
    for mapped in mapper.map_session_event(event) {
        thread_event(sink, mapped).await?;
    }
    Ok(())
}

/// Send one already-mapped thread event.
/// # Arguments
/// * `sink` — Sending half.
/// * `event` — Thread event to forward.
/// # Returns
/// Unit after sending.
/// # Errors
/// Returns serialization or send failure text.
/// # Examples
/// ```no_run
/// # async fn example(sink: &mut codetether_session_realtime::socket::SocketSink, event: codetether_thread_store::ThreadEvent) {
/// let _ = codetether_session_realtime::event_forward::thread_event(sink, event).await;
/// # }
/// ```
pub async fn thread_event(sink: &mut SocketSink, event: ThreadEvent) -> Result<(), String> {
    send::frame(sink, &ServerFrame::Event { event }).await
}
