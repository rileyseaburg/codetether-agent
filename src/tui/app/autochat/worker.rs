//! Thin wrapper to start an autochat relay worker thread.

use tokio::sync::mpsc;

use super::events::AutochatUiEvent;

/// Start the autochat relay in a background task.
///
/// Returns the receiver end of the UI event channel. The caller
/// (event loop) drains events from this channel on each tick.
pub fn start_autochat_relay(
    task: String,
    model: String,
) -> mpsc::UnboundedReceiver<AutochatUiEvent> {
    let (ui_tx, ui_rx) = mpsc::unbounded_channel();
    let _ = ui_tx.send(AutochatUiEvent::Progress(format!(
        "Starting autochat relay for: {task}"
    )));
    tokio::spawn(async move {
        super::run::run_relay(task, model, ui_tx).await;
    });
    ui_rx
}
