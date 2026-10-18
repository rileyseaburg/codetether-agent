//! Native session selection operations for CLI runs.
use crate::session::{Session, import_codex_session_by_id};
use std::path::Path;
#[async_trait::async_trait(?Send)]
impl codetether_run_resume::SessionSource for Session {
    fn id(&self) -> &str {
        &self.id
    }
    async fn create() -> anyhow::Result<Self> {
        Session::new().await
    }
    async fn import_codex(id: &str) -> anyhow::Result<Self> {
        import_codex_session_by_id(id).await
    }
    async fn resume(id: &str) -> anyhow::Result<Self> {
        Session::resume(id).await
    }
    async fn resume_last(workspace: Option<&Path>) -> anyhow::Result<Self> {
        Session::resume_last(workspace).await
    }
}
