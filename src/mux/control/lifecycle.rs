//! Managed mux/TUI startup and in-place rollover.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};

use super::MuxSessionSummary;

pub(crate) async fn start_managed_session(
    name: &str,
    workspace: PathBuf,
    session_id: Option<&str>,
) -> Result<MuxSessionSummary> {
    super::start_session(name, workspace).await?;
    launch(name, session_id).await
}

pub(crate) async fn restart_session(
    name: &str,
    supplied_session: Option<&str>,
) -> Result<MuxSessionSummary> {
    let record = super::agent_target::load(name)
        .await?
        .context("mux session not found")?;
    let runtime = record.state.runtime.as_ref();
    if runtime.is_some_and(|item| item.processing) {
        bail!("refusing to roll a working mux session");
    }
    let session = supplied_session.or_else(|| runtime.map(|item| item.session_id.as_str()));
    let workspace = record
        .state
        .windows
        .iter()
        .find(|item| item.id == record.state.active_window)
        .map(|item| item.workspace.clone())
        .context("mux workspace not found")?;
    super::stop_session(name).await?;
    super::lifecycle_restart::wait_stopped(name).await?;
    super::lifecycle_restart::start_exact(name, &workspace).await?;
    launch(name, session).await
}

async fn launch(name: &str, session_id: Option<&str>) -> Result<MuxSessionSummary> {
    super::lifecycle_launch::tui(name, session_id).await?;
    super::lifecycle_launch::wait_runtime(name).await
}
