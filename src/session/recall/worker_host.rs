//! Native listing, cursor delivery and projection commit bindings.
use crate::session::Session;
use anyhow::Result;
use async_trait::async_trait;
use codetether_session_store::{Delivery, Ticket};
use serde_json::Value;
use std::path::Path;
#[async_trait]
impl codetether_recall_worker::Host for super::storage_host::NativeHost {
    async fn list_sessions(&self, workspace: &Path) -> Result<Vec<String>> {
        Ok(
            crate::session::listing::list_sessions_for_directory(workspace)
                .await?
                .into_iter()
                .map(|summary| summary.id)
                .collect(),
        )
    }
    async fn next(
        &self,
        id: &str,
        consumer: &str,
        limit: usize,
    ) -> Result<Option<Delivery<Session>>> {
        crate::session::store::cursor::next(id, consumer, limit).await
    }
    async fn commit(&self, ticket: Ticket, view: Value) -> Result<bool> {
        crate::session::store::projection::commit(ticket, view).await
    }
}
