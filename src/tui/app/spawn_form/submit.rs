//! Submit logic: delegates to the existing spawn command handler.

use crate::tui::app::spawn_agent::handle_spawn_command;
use crate::tui::app::spawn_form::SpawnFormState;
use crate::tui::app::state::App;

/// Submit the form asynchronously, then close it.
///
/// Reuses [`handle_spawn_command`] so depth checks, duplicate-name
/// guards, and session creation are never duplicated.
pub async fn submit_spawn_form(app: &mut App, form: SpawnFormState) {
    let Some(args) = form.to_args() else {
        app.state.status = "Name is required".to_string();
        app.state.spawn_form = Some(form);
        return;
    };
    handle_spawn_command(app, &args).await;
    // Close the form on success (status unchanged by handler on success).
    if app.state.spawn_form.is_some() {
        app.state.spawn_form = None;
    }
}
