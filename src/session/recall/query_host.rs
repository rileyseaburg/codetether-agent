//! Native session loading and backfill hooks for shared recall queries.
use crate::session::Session;
use anyhow::Result;
use async_trait::async_trait;
use codetether_recall_index::IndexedSession;
use std::path::Path;
#[async_trait]
impl codetether_recall_query::Host for super::storage_host::NativeHost {
    type Session = Session;
    fn schedule_backfill(&self, workspace: &Path) {
        super::backfill::schedule(workspace);
    }
    async fn load_tail(&self, id: &str, limit: usize) -> Result<Session> {
        Ok(Session::load_tail(id, limit).await?.session)
    }
    fn build(session: &Session) -> Option<IndexedSession> {
        super::build::session(session)
    }
}
