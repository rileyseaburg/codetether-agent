//! Indexed workspace listing and transactional recall worker cursors.
use crate::{Host, Session};
use codetether_session_store::{Delivery, Ticket};
#[async_trait::async_trait]
impl<S: Session> codetether_recall_worker::Host for Host<S> {
    async fn list_sessions(&self, workspace: &std::path::Path) -> anyhow::Result<Vec<String>> {
        Ok(codetether_session_listing::list_sessions_for_directory::<
            codetether_session_listing::indexed::IndexedSource<S>,
        >(workspace)
        .await?
        .into_iter()
        .map(|summary| summary.id)
        .collect())
    }
    async fn next(
        &self,
        id: &str,
        consumer: &str,
        limit: usize,
    ) -> anyhow::Result<Option<Delivery<S>>> {
        codetether_session_store::consumer::next_for::<S>(id, consumer, limit).await
    }
    async fn commit(&self, ticket: Ticket, view: serde_json::Value) -> anyhow::Result<bool> {
        codetether_session_store::projection::commit(ticket, view).await
    }
}
