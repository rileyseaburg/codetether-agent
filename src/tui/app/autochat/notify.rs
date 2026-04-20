//! Send autochat UI completion and error events.

use tokio::sync::mpsc;

use super::events::AutochatUiEvent;

/// Notify the UI that provider loading failed.
pub fn send_registry_error(ui_tx: &mpsc::UnboundedSender<AutochatUiEvent>, err: anyhow::Error) {
    let _ = ui_tx.send(AutochatUiEvent::SystemMessage(format!(
        "Failed to load providers for /autochat: {err}"
    )));
    let _ = ui_tx.send(completed(
        "Autochat aborted: provider registry unavailable.".to_string(),
    ));
}

/// Notify the UI that no provider is configured.
pub fn send_no_provider_error(ui_tx: &mpsc::UnboundedSender<AutochatUiEvent>) {
    let _ = ui_tx.send(completed(
        "Autochat aborted: no providers configured.".to_string(),
    ));
}

/// Notify the UI about a successful completion.
pub fn send_success(
    ui_tx: &mpsc::UnboundedSender<AutochatUiEvent>,
    resolved_model: String,
    response: crate::provider::CompletionResponse,
) {
    let summary = super::summary::summarize_response(response);
    let _ = ui_tx.send(AutochatUiEvent::SystemMessage(format!(
        "Autochat used model: {resolved_model}"
    )));
    let _ = ui_tx.send(completed(summary));
}

/// Notify the UI that completion failed.
pub fn send_completion_error(
    ui_tx: &mpsc::UnboundedSender<AutochatUiEvent>,
    err: anyhow::Error,
) {
    let _ = ui_tx.send(completed(format!("❌ Autochat execution failed: {err}")));
}

fn completed(summary: String) -> AutochatUiEvent {
    AutochatUiEvent::Completed {
        summary,
        okr_id: None,
        okr_run_id: None,
        relay_id: None,
    }
}
