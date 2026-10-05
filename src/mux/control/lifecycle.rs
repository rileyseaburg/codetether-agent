//! Managed mux/TUI startup for the human-operated CLI.

use std::path::PathBuf;

use anyhow::Result;

use super::MuxSessionSummary;
use crate::mux::isolation::Isolation;

pub(crate) async fn start_managed_session(
    name: &str,
    workspace: PathBuf,
    session_id: Option<&str>,
    isolation: Isolation,
) -> Result<MuxSessionSummary> {
    super::start_session(name, workspace, isolation).await?;
    launch(name, session_id).await
}

async fn launch(name: &str, session_id: Option<&str>) -> Result<MuxSessionSummary> {
    super::lifecycle_launch::tui(name, session_id).await?;
    super::lifecycle_launch::wait_runtime(name).await
}
