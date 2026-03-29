use crate::tui::app::state::SessionEvent;
use tokio::sync::mpsc;

/// Placeholder UI event type for chat sync (not yet implemented).
#[derive(Debug)]
pub struct ChatSyncUiEvent {
    pub kind: String,
}

pub async fn run_chat_sync_worker(
    _tx: mpsc::UnboundedSender<SessionEvent>,
    _rx: mpsc::UnboundedReceiver<String>,
) {
    // Placeholder for sync worker logic
}
