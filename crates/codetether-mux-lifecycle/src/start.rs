//! Session startup shared by command surfaces: join the workspace's server
//! when one is running, otherwise start one.
//!
//! Managed-worktree sessions always allocate a fresh checkout, so they always
//! start a new server. Shared (`--no-worktree`) sessions resolve the requested
//! checkout first; when a live server already owns it the session is created
//! there, keeping one server — and one lease table — per workspace.

use std::path::PathBuf;

use anyhow::{Context, Result};

use super::MuxSessionSummary;
use codetether_mux_registry::SessionTarget;
use codetether_mux_workspace::Isolation;

/// Start or join a named session for a workspace.
///
/// # Arguments
/// name is the unique session name; workspace is its initial directory; isolation selects workspace allocation.
/// # Returns
/// A summary of the discovered or started session.
/// # Errors
/// Returns validation, workspace, registry or process startup errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let session = codetether_mux_lifecycle::start_session("work", ".".into(), codetether_mux_workspace::Isolation::Shared).await?;
/// # Ok(()) }
/// ```
pub async fn start_session(
    name: &str,
    workspace: PathBuf,
    isolation: Isolation,
) -> Result<MuxSessionSummary> {
    let target = start_target(name, workspace, isolation).await?;
    Ok(MuxSessionSummary::from_target(&target, true))
}

/// Start or join a session and return its authenticated registry target.
///
/// # Arguments
/// name identifies the session; workspace is its directory; isolation selects workspace allocation.
/// # Returns
/// The server record and selected session.
/// # Errors
/// Returns validation, workspace, registry or process startup errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let target = codetether_mux_lifecycle::start_target("work", ".".into(), codetether_mux_workspace::Isolation::Shared).await?;
/// # Ok(()) }
/// ```
pub async fn start_target(
    name: &str,
    workspace: PathBuf,
    isolation: Isolation,
) -> Result<SessionTarget> {
    codetether_mux_registry::validate_name(name)?;
    crate::startup::reject_duplicate(name).await?;
    let workspace = codetether_mux_workspace::workspace(name, 0, &workspace, isolation)
        .await
        .context("resolve initial mux agent workspace")?;
    if let Some(record) = super::start_join::live_server(&workspace).await {
        return super::start_join::create_session(record, name, workspace).await;
    }
    let token = codetether_mux_server::token::generate();
    let mut process = crate::spawn::command(name, &workspace, &token, isolation)?;
    let mut child = process.spawn().context("start mux server")?;
    crate::startup::wait_for_record(name, &mut child).await
}
