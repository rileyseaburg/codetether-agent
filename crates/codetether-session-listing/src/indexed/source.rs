//! Standard session location and indexed metadata query bindings.
use super::{Header, IndexedSource, header::decode};
use crate::Source;
use anyhow::Result;
use async_trait::async_trait;
use codetether_session_metadata::SessionSummary;
use std::path::{Path, PathBuf};
#[async_trait]
impl<H: Header> Source for IndexedSource<H> {
    const INDEX_FILENAME: &'static str = codetether_workspace_index::INDEX_FILENAME;
    fn sessions_dir() -> Result<PathBuf> {
        codetether_session_location::sessions_dir()
    }
    async fn indexed_sessions(
        root: &Path,
        workspace: Option<PathBuf>,
    ) -> Result<Vec<SessionSummary>> {
        codetether_session_store::metadata::listing::list(root, workspace, decode::<H>).await
    }
}
