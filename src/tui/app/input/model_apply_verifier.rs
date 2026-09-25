//! Applying a model picker choice as the goal verifier model.

use crate::tui::app::session_sync::return_to_chat;
use crate::tui::app::state::App;

/// Set the highlighted picker model as the goal verifier model and close.
///
/// The verifier is the second LLM that independently decides whether a goal
/// is complete or blocked; see [`crate::tool::goal::verify`]. The choice is
/// stored with [`set_verifier_model`](crate::tool::goal::verify::set_verifier_model)
/// and applies to every later `update_goal` call in this process.
///
/// # Arguments
///
/// * `app` — TUI state whose model picker is open in verifier mode.
pub(super) fn apply_verifier_model(app: &mut App) {
    let model = app.state.selected_model().map(str::to_string);
    crate::tool::goal::verify::set_verifier_model(model.as_deref());
    app.state.status = match model {
        Some(model) => format!("Verifier model set: {model}"),
        None => "No verifier model selected".to_string(),
    };
    app.state.close_model_picker();
    app.state.model_picker_target_model = None;
    return_to_chat(app);
}
