//! Execute the autochat relay against a live provider.
//! This module resolves a provider/model pair and streams status back to the UI.

use tokio::sync::mpsc;

use super::events::AutochatUiEvent;

/// Run the relay flow and publish UI events.
pub async fn run_relay(
    task: String,
    model: String,
    ui_tx: mpsc::UnboundedSender<AutochatUiEvent>,
) {
    let _ = ui_tx.send(AutochatUiEvent::Progress(
        "Loading providers from Vault…".to_string(),
    ));
    let registry = match crate::provider::ProviderRegistry::from_vault().await {
        Ok(registry) => registry,
        Err(err) => return super::notify::send_registry_error(&ui_tx, err),
    };
    let model_ref = resolve_model_ref(model);
    let (provider, resolved_model) = match registry.resolve_model(&model_ref) {
        Ok(pair) => pair,
        Err(_) => return super::notify::send_no_provider_error(&ui_tx),
    };
    let _ = ui_tx.send(AutochatUiEvent::Progress(
        "Running autochat task with live provider…".to_string(),
    ));
    let request = super::request::build_request(task, resolved_model.clone());
    match provider.complete(request).await {
        Ok(response) => super::notify::send_success(&ui_tx, resolved_model, response),
        Err(err) => super::notify::send_completion_error(&ui_tx, err),
    }
}

fn resolve_model_ref(model: String) -> String {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        "codetether-autochat".to_string()
    } else {
        trimmed.to_string()
    }
}
