//! Persistence of semantic state reported by the mux-owned TUI.

use crate::mux::model::MuxRuntimeStatus;
use crate::mux::protocol::ServerResponse;

use super::context::ServerContext;

pub(super) async fn apply(
    context: &ServerContext,
    status: Option<MuxRuntimeStatus>,
) -> ServerResponse {
    context.state.write().await.runtime = status;
    match context.persist().await {
        Ok(()) => ServerResponse::Acknowledged,
        Err(error) => ServerResponse::Error {
            message: error.to_string(),
        },
    }
}
