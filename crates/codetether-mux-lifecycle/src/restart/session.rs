//! Restore one idle mux session on its existing workspace server.
use crate::MuxSessionSummary;
use anyhow::{Context, Result, bail};
/// Recreate one session on its workspace server and relaunch its TUI.
///
/// Other sessions on the same server keep running; the server only restarts
/// when the rolled session was the last one it hosted.
/// # Arguments
/// Receives the exact mux name and optional durable session to resume.
/// # Returns
/// The recreated session after its semantic runtime registers.
/// # Errors
/// Returns missing session/workspace, working-session, stop/start or launch errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _ = codetether_mux_lifecycle::restart::restart_session("work", None).await?;
/// # Ok(()) }
/// ```
pub async fn restart_session(
    name: &str,
    supplied_session: Option<&str>,
) -> Result<MuxSessionSummary> {
    let target = codetether_mux_registry::load(name).await?;
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
    codetether_mux_control::stop_session(name).await?;
    super::wait_stopped(name).await?;
    super::start_exact(name, &workspace, isolation).await?;
    crate::lifecycle_launch::tui(name, session).await?;
    crate::lifecycle_launch::wait_runtime(name).await
}
