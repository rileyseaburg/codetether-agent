//! `/verifier-model` command: choose the LLM that independently verifies goals.

use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tool::goal::verify::{selected_verifier_model, set_verifier_model};
use crate::tui::app::state::App;

/// Handle `/verifier-model [model|clear]`; returns `false` for other input.
///
/// With no argument the model picker opens targeting the verifier. With a
/// model it is applied directly; `clear` restores the default precedence.
pub(super) fn run(app: &mut App, registry: Option<&Arc<ProviderRegistry>>, prompt: &str) -> bool {
    let Some(rest) =
        crate::tui::app::text::command_with_optional_args(prompt.trim(), "/verifier-model")
    else {
        return false;
    };
    match rest.trim() {
        "" => open(app, registry),
        "clear" | "default" => {
            set_verifier_model(None);
            app.state.status = "Verifier model cleared; using env, worker, or default".into();
        }
        model => {
            set_verifier_model(Some(model));
            app.state.status = format!("Verifier model set: {model}");
        }
    }
    app.state.clear_input();
    true
}

fn open(app: &mut App, registry: Option<&Arc<ProviderRegistry>>) {
    app.state.open_model_picker();
    app.state.set_view_mode(crate::tui::models::ViewMode::Model);
    app.state.model_picker_for_verifier = true;
    app.state.model_picker_target_model = selected_verifier_model();
    if let Some(registry) = registry {
        app.state.hydrate_models_from_store();
        app.state.start_model_refresh(Arc::clone(registry));
    }
    app.state.status = "Verifier model picker — the second LLM that decides goal completion".into();
}
