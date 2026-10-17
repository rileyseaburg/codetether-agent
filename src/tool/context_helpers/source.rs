//! Adapt the existing persistence methods without changing loading behavior.
use crate::session::Session;
use anyhow::Result;
use std::path::Path;
#[async_trait::async_trait]
impl codetether_context_loader::Source for Session {
    async fn load_tail(id: &str, window: usize) -> Result<Self> {
        Session::load_tail(id, window)
            .await
            .map(|loaded| loaded.session)
    }
    async fn last_for_directory_tail(workspace: Option<&Path>, window: usize) -> Result<Self> {
        Session::last_for_directory_tail(workspace, window)
            .await
            .map(|loaded| loaded.session)
    }
    async fn last_for_directory(workspace: Option<&Path>) -> Result<Self> {
        Session::last_for_directory(workspace).await
    }
}
