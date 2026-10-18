//! Managed mux/TUI startup for the human-operated CLI.

use std::path::PathBuf;

use anyhow::Result;

use super::MuxSessionSummary;
use codetether_mux_workspace::Isolation;

/// Start or join a session and launch its semantic TUI.
///
/// # Arguments
/// name identifies the session; workspace is its directory; session_id optionally resumes a conversation; isolation selects workspace allocation.
/// # Returns
/// The session summary once its semantic runtime is registered.
/// # Errors
/// Returns startup, invalid conversation ID, TUI launch or registration timeout errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let session = codetether_mux_lifecycle::start_managed_session("work", ".".into(), None, codetether_mux_workspace::Isolation::Shared).await?;
/// # Ok(()) }
/// ```
pub async fn start_managed_session(
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
