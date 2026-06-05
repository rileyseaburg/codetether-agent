use std::path::{Path, PathBuf};

use anyhow::Result;

use super::scan::scan;
use super::summary::SessionSummary;
use super::workspace::canonical;

/// List all persisted sessions.
///
/// # Errors
///
/// Returns an error if the data directory cannot be read.
pub async fn list_sessions() -> Result<Vec<SessionSummary>> {
    scan(sessions_dir()?, None).await
}

/// List sessions scoped to a workspace directory.
///
/// # Errors
///
/// Returns an error if the data directory cannot be read.
pub async fn list_sessions_for_directory(dir: &Path) -> Result<Vec<SessionSummary>> {
    scan(sessions_dir()?, Some(canonical(dir))).await
}

/// List workspace sessions with pagination.
///
/// # Errors
///
/// Returns an error if the data directory cannot be read.
pub async fn list_sessions_paged(
    dir: &Path,
    limit: usize,
    offset: usize,
) -> Result<Vec<SessionSummary>> {
    let sessions = list_sessions_for_directory(dir).await?;
    Ok(sessions.into_iter().skip(offset).take(limit).collect())
}

fn sessions_dir() -> Result<PathBuf> {
    crate::config::Config::data_dir()
        .map(|dir| dir.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
}
