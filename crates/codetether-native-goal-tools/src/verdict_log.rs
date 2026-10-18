//! Native session-path resolution for the shared verdict journal.
use anyhow::Result;
pub use codetether_goal_verification::VerdictRecord;
use std::path::PathBuf;

/// Path of the verdict log for `session_id`.
///
/// # Errors
///
/// Fails when the data directory cannot be determined.
/// # Arguments
/// * session_id - Durable identity used for the sibling verdict journal.
/// # Returns
/// The canonical verdict-journal path.
/// # Examples
/// ```no_run
/// let path = codetether_native_goal_tools::verdict_log::path("session-id")?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn path(session_id: &str) -> Result<PathBuf> {
    let tasks = codetether_native_goals::task_log_path(session_id)?;
    Ok(tasks.with_file_name(format!("{session_id}.verdicts.jsonl")))
}
/// Append a verdict to the current session's journal.
/// # Arguments
/// * session_id - Durable session identity.
/// * record - Captured verifier decision and execution identity.
/// # Returns
/// Unit when the append completes.
/// # Errors
/// Propagates path-resolution and journal-write failures.
/// # Examples
/// ```no_run
/// # async fn example(record: &codetether_native_goal_tools::verdict_log::VerdictRecord) -> anyhow::Result<()> {
/// codetether_native_goal_tools::verdict_log::append("session-id", record).await?;
/// # Ok(()) }
/// ```
pub async fn append(session_id: &str, record: &VerdictRecord) -> Result<()> {
    codetether_goal_verification::journal::append(&path(session_id)?, record).await
}
/// Read the current session's verdict journal.
/// # Arguments
/// * session_id - Durable session identity.
/// # Returns
/// The stored verdicts in journal order.
/// # Errors
/// Propagates path-resolution and journal-read failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let records = codetether_native_goal_tools::verdict_log::read("session-id").await?;
/// # Ok(()) }
/// ```
pub async fn read(session_id: &str) -> Result<Vec<VerdictRecord>> {
    codetether_goal_verification::journal::read(&path(session_id)?).await
}
