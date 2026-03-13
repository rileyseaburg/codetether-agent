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
    let _ = app.state.refresh_available_models(registry).await;
    app.state.open_model_picker();
    app.state.set_view_mode(ViewMode::Model);
    if let Some(current_model) = session.metadata.model.as_deref()
        && let Some(index) = app
            .state
            .filtered_models()
            .iter()
            .position(|model| *model == current_model)
    {
        app.state.selected_model_index = index;
    }
    app.state.status = if app.state.available_models.is_empty() {
        "No models available".to_string()
    } else {
        "Model picker".to_string()
    };
}

pub fn close_model_picker(app: &mut App) {
    app.state.close_model_picker();
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
    return_to_chat(app);
}
