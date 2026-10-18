//! Chat sync UI event types.

/// Status, error or successful batch reported by the background worker.
/// # Examples
/// ```
/// use codetether_chat_sync::ChatSyncUiEvent;
/// match ChatSyncUiEvent::Status("ready".into()) { ChatSyncUiEvent::Status(_) | ChatSyncUiEvent::Error(_) | ChatSyncUiEvent::BatchUploaded { .. } => {} }
/// ```
#[derive(Debug)]
pub enum ChatSyncUiEvent {
    /// Human-readable worker status.
    Status(String),
    /// Error from an upload attempt.
    Error(String),
    /// Completed upload metadata.
    BatchUploaded {
        bytes: u64,
        records: usize,
        object_key: String,
    },
}
