use crate::tui::app::state::App;

use super::preview_cache::refresh_preview;
use super::types::FilePickerMode;

pub fn file_picker_select_prev(app: &mut App) {
    if app.state.file_picker.mode == FilePickerMode::Browse && app.state.file_picker.selected > 0 {
        app.state.file_picker.selected -= 1;
        refresh_preview(&mut app.state.file_picker);
    }
}

pub fn file_picker_select_next(app: &mut App) {
    let picker = &mut app.state.file_picker;
    if picker.mode == FilePickerMode::Browse && picker.selected + 1 < picker.entries.len() {
        picker.selected += 1;
        refresh_preview(picker);
    }
}

pub fn file_picker_page_up(app: &mut App) {
    if app.state.file_picker.mode == FilePickerMode::Viewer {
        app.state.file_picker.preview_scroll =
            app.state.file_picker.preview_scroll.saturating_sub(10);
    }
}

pub fn file_picker_page_down(app: &mut App) {
    if app.state.file_picker.mode == FilePickerMode::Viewer {
        let max_scroll = app
            .state
            .file_picker
            .preview
            .as_ref()
            .map(|preview| preview.lines.len().saturating_sub(1))
            .unwrap_or(0);
        app.state.file_picker.preview_scroll = app
            .state
            .file_picker
            .preview_scroll
            .saturating_add(10)
            .min(max_scroll);
    }
}
