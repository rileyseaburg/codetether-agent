//! Strict ordered JSONL reads; damaged governance is never silently skipped.

use super::TaskLog;
use crate::TaskEvent;
use anyhow::{Context, Result, ensure};

impl TaskLog {
    /// Read events in insertion order, allowing blank lines only.
    ///
    /// # Errors
    /// Returns I/O or malformed/incomplete-event errors; a missing journal is empty.
    pub async fn read_all(&self) -> Result<Vec<TaskEvent>> {
        match tokio::fs::read_to_string(&self.path).await {
            Ok(text) => decode(&text),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => Err(error.into()),
        }
    }

    /// Read events without requiring a Tokio runtime.
    ///
    /// # Errors
    /// Returns I/O or malformed/incomplete-event errors; a missing journal is empty.
    pub fn read_all_blocking(&self) -> Result<Vec<TaskEvent>> {
        match std::fs::read_to_string(&self.path) {
            Ok(text) => decode(&text),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(error.into()),
        }
    }
}

fn decode(text: &str) -> Result<Vec<TaskEvent>> {
    ensure!(
        text.is_empty() || text.ends_with('\n'),
        "goal journal has an incomplete final event"
    );
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            serde_json::from_str(line)
                .with_context(|| format!("invalid goal journal event on line {}", index + 1))
        })
        .collect()
}
