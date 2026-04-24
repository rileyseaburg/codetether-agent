//! Multi-step relay execution for `/autochat`.
//!
//! Runs a short persona chain (architect → implementer → reviewer),
//! feeding each step's output forward as the baton, and streams
//! progress + per-step system messages + a final summary to the UI.

use tokio::sync::mpsc;

use super::events::AutochatUiEvent;
use super::persona::{Persona, default_chain};
use super::step_request::build_step_request;
use super::summary::summarize_response;

/// Run the relay flow and publish UI events.
pub async fn run_relay(task: String, model: String, ui_tx: mpsc::UnboundedSender<AutochatUiEvent>) {
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

    let _ = ui_tx.send(AutochatUiEvent::SystemMessage(format!(
        "Autochat using model: {resolved_model}"
    )));

    let chain = default_chain();
    let mut baton = String::new();

    for persona in chain {
        let _ = ui_tx.send(AutochatUiEvent::Progress(format!(
            "Relay step: {} thinking…",
            persona.name
        )));
        match run_step(&*provider, &resolved_model, persona, &task, &baton).await {
            Ok(output) => {
                let _ = ui_tx.send(AutochatUiEvent::SystemMessage(format!(
                    "[{}]\n{}",
                    persona.name, output
                )));
                baton = output;
            }
            Err(err) => return super::notify::send_completion_error(&ui_tx, err),
        }
    }

    super::notify::send_success_text(&ui_tx, resolved_model, baton);
}

async fn run_step(
    provider: &dyn crate::provider::Provider,
    resolved_model: &str,
    persona: &Persona,
    task: &str,
    baton: &str,
) -> anyhow::Result<String> {
    let request = build_step_request(resolved_model.to_string(), persona, task, baton);
    let response = provider.complete(request).await?;
    Ok(summarize_response(response))
}

fn resolve_model_ref(model: String) -> String {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        "codetether-autochat".to_string()
    } else {
        trimmed.to_string()
    }
}
