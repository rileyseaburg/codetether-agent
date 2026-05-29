use crate::tui::app::state::App;

use super::preview::load_preview;
use super::preview_cache::refresh_preview;
use super::scan::scan_directory;
use super::types::FilePickerMode;

pub fn file_picker_enter(app: &mut App, _cwd: &std::path::Path) {
    let Some(entry) = app
        .state
        .file_picker
        .entries
        .get(app.state.file_picker.selected)
        .cloned()
    else {
        return;
    };
    if entry.is_dir {
        app.state.file_picker.dir = entry.path;
        app.state.file_picker.entries = scan_directory(&app.state.file_picker.dir);
        app.state.file_picker.selected = 0;
        app.state.file_picker.filter.clear();
        refresh_preview(&mut app.state.file_picker);
    } else {
        app.state.file_picker.preview = Some(load_preview(&entry.path));
        app.state.file_picker.preview_scroll = 0;
        app.state.file_picker.mode = FilePickerMode::Viewer;
    }
}

pub fn file_picker_escape(app: &mut App) -> bool {
    if app.state.file_picker.mode == FilePickerMode::Viewer {
        app.state.file_picker.mode = FilePickerMode::Browse;
        true
    } else {
        false
    }
}
