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
    let (tx, rx) = mpsc::unbounded_channel();
    let _ = tx.send(AutochatUiEvent::Progress(format!(
        "Starting autochat relay for: {task}"
    )));
    // TODO: wire to relay worker once autochat module exposes a public entry point.
    // For now the relay completes immediately with a stub.
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let _ = tx_clone.send(AutochatUiEvent::Completed {
            summary: "Autochat relay stub — not yet wired to backend.".into(),
            okr_id: None,
            okr_run_id: None,
            relay_id: None,
        });
    });
    let _ = model;
    rx
}
