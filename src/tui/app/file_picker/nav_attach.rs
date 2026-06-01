use crate::tui::app::state::App;

use super::attach::attach_path;

pub fn file_picker_attach(app: &mut App) {
    let path = preview_path(app)
        .filter(|p| p.is_file())
        .or_else(|| selected_path(app));
    if let Some(path) = path.filter(|path| path.is_file()) {
        attach_path(app, &path);
    }
}

fn preview_path(app: &App) -> Option<std::path::PathBuf> {
    app.state
        .file_picker
        .preview
        .as_ref()
        .map(|preview| preview.path.clone())
}

fn selected_path(app: &App) -> Option<std::path::PathBuf> {
    app.state
        .file_picker
        .entries
        .get(app.state.file_picker.selected)
        .map(|entry| entry.path.clone())
}
