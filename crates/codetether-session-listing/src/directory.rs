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
/// # Arguments
/// Uses the directory resolved by the source.
/// # Returns
/// Matching summaries sorted by descending update time.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_listing::Source>() -> anyhow::Result<()> {
/// let rows = codetether_session_listing::list_sessions::<S>().await?;
/// assert!(rows.windows(2).all(|w| w[0].updated_at >= w[1].updated_at));
/// # Ok(()) }
/// ```
pub async fn list_sessions<S: crate::Source>() -> Result<Vec<SessionSummary>> {
    scan::<S>(sessions_dir::<S>()?, None).await
}

/// List sessions scoped to a workspace directory.
///
/// # Errors
///
/// Returns an error if the data directory cannot be read.
/// # Arguments
/// * `dir` — Workspace whose sessions should be listed.
/// # Returns
/// Matching summaries sorted by descending update time.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_listing::Source>() -> anyhow::Result<()> {
/// let rows = codetether_session_listing::list_sessions_for_directory::<S>(std::path::Path::new(".")).await?;
/// assert!(rows.windows(2).all(|w| w[0].updated_at >= w[1].updated_at));
/// # Ok(()) }
/// ```
pub async fn list_sessions_for_directory<S: crate::Source>(
    dir: &Path,
) -> Result<Vec<SessionSummary>> {
    scan::<S>(sessions_dir::<S>()?, Some(canonical(dir))).await
}

/// List workspace sessions with pagination.
///
/// # Errors
///
/// Returns an error if the data directory cannot be read.
/// # Arguments
/// * `dir` — Workspace filter.
/// * `limit` — Maximum rows to return.
/// * `offset` — Rows to skip after sorting.
/// # Returns
/// Matching summaries sorted by descending update time.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_listing::Source>() -> anyhow::Result<()> {
/// let rows = codetether_session_listing::list_sessions_paged::<S>(std::path::Path::new("."), 10, 0).await?;
/// assert!(rows.windows(2).all(|w| w[0].updated_at >= w[1].updated_at));
/// # Ok(()) }
/// ```
pub async fn list_sessions_paged<S: crate::Source>(
    dir: &Path,
    limit: usize,
    offset: usize,
) -> Result<Vec<SessionSummary>> {
    let sessions = list_sessions_for_directory::<S>(dir).await?;
    Ok(sessions.into_iter().skip(offset).take(limit).collect())
}

fn sessions_dir<S: crate::Source>() -> Result<PathBuf> {
    S::sessions_dir()
}
