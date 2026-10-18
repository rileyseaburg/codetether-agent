//! Verifier-model command parsing, updates and picker initialization.
use super::View;
use codetether_model_discovery::Registry;
use std::sync::Arc;

/// Native verifier preference storage and composer operations.
/// # Implementors
/// Frontends implement this alongside the existing model-picker View.
pub trait CommandView: View {
    /// Replace the verifier override; None restores default precedence.
    fn set_verifier_model(&mut self, model: Option<&str>);
    /// Read the currently selected verifier model.
    fn selected_verifier_model(&self) -> Option<String>;
    /// Clear the submitted command input.
    fn clear_input(&mut self);
}
/// Handle `/verifier-model [model|clear]`; returns `false` for other input.
///
/// With no argument the model picker opens targeting the verifier. With a
/// model it is applied directly; `clear` restores the default precedence.
/// # Arguments
/// * `view` — Picker state, verifier preferences and input clearing.
/// * `registry` — Optional provider catalog for background refresh.
/// * `prompt` — Submitted command text.
/// # Returns
/// Whether this was a verifier-model command.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_model_picker::interactive::verifier_command::CommandView, R: codetether_model_discovery::Registry + Send + 'static>(view: &mut V) {
/// let _ = codetether_model_picker::interactive::verifier_command::run::<V, R>(view, None, "/verifier-model clear");
/// # }
/// ```
pub fn run<V: CommandView, R: Registry + Send + 'static>(
    view: &mut V,
    registry: Option<&Arc<R>>,
    prompt: &str,
) -> bool {
    let Some(rest) =
        codetether_slash_command::command_with_optional_args(prompt.trim(), "/verifier-model")
    else {
        return false;
    };
    match rest.trim() {
        "" => open(view, registry),
        "clear" | "default" => {
            view.set_verifier_model(None);
            view.set_status("Verifier model cleared; using env, worker, or default".into());
        }
        model => {
            view.set_verifier_model(Some(model));
            view.set_status(format!("Verifier model set: {model}"));
        }
    }
    view.clear_input();
    true
}

fn open<V: CommandView, R: Registry + Send + 'static>(view: &mut V, registry: Option<&Arc<R>>) {
    view.picker().open_model_picker();
    view.set_status("Model picker".to_string());
    view.show_picker();
    view.picker().model_picker_for_verifier = true;
    view.picker().model_picker_target_model = view.selected_verifier_model();
    if let Some(registry) = registry {
        view.picker().hydrate_models_from_store();
        view.picker().start_model_refresh(Arc::clone(registry));
    }
    view.set_status("Verifier model picker — the second LLM that decides goal completion".into());
}
