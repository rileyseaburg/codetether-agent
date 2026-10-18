//! Construction and initial event emission for one realtime turn.

use codetether_thread_events::{ThreadEventContext, ThreadEventMapper};

use super::event_forward;
use super::prompt_run::PromptRun;
use super::socket::SocketSink;

/// Start one prompt after its turn-start event reaches the client.
pub(super) async fn start<S>(
    sink: &mut SocketSink,
    session_id: &str,
    message: String,
    model: Option<String>,
    start: super::PromptStarter<S>,
) -> Option<(ThreadEventMapper, PromptRun<S>)> {
    let context = ThreadEventContext::for_session(session_id.to_string());
    let mut mapper = ThreadEventMapper::new(context);
    let started = mapper.turn_started(&message);
    event_forward::thread_event(sink, started).await.ok()?;
    let run = start(session_id.to_string(), message, model);
    Some((mapper, run))
}
