//! Ordered JSONL reads, retaining compatibility with malformed-line skipping.

use super::TaskLog;
use crate::TaskEvent;
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};

impl TaskLog {
    /// Read events in insertion order, skipping blank or malformed lines.
    ///
    /// # Errors
    /// Returns IO errors; a missing journal is an empty event stream.
    pub async fn read_all(&self) -> Result<Vec<TaskEvent>> {
        let file = match tokio::fs::File::open(&self.path).await {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(error.into()),
        };
        let mut events = Vec::new();
        let mut lines = BufReader::new(file).lines();
        while let Some(line) = lines.next_line().await? {
            if let Ok(event) = serde_json::from_str::<TaskEvent>(&line) {
                events.push(event);
            }
        }
        Ok(events)
    }

    /// Read events without requiring a Tokio runtime.
    ///
    /// # Errors
    /// Returns IO errors; a missing journal is an empty event stream.
    pub fn read_all_blocking(&self) -> Result<Vec<TaskEvent>> {
        match std::fs::read_to_string(&self.path) {
            Ok(text) => Ok(text
                .lines()
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(error.into()),
        }
    }
}
