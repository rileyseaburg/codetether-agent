//! Application adapter for shared file picker navigation.
use crate::tui::app::state::App;

pub fn file_picker_filter_backspace(app: &mut App) {
    codetether_file_picker::file_picker_filter_backspace(&mut app.state.file_picker);
}

pub fn file_picker_filter_push(app: &mut App, c: char) {
    codetether_file_picker::file_picker_filter_push(&mut app.state.file_picker, c);
}
