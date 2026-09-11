//! Session removal on a workspace mux server.

use std::sync::Arc;

use crate::mux::protocol::ServerResponse;

use super::context::ServerContext;
use super::dispatch::error;

/// Remove a session and its PTYs; the server exits when its last session closes.
pub(super) async fn close(context: &Arc<ServerContext>, name: &str) -> (ServerResponse, bool) {
    let closed = context.state.write().await.close_session(name);
    let windows = match closed {
        Ok(windows) => windows,
        Err(failure) => return (error(&failure.to_string()), false),
    };
    context.tasks.cancel_session(name);
    for window in windows {
        context.programs.stop(window);
    }
    if context.state.read().await.sessions.is_empty() {
        return (ServerResponse::ShuttingDown, true);
    }
    (persisted(context).await, false)
}

pub(super) async fn persisted(context: &ServerContext) -> ServerResponse {
    match context.persist().await {
        Ok(()) => super::dispatch::snapshot(context).await,
        Err(failure) => error(&failure.to_string()),
    }
}
