//! Canonical session paths and materialized recall projections.
use crate::{Host, Session};
use std::path::PathBuf;
#[async_trait::async_trait]
impl<S: Session> codetether_recall_store::Host for Host<S> {
    fn session_path(&self, id: &str) -> anyhow::Result<PathBuf> {
        codetether_session_store::lifecycle::session_path(id)
    }
    fn root(&self) -> anyhow::Result<PathBuf> {
        Ok(codetether_session_store::lifecycle::sessions_dir()?.join("recall"))
    }
    async fn projection(&self, id: &str) -> anyhow::Result<Option<serde_json::Value>> {
        codetether_session_store::projection::read_session(id, "recall").await
    }
}
