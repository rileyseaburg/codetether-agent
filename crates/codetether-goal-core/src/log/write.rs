//! Single-event JSONL appends for async and synchronous callers.

use super::TaskLog;
use crate::TaskEvent;
use anyhow::{Context, Result};
use tokio::io::AsyncWriteExt;

impl TaskLog {
    /// Append one newline-terminated event and flush the stream.
    ///
    /// # Errors
    /// Returns serialization, directory creation, open, or write errors.
    pub async fn append(&self, event: &TaskEvent) -> Result<()> {
        if let Some(parent) = self
            .path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
        {
            tokio::fs::create_dir_all(parent).await?;
        }
        let mut line = serde_json::to_string(event).context("serialize task event")?;
        line.push('\n');
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await
            .with_context(|| format!("open {} for append", self.path.display()))?;
        file.write_all(line.as_bytes()).await?;
        file.flush().await?;
        file.sync_data().await?;
        Ok(())
    }

    /// Append one event without requiring a Tokio runtime.
    ///
    /// # Errors
    /// Returns serialization, directory creation, open, or write errors.
    pub fn append_blocking(&self, event: &TaskEvent) -> Result<()> {
        use std::io::Write;
        if let Some(parent) = self
            .path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)?;
        }
        let mut line = serde_json::to_string(event).context("serialize task event")?;
        line.push('\n');
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .with_context(|| format!("open {} for append", self.path.display()))?;
        file.write_all(line.as_bytes())?;
        file.sync_data()?;
        Ok(())
    }
}
