//! Persistence of semantic state reported by a session's mux-owned TUI.

use codetether_mux_model::MuxRuntimeStatus;
use codetether_mux_protocol::ServerResponse;

use super::context::ServerContext;

pub(super) async fn apply(
    context: &ServerContext,
    session: &str,
    status: Option<MuxRuntimeStatus>,
) -> ServerResponse {
    {
        let mut state = context.state.write().await;
        let Some(item) = state.session_mut(session) else {
            return ServerResponse::Error {
                message: format!("unknown mux session '{session}'"),
            };
        };
        item.runtime = status;
    }
    match context.persist().await {
        Ok(()) => ServerResponse::Acknowledged,
        Err(error) => ServerResponse::Error {
            message: error.to_string(),
        },
    }
}
