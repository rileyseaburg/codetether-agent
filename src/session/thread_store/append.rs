//! Append events to a thread JSONL file.

use anyhow::{Context, Result};
use tokio::io::AsyncWriteExt;

use super::{ThreadEvent, ThreadStore};

impl ThreadStore {
    /// Append one event to its thread file.
    ///
    /// # Errors
    ///
    /// Returns an error when the thread id is unsafe, serialization fails, or
    /// the filesystem refuses the append.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::session::thread_store::{ThreadEvent, ThreadStore};
    ///
    /// let store = ThreadStore::new("/tmp/codetether-threads");
    /// let event = ThreadEvent {
    ///     event_id: "event-1".into(),
    ///     thread_id: "thread-1".into(),
    ///     turn_id: "turn-1".into(),
    ///     kind: "turn.started".into(),
    ///     timestamp_ms: 1,
    ///     payload: serde_json::json!({ "role": "user" }),
    /// };
    /// store.append(event).await.unwrap();
    /// # });
    /// ```
    pub async fn append(&self, event: ThreadEvent) -> Result<()> {
        let path = self.path_for(&event.thread_id)?;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .with_context(|| format!("create thread store directory {}", parent.display()))?;
        }

        let mut line = serde_json::to_vec(&event).context("serialize thread event")?;
        line.push(b'\n');
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await
            .with_context(|| format!("open thread event log {}", path.display()))?;
        file.write_all(&line)
            .await
            .with_context(|| format!("append thread event to {}", path.display()))
    }
}
