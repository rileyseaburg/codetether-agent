//! Semantic steering routed through the authenticated mux daemon.

use anyhow::{Result, ensure};

use crate::mux::protocol::ServerResponse;

use super::context::ServerContext;

pub(super) async fn apply(context: &ServerContext, text: &str) -> Result<ServerResponse> {
    let state = context.state.read().await;
    let session_id = state
        .runtime
        .as_ref()
        .filter(|runtime| runtime.processing)
        .map(|runtime| runtime.session_id.clone());
    drop(state);
    let session_id = session_id.ok_or_else(|| anyhow::anyhow!("mux TUI is not processing"))?;
    let accepted = crate::session::helper::steering::send(&session_id, text).await?;
    ensure!(accepted, "active session stopped accepting steering");
    Ok(ServerResponse::Acknowledged)
}
