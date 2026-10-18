//! Native model-picker projection for newly spawned agents.
use crate::tui::app::state::App;
/// Return the current model id to show for a newly created sub-agent.
pub fn current_model_id(app: &App) -> Option<String> {
    codetether_agent_command::model::current(
        app.state.last_completion.model.as_ref(),
        &app.state.model_picker.available_models,
        app.state.model_picker.selected_model_index,
    )
}
