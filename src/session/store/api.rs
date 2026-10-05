//! Explicit export and indexed history access; never invoked by ordinary saves.
use crate::provider::Message;
use crate::session::{Session, pages::PageKind};
use anyhow::Result;
impl Session {
    /// Resume an agent with a bounded working window and unchanged durable identity.
    ///
    /// # Arguments
    /// * `id` - Valid durable session identifier.
    /// # Returns
    /// The most recent working window; older records remain addressable on disk.
    /// # Errors
    /// Returns storage/import errors rather than discarding history.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let session = codetether_agent::session::Session::resume("session-id").await?;
    /// assert!(session.messages.len() <= codetether_agent::session::store::WINDOW);
    /// # Ok(()) }
    /// ```
    pub async fn resume(id: &str) -> Result<Self> {
        Ok(Self::load_tail(id, super::WINDOW).await?.session)
    }
    /// Export the complete durable transcript as legacy-compatible JSON.
    /// # Arguments
    /// * `id` - Session to export, including records outside the working window.
    /// # Returns
    /// A standalone JSON snapshot, not a database locator.
    ///
    /// # Errors
    /// Returns storage, decoding, or serialization errors. This explicit export
    /// is O(total transcript bytes); the normal save path never calls it.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let json = codetether_agent::session::Session::export_json("session-id").await?;
    /// assert!(serde_json::from_str::<serde_json::Value>(&json)?.is_object());
    /// # Ok(()) }
    /// ```
    pub async fn export_json(id: &str) -> Result<String> {
        let session = Self::load(id).await?;
        Ok(serde_json::to_string(&session)?)
    }
    /// Read at most `limit` messages starting at absolute sequence `start`.
    /// # Arguments
    /// * `id` - Durable session identifier.
    /// * `start` - Inclusive, zero-based absolute sequence.
    /// * `limit` - Maximum records to decode, zero for an empty result.
    /// # Returns
    /// Chronologically ordered messages; out-of-range requests return no records.
    ///
    /// # Errors
    /// Returns an error for missing sessions, corrupt records, or failed imports.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let page = codetether_agent::session::Session::read_messages("session-id", 100, 20).await?;
    /// assert!(page.len() <= 20);
    /// # Ok(()) }
    /// ```
    pub async fn read_messages(id: &str, start: usize, limit: usize) -> Result<Vec<Message>> {
        let path = Self::session_path(id)?;
        let id = id.to_owned();
        tokio::task::spawn_blocking(move || {
            super::migration::ensure(&path)?;
            let db = super::connection::open(&path)?;
            let rows = super::read::read_window(&db, &id, 0, start, start.saturating_add(limit))?;
            rows.into_iter()
                .map(|body| {
                    let (message, _): (Message, PageKind) = serde_json::from_str(&body)?;
                    anyhow::Ok(message)
                })
                .collect()
        })
        .await?
    }
    /// Absolute sequence corresponding to the first message in the working window.
    pub fn message_offset(&self) -> usize {
        self.storage.0.lock().unwrap().message_start
    }
    /// Total messages represented by durable history plus the current working tail.
    pub fn message_count(&self) -> usize {
        let state = self.storage.0.lock().unwrap();
        (state.message_start + self.messages.len()).max(state.message_end)
    }
}
