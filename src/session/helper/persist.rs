//! Persistence helpers for live tool context.

use std::time::Instant;

use crate::session::Session;

/// Save the current session before executing a history-sensitive tool.
pub(super) async fn before_tool(session: &Session) -> Instant {
    let _ = session.save().await;
    Instant::now()
}
