//! Native directory and indexed-summary inputs for shared listing.
use anyhow::Result;
use async_trait::async_trait;
use codetether_session_metadata::SessionSummary;
use std::path::{Path, PathBuf};
/// Application sources combined with legacy session files.
/// # Implementors
/// Delegate to existing directory resolution and metadata-only database queries.
#[async_trait]
pub trait Source: Send + Sync {
    /// Filename of the native workspace index, excluded from session JSON files.
    const INDEX_FILENAME: &'static str;
    /// Resolve the existing session storage directory.
    /// # Errors
    /// Propagates application directory resolution failures.
    fn sessions_dir() -> Result<PathBuf>;
    /// Read indexed session summaries, propagating the native query errors.
    /// # Arguments
    /// * `root` — Session storage directory.
    /// * `workspace` — Optional canonical workspace filter.
    /// # Returns
    /// Indexed summaries selected by the application.
    /// # Errors
    /// Propagates index query failures.
    async fn indexed_sessions(
        root: &Path,
        workspace: Option<PathBuf>,
    ) -> Result<Vec<SessionSummary>>;
}
