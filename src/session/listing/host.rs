//! Native directory, metadata index and filename bindings for shared listings.
use crate::session::{Session, SessionSummary};
use anyhow::Result;
use async_trait::async_trait;
use codetether_session_listing::Source;
use std::path::{Path, PathBuf};
pub(in crate::session) struct ApplicationSource;
#[async_trait]
impl Source for ApplicationSource {
    const INDEX_FILENAME: &'static str = crate::session::workspace_index::INDEX_FILENAME;
    fn sessions_dir() -> Result<PathBuf> {
        Session::sessions_dir()
    }
    async fn indexed_sessions(
        root: &Path,
        workspace: Option<PathBuf>,
    ) -> Result<Vec<SessionSummary>> {
        crate::session::store::listing::list(root, workspace).await
    }
}
