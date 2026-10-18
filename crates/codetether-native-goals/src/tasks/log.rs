//! Session-path adapter for the independent append-only goal journal.

use super::path::task_log_path;
use anyhow::Result;
use std::ops::Deref;
use std::path::PathBuf;

/// Journal with application-specific session-path resolution.
///
/// # Examples
/// ```
/// use codetether_native_goals::TaskLog;
/// let log = TaskLog::at("example.tasks.jsonl");
/// assert_eq!(log.path().to_str(), Some("example.tasks.jsonl"));
/// ```
pub struct TaskLog(codetether_goal_core::TaskLog);

impl TaskLog {
    /// Resolve the canonical session journal.
    ///
    /// # Errors
    /// Returns invalid-session-ID or storage-location errors.
    pub fn for_session(session_id: &str) -> Result<Self> {
        Ok(Self::at(task_log_path(session_id)?))
    }
    /// Open a journal at an explicit path without creating it.
    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self(codetether_goal_core::TaskLog::at(path))
    }
}
impl Deref for TaskLog {
    type Target = codetether_goal_core::TaskLog;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
