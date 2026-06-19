//! Helpers for [`super::slash_no_session`]: file attach and model picker.

use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::file_share::attach_file_to_input;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Attach a file to the input, or open the file picker when `cleaned` is empty.
pub(super) fn attach_file_command(app: &mut App, cwd: &Path, cleaned: &str) {
    if cleaned.is_empty() {
        crate::tui::app::file_picker::open_file_picker(app, cwd);
    } else {
        attach_file_to_input(app, cwd, Path::new(cleaned));
    }
}

/// Open the model picker view and kick off a background model refresh.
pub(super) fn open_model_view(app: &mut App, registry: &Option<Arc<ProviderRegistry>>) {
    app.state.open_model_picker();
    app.state.set_view_mode(ViewMode::Model);
    if let Some(registry) = registry {
        let cached = app.state.hydrate_models_from_store();
        app.state.status = if cached == 0 {
            "Loading models...".to_string()
        } else {
            format!("Model picker: refreshing {cached} cached models")
        };
        app.state.start_model_refresh(Arc::clone(registry));
    } else {
        app.state.status = "No models available".to_string();
    }
}
