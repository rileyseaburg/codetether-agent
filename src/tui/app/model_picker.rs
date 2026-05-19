use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::session_sync::return_to_chat;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

pub async fn open_model_picker(
    app: &mut App,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
) {
    app.state.open_model_picker();
    app.state.set_view_mode(ViewMode::Model);
    app.state.model_picker_target_model = session.metadata.model.clone();

    if let Some(current_model) = app.state.model_picker_target_model.as_deref()
        && let Some(index) = app
            .state
            .filtered_models()
            .iter()
            .position(|model| *model == current_model)
    {
        app.state.selected_model_index = index;
    }

    if let Some(registry) = registry {
        let cached_count = app.state.available_models.len();
        app.state.status = if cached_count == 0 {
            "Loading models...".to_string()
        } else {
            format!("Model picker: refreshing {cached_count} cached models")
        };
        app.state.start_model_refresh(Arc::clone(registry));
    } else {
        app.state.available_models.clear();
        app.state.model_refresh_in_flight = false;
        app.state.model_refresh_rx = None;
        app.state.status = "No models available".to_string();
    }
}

pub fn close_model_picker(app: &mut App) {
    app.state.close_model_picker();
    app.state.model_picker_target_model = None;
    return_to_chat(app);
}

pub fn apply_selected_model(app: &mut App, session: &mut Session) {
    if let Some(model) = app.state.selected_model() {
        session.metadata.model = Some(model.to_string());
        app.state.status = format!("Model set: {model}");
    } else {
        app.state.status = "No model selected".to_string();
    }
    app.state.close_model_picker();
    app.state.model_picker_target_model = None;
    return_to_chat(app);
}
