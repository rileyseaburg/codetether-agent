use tokio::sync::mpsc;
use crate::tui::app::state::SessionEvent;

pub async fn run_chat_sync_worker(
    _tx: mpsc::UnboundedSender<SessionEvent>,
    _rx: mpsc::UnboundedReceiver<String>,
) {
    // Placeholder for sync worker logic
}
