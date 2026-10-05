//! Explicit external imports replace a session only against its current revision.
use crate::session::Session;
use anyhow::Result;
use std::path::Path;
pub(crate) async fn persist(mut session: Session, path: &Path) -> Result<bool> {
    if path.exists() {
        let previous = super::load(path, 0).await?.session;
        if previous.updated_at >= session.updated_at {
            return Ok(false);
        }
        let mut checkpoint = previous.storage.0.lock().unwrap().clone();
        checkpoint.message_start = 0;
        checkpoint.tool_start = 0;
        session.storage = super::State(std::sync::Mutex::new(checkpoint));
    }
    super::save(&session, path).await
}
