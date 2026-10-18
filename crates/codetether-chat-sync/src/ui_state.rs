//! Frontend-owned receiver and presentation values for the archive worker.
use crate::ChatSyncUiEvent;
use tokio::sync::mpsc::UnboundedReceiver;

/// Chat synchronization state retained by an interactive frontend.
///
/// This container holds the event receiver and presentation values. Creating
/// it does not start a worker or consume events; the frontend controls those
/// lifecycles independently.
///
/// # Examples
/// ```
/// let state = codetether_chat_sync::ChatSyncState::default();
/// assert!(state.receiver.is_none());
/// assert!(state.status.is_none());
/// assert_eq!(state.uploaded_bytes, 0);
/// assert_eq!(state.uploaded_batches, 0);
/// ```
#[derive(Default)]
pub struct ChatSyncState {
    /// Receiver for worker events, when connected by the frontend.
    pub receiver: Option<UnboundedReceiver<ChatSyncUiEvent>>,
    /// Latest human-readable worker status.
    pub status: Option<String>,
    /// Latest successful upload description retained by the frontend.
    pub last_success: Option<String>,
    /// Latest upload error description retained by the frontend.
    pub last_error: Option<String>,
    /// Number of uploaded bytes recorded by the frontend.
    pub uploaded_bytes: u64,
    /// Number of uploaded batches recorded by the frontend.
    pub uploaded_batches: u64,
}
