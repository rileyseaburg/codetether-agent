//! Native-source specialization retained for listing integration tests.
use crate::session::SessionSummary;
use anyhow::Result;
use std::path::PathBuf;
pub(super) async fn scan(
    sessions_dir: PathBuf,
    workspace: Option<PathBuf>,
) -> Result<Vec<SessionSummary>> {
    codetether_session_listing::scan::<super::host::ApplicationSource>(sessions_dir, workspace)
        .await
}
