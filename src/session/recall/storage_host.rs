//! Bind shared recall storage to native session paths and projections.
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;
#[derive(Default)]
pub(crate) struct NativeHost;
#[async_trait]
impl codetether_recall_store::Host for NativeHost {
    fn session_path(&self, id: &str) -> Result<PathBuf> {
        crate::session::Session::session_path(id)
    }
    fn root(&self) -> Result<PathBuf> {
        Ok(crate::session::Session::sessions_dir()?.join("recall"))
    }
    async fn projection(&self, id: &str) -> Result<Option<Value>> {
        crate::session::store::projection::read_session(id, "recall").await
    }
}
