//! Semantic steering routed through the authenticated mux daemon.

use anyhow::{Result, ensure};

use codetether_mux_protocol::ServerResponse;

use super::context::ServerContext;

pub(super) async fn apply(
    context: &ServerContext,
    session: &str,
    text: &str,
) -> Result<ServerResponse> {
    let state = context.state.read().await;
    let session_id = state
        .session(session)
        .and_then(|item| item.runtime.as_ref())
        .filter(|runtime| runtime.processing)
        .map(|runtime| runtime.session_id.clone());
    drop(state);
    let session_id = session_id.ok_or_else(|| anyhow::anyhow!("mux TUI is not processing"))?;
    let accepted = codetether_session_steering::send(&session_id, text).await?;
    ensure!(accepted, "active session stopped accepting steering");
    Ok(ServerResponse::Acknowledged)
}
