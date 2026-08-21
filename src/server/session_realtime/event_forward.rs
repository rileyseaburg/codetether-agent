//! Mapping and forwarding of live session events.

use crate::session::SessionEvent;
use crate::session::thread_events::ThreadEventMapper;
use crate::session::thread_store::ThreadEvent;

use super::frames::ServerFrame;
use super::send;
use super::socket::SocketSink;

/// Map and send every thread event produced by one session event.
pub(super) async fn session_event(
    sink: &mut SocketSink,
    mapper: &mut ThreadEventMapper,
    event: &SessionEvent,
) -> Result<(), String> {
    for mapped in mapper.map_session_event(event) {
        thread_event(sink, mapped).await?;
    }
    Ok(())
}

/// Send one already-mapped thread event.
pub(super) async fn thread_event(sink: &mut SocketSink, event: ThreadEvent) -> Result<(), String> {
    send::frame(sink, &ServerFrame::Event { event }).await
}
