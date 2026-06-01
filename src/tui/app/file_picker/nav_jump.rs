use crate::tui::app::state::App;

use super::preview_cache::refresh_preview;
use super::types::FilePickerMode;

pub fn file_picker_select_first(app: &mut App) {
    let picker = &mut app.state.file_picker;
    if picker.mode == FilePickerMode::Browse && !picker.entries.is_empty() {
        picker.selected = 0;
        refresh_preview(picker);
    }
}

pub fn file_picker_select_last(app: &mut App) {
    let picker = &mut app.state.file_picker;
    if picker.mode == FilePickerMode::Browse && !picker.entries.is_empty() {
        picker.selected = picker.entries.len() - 1;
        refresh_preview(picker);
    }
}
