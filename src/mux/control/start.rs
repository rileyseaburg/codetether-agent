//! Detached mux server startup shared by command surfaces.

use std::path::PathBuf;

use anyhow::{Context, Result};

use super::MuxSessionSummary;
use crate::mux::registry::MuxRecord;

/// Start a detached named mux server rooted at `workspace`.
pub(crate) async fn start_session(name: &str, workspace: PathBuf) -> Result<MuxSessionSummary> {
    let record = start_record(name, workspace).await?;
    Ok(MuxSessionSummary::from_record(&record, true))
}

pub(in crate::mux) async fn start_record(name: &str, workspace: PathBuf) -> Result<MuxRecord> {
    crate::mux::registry::validate_name(name)?;
    crate::mux::command::startup::reject_duplicate(name).await?;
    let workspace = tokio::fs::canonicalize(workspace)
        .await
        .context("resolve initial workspace")?;
    let token = crate::mux::token::generate();
    let mut process = crate::mux::command::spawn::command(name, &workspace, &token)?;
    let mut child = process.spawn().context("start mux server")?;
    crate::mux::command::startup::wait_for_record(name, &mut child).await
}
