//! Read, write, and remove individual session recall sidecars.

use anyhow::Result;

use super::indexed_session::IndexedSession;

pub(super) async fn read(session_id: &str) -> Option<IndexedSession> {
    let path = super::paths::session(session_id).ok()?;
    let indexed: IndexedSession = super::atomic::read(&path).await?;
    indexed.is_current_schema().then_some(indexed)
}

pub(super) async fn write(indexed: &IndexedSession) -> Result<()> {
    super::atomic::write(&super::paths::session(&indexed.session_id)?, indexed).await
}

pub(super) async fn remove(session_id: &str) -> Result<()> {
    let path = super::paths::session(session_id)?;
    match tokio::fs::remove_file(path).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}
