//! Query migration and opt-in backfill using the native recall worker.
use crate::{Host, Session};
#[async_trait::async_trait]
impl<S: Session> codetether_recall_query::Host for Host<S> {
    type Session = S;
    fn schedule_backfill(&self, workspace: &std::path::Path) {
        codetether_recall_worker::schedule_backfill(
            workspace,
            std::sync::Arc::new(Self::default()),
        );
    }
    async fn load_tail(&self, id: &str, limit: usize) -> anyhow::Result<S> {
        S::load_tail(id, limit).await
    }
    fn build(session: &S) -> Option<codetether_recall_index::IndexedSession> {
        crate::build::session(session)
    }
}
