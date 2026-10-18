//! Native session loading, listing and indexing callbacks for persistence.
use crate::session::Session;
use std::path::Path;
impl codetether_session_store::lifecycle::Host for Session {
    type Session = Session;
    async fn load_path(
        path: &Path,
        window: usize,
    ) -> anyhow::Result<codetether_session_store::TailLoad<Session>> {
        crate::session::store::load(path, window).await
    }
    async fn list_session_ids(workspace: Option<&Path>) -> anyhow::Result<Vec<String>> {
        let sessions = match workspace {
            Some(workspace) => {
                crate::session::listing::list_sessions_for_directory(workspace).await?
            }
            None => crate::session::list_sessions().await?,
        };
        Ok(sessions.into_iter().map(|session| session.id).collect())
    }
    fn schedule_recall(session: &Session) {
        crate::session::index::recall::schedule(session);
    }
    fn notify_saved(session: &Session) {
        crate::session::index_produce::notify::saved(session);
    }
    fn schedule_archive(session: &Session) {
        crate::session::store::archive::schedule(session);
    }
    async fn notify_removed(id: &str) {
        crate::session::index_produce::notify::removed(id).await;
    }
    async fn remove_recall(id: &str) -> anyhow::Result<()> {
        crate::session::index::recall::remove(id).await
    }
}
