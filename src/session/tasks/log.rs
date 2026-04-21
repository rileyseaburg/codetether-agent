//! Append-only JSONL IO for the session task log.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use super::event::TaskEvent;
use super::path::task_log_path;

/// Handle to a session's task log file.
///
/// All writes append a single JSON line; reads return events in
/// insertion order. The log is intentionally tiny and synchronous
/// at the line level — one event per line, newline-terminated.
pub struct TaskLog {
    path: PathBuf,
}

impl TaskLog {
    /// Open (or lazily create) the log for the given session id.
    pub fn for_session(session_id: &str) -> Result<Self> {
        Ok(Self {
            path: task_log_path(session_id)?,
        })
    }

    /// Open a log at an explicit path (used by tests).
    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Path to the underlying `.tasks.jsonl` file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Append a single event as a JSON line.
    pub async fn append(&self, event: &TaskEvent) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).await.ok();
        }
        let mut line = serde_json::to_string(event).context("serialize task event")?;
        line.push('\n');
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await
            .with_context(|| format!("open {} for append", self.path.display()))?;
        f.write_all(line.as_bytes()).await?;
        f.flush().await?;
        Ok(())
    }

    /// Read every event, skipping malformed lines.
    pub async fn read_all(&self) -> Result<Vec<TaskEvent>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let file = fs::File::open(&self.path).await?;
        let mut out = Vec::new();
        let mut lines = BufReader::new(file).lines();
        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(event) = serde_json::from_str::<TaskEvent>(&line) {
                out.push(event);
            }
        }
        Ok(out)
    }

    /// Blocking read — suitable for sync contexts like system-prompt
    /// assembly, which runs on each turn and must not depend on a
    /// tokio runtime. The log is small (goal + open tasks), so this
    /// is cheap in practice.
    pub fn read_all_blocking(&self) -> Result<Vec<TaskEvent>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(&self.path)?;
        let mut out = Vec::new();
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(event) = serde_json::from_str::<TaskEvent>(line) {
                out.push(event);
            }
        }
        Ok(out)
    }

    /// Blocking append — used by the TUI slash-command handler which
    /// runs inside an async context but wants a single synchronous write.
    pub fn append_blocking(&self, event: &TaskEvent) -> Result<()> {
        use std::io::Write;
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let mut line = serde_json::to_string(event).context("serialize task event")?;
        line.push('\n');
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .with_context(|| format!("open {} for append", self.path.display()))?;
        f.write_all(line.as_bytes())?;
        Ok(())
    }
}
