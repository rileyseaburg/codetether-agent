//! Multi-step relay execution for `/autochat`.
//!
//! Runs a short persona chain (architect → implementer → reviewer),
//! feeding each step's output forward as the baton, and streams
//! progress + per-step system messages + a final summary to the UI.

use tokio::sync::mpsc;

use super::events::AutochatUiEvent;
use super::persona_pick::{rank_chain, record_outcome, relay_bucket};
use super::persona_state;
use super::run_step::{resolve_model_ref, run_step};

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

    let bucket = relay_bucket();
    let chain = rank_chain(&persona_state::lock(), bucket);
    let mut baton = String::new();

    for persona in &chain {
        let _ = ui_tx.send(AutochatUiEvent::Progress(format!(
            "Relay step: {} thinking…",
            persona.name
        )));
        match run_step(&*provider, &resolved_model, persona, &task, &baton).await {
            Ok(output) => {
                record_outcome(&mut persona_state::lock(), persona.name, bucket, true);
                let _ = ui_tx.send(AutochatUiEvent::SystemMessage(format!(
                    "[{}]\n{}",
                    persona.name, output
                )));
                baton = output;
            }
            Err(err) => {
                record_outcome(&mut persona_state::lock(), persona.name, bucket, false);
                return super::notify::send_completion_error(&ui_tx, err);
            }
        }
    }

    super::notify::send_success_text(&ui_tx, resolved_model, baton);
}
