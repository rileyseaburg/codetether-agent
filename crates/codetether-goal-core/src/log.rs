//! Path-based append-only JSONL persistence, with no application dependency.

#[path = "log/read.rs"]
mod read;
#[path = "log/write.rs"]
mod write;

use std::path::{Path, PathBuf};

/// Handle to a newline-terminated, append-only task event journal.
///
/// # Examples
/// ```
/// use codetether_goal_core::TaskLog;
/// let log = TaskLog::at("goal.tasks.jsonl");
/// assert_eq!(log.path(), std::path::Path::new("goal.tasks.jsonl"));
/// ```
pub struct TaskLog {
    path: PathBuf,
}

impl TaskLog {
    /// Open a journal at an explicit path; creation is deferred until append.
    ///
    /// # Examples
    /// ```
    /// use codetether_goal_core::TaskLog;
    /// assert_eq!(TaskLog::at("task.jsonl").path().to_str(), Some("task.jsonl"));
    /// ```
    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Return the journal's storage path.
    ///
    /// # Examples
    /// ```
    /// use codetether_goal_core::TaskLog;
    /// assert!(TaskLog::at("task.jsonl").path().ends_with("task.jsonl"));
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }
}
