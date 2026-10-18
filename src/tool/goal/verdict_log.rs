//! Native session-path resolution for the shared verdict journal.
use anyhow::Result;
pub use codetether_goal_verification::VerdictRecord;
use std::path::PathBuf;

/// Path of the verdict log for `session_id`.
///
/// # Errors
///
/// Fails when the data directory cannot be determined.
pub fn path(session_id: &str) -> Result<PathBuf> {
    let tasks = crate::session::tasks::task_log_path(session_id)?;
    Ok(tasks.with_file_name(format!("{session_id}.verdicts.jsonl")))
}
/// Append a verdict to the current session's journal.
pub async fn append(session_id: &str, record: &VerdictRecord) -> Result<()> {
    codetether_goal_verification::journal::append(&path(session_id)?, record).await
}
/// Read the current session's verdict journal.
pub async fn read(session_id: &str) -> Result<Vec<VerdictRecord>> {
    codetether_goal_verification::journal::read(&path(session_id)?).await
}
