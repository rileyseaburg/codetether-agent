//! Persistence helpers for live tool context.

use std::time::Instant;

use crate::session::Session;

/// Save the current session before executing a history-sensitive tool.
pub(super) async fn before_tool(session: &Session) -> anyhow::Result<Instant> {
    session.save().await?;
    Ok(Instant::now())
}

/// Persist a completed tool result before the loop advances.
pub(super) async fn after_tool(session: &Session) -> anyhow::Result<()> {
    session.save().await
}
