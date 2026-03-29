use crate::tui::app::state::SessionEvent;
use tokio::sync::mpsc;

/// UI-facing event emitted by the background chat-sync worker.
#[derive(Debug, Clone)]
pub enum ChatSyncUiEvent {
    /// A batch was successfully uploaded.
    Uploaded { batches: u64, bytes: u64 },
    /// An error occurred during sync.
    Error(String),
    /// Sync status update.
    Status(String),
}

/// Called from the event loop when a [`ChatSyncUiEvent`] arrives.
pub fn handle_chat_sync_event(
    state: &mut crate::tui::app::state::AppState,
    evt: ChatSyncUiEvent,
) {
    match evt {
        ChatSyncUiEvent::Uploaded { batches, bytes } => {
            state.chat_sync_uploaded_batches += batches;
            state.chat_sync_uploaded_bytes += bytes;
            state.chat_sync_last_success =
                Some(chrono::Local::now().format("%H:%M:%S").to_string());
            state.chat_sync_status =
                Some(format!("synced {batches} batches, {bytes} bytes"));
        }
        ChatSyncUiEvent::Error(e) => {
            state.chat_sync_last_error = Some(e);
            state.chat_sync_status = Some("sync error".to_string());
        }
        ChatSyncUiEvent::Status(s) => {
            state.chat_sync_status = Some(s);
        }
    }
}

pub async fn run_chat_sync_worker(
    _tx: mpsc::UnboundedSender<SessionEvent>,
    _rx: mpsc::UnboundedReceiver<String>,
) {
    // Placeholder for sync worker logic
}
