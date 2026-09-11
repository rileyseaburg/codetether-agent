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
use crate::mux::isolation::Isolation;
use crate::mux::registry::SessionTarget;

/// Start (or join) a session named `name` for `workspace`.
pub(crate) async fn start_session(
    name: &str,
    workspace: PathBuf,
    isolation: Isolation,
) -> Result<MuxSessionSummary> {
    let target = start_target(name, workspace, isolation).await?;
    Ok(MuxSessionSummary::from_target(&target, true))
}

pub(in crate::mux) async fn start_target(
    name: &str,
    workspace: PathBuf,
    isolation: Isolation,
) -> Result<SessionTarget> {
    crate::mux::registry::validate_name(name)?;
    crate::mux::command::startup::reject_duplicate(name).await?;
    let workspace = crate::mux::isolation::workspace(name, 0, &workspace, isolation)
        .await
        .context("resolve initial mux agent workspace")?;
    if let Some(record) = super::start_join::live_server(&workspace).await {
        return super::start_join::create_session(record, name, workspace).await;
    }
    let token = crate::mux::token::generate();
    let mut process = crate::mux::command::spawn::command(name, &workspace, &token, isolation)?;
    let mut child = process.spawn().context("start mux server")?;
    crate::mux::command::startup::wait_for_record(name, &mut child).await
}
