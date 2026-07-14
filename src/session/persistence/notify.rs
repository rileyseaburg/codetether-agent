//! Persistence-boundary notifications for proactive RLM preparation.

use std::path::Path;

use crate::session::Session;

pub(crate) fn is_unchanged(session: &Session, content: &[u8], path: &Path) -> bool {
    let unchanged = super::is_unchanged(&session.id, content, path);
    if unchanged {
        saved(session);
    }
    unchanged
}

pub(crate) fn record_saved(session: &Session, content: &[u8]) {
    super::record_saved(&session.id, content);
    saved(session);
}

fn saved(session: &Session) {
    crate::session::index_produce::notify::saved(session);
    crate::session::index::recall::schedule(session);
}

pub(crate) async fn forget(session_id: &str) -> anyhow::Result<()> {
    super::forget(session_id);
    crate::session::index_produce::notify::removed(session_id).await;
    crate::session::index::recall::remove(session_id).await
}
