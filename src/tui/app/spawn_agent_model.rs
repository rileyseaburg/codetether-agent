//! Model selection snapshot for spawned TUI agents.

use crate::tui::app::state::App;

/// Return the current model id to show for a newly created sub-agent.
pub fn current_model_id(app: &App) -> Option<String> {
    app.state.last_completion_model.clone().or_else(|| {
        app.state
            .available_models
            .get(app.state.selected_model_index)
            .cloned()
    })
}
