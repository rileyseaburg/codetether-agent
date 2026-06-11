//! Read events from a thread JSONL file.

use anyhow::{Context, Result};

use super::{ThreadEvent, ThreadStore};

impl ThreadStore {
    /// Read every event for `thread_id` in append order.
    ///
    /// # Errors
    ///
    /// Returns an error when the thread id is unsafe, the file cannot be read,
    /// or any JSONL record is malformed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::session::thread_store::ThreadStore;
    ///
    /// let store = ThreadStore::new("/tmp/codetether-threads");
    /// let events = store.read_thread("thread-1").await.unwrap();
    /// assert!(events.len() <= usize::MAX);
    /// # });
    /// ```
    pub async fn read_thread(&self, thread_id: &str) -> Result<Vec<ThreadEvent>> {
        let path = self.path_for(thread_id)?;
        let data = match tokio::fs::read_to_string(&path).await {
            Ok(data) => data,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(err).with_context(|| format!("read {}", path.display())),
        };
        data.lines()
            .enumerate()
            .map(|(idx, line)| parse_line(line, idx))
            .collect()
    }
}

fn parse_line(line: &str, idx: usize) -> Result<ThreadEvent> {
    serde_json::from_str(line).with_context(|| format!("decode thread event line {}", idx + 1))
}
