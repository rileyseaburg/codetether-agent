//! Native loading, listing and ordered persistence notifications.
use crate::{Host, Session};
use codetether_session_listing::indexed::IndexedSource;
use codetether_session_store::TailLoad;
use std::path::Path;
impl<S: Session> codetether_session_store::lifecycle::Host for Host<S> {
    type Session = S;
    async fn load_path(path: &Path, window: usize) -> anyhow::Result<TailLoad<S>> {
        crate::load::<S>(path, window).await
    }
    async fn list_session_ids(workspace: Option<&Path>) -> anyhow::Result<Vec<String>> {
        let sessions = match workspace {
            Some(workspace) => {
                codetether_session_listing::list_sessions_for_directory::<IndexedSource<S>>(
                    workspace,
                )
                .await?
            }
            None => codetether_session_listing::list_sessions::<IndexedSource<S>>().await?,
        };
        Ok(sessions.into_iter().map(|session| session.id).collect())
    }
    fn schedule_recall(session: &S) {
        codetether_native_recall::queue::schedule(session);
    }
    fn notify_saved(session: &S) {
        codetether_native_proactive::schedule(session);
    }
    fn schedule_archive(session: &S) {
        crate::archive::schedule(session);
    }
    async fn notify_removed(id: &str) {
        codetether_native_proactive::remove::<S>(id).await;
    }
    async fn remove_recall(id: &str) -> anyhow::Result<()> {
        codetether_native_recall::queue::remove::<S>(id).await
    }
}
