//! Managed mux/TUI startup and in-place session rollover.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};

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

/// Recreate one session on its workspace server and relaunch its TUI.
///
/// Other sessions on the same server keep running; the server only restarts
/// when the rolled session was the last one it hosted.
pub(crate) async fn restart_session(
    name: &str,
    supplied_session: Option<&str>,
) -> Result<MuxSessionSummary> {
    let target = crate::mux::registry::load(name).await?;
    let current = target.session().context("mux session not found")?;
    let runtime = current.runtime.as_ref();
    if runtime.is_some_and(|item| item.processing) {
        bail!("refusing to roll a working mux session");
    }
    let session = supplied_session.or_else(|| runtime.map(|item| item.session_id.as_str()));
    let isolation = target.record.state.isolation;
    let workspace = current
        .active()
        .map(|item| item.workspace.clone())
        .context("mux workspace not found")?;
    super::stop_session(name).await?;
    super::lifecycle_restart::wait_stopped(name).await?;
    super::lifecycle_restart::start_exact(name, &workspace, isolation).await?;
    launch(name, session).await
}

async fn launch(name: &str, session_id: Option<&str>) -> Result<MuxSessionSummary> {
    super::lifecycle_launch::tui(name, session_id).await?;
    super::lifecycle_launch::wait_runtime(name).await
}
