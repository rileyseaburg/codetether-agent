//! Session ownership checks for server-wide PTY window ids.

use crate::mux::protocol::ProgramRequest;

use super::context::ServerContext;

/// The window a request targets, if it targets one.
pub(super) fn window_of(request: &ProgramRequest) -> Option<u64> {
    match request {
        ProgramRequest::Steer { .. } => None,
        ProgramRequest::Start { window_id, .. }
        | ProgramRequest::Attach { window_id, .. }
        | ProgramRequest::Tail { window_id }
        | ProgramRequest::Input { window_id, .. }
        | ProgramRequest::Read { window_id, .. }
        | ProgramRequest::Resize { window_id, .. } => Some(*window_id),
    }
}

/// Fail unless `session` owns window `id`; other sessions' windows do not exist to it.
pub(super) async fn owned(context: &ServerContext, session: &str, id: u64) -> anyhow::Result<()> {
    let state = context.state.read().await;
    let exists = state.session(session).and_then(|item| item.window(id));
    anyhow::ensure!(exists.is_some(), "window {id} does not exist");
    Ok(())
}
