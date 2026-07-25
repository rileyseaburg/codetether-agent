//! Persistence helpers for live tool context.

use std::time::Instant;

use crate::session::Session;

/// Save the current session before executing a history-sensitive tool.
pub(super) async fn before_tool(session: &Session) -> Instant {
    let _ = session.save().await;
    Instant::now()
}

/// Persist a completed tool result before the loop advances.
pub(super) async fn after_tool(session: &Session) {
    if let Err(error) = session.save().await {
        tracing::warn!(
            session_id = %session.id,
            error = %error,
            "Failed to persist completed tool result"
        );
    }
}
