//! Application adapter for shared file picker navigation.
use crate::tui::app::state::App;

pub fn file_picker_select_first(app: &mut App) {
    codetether_file_picker::file_picker_select_first(&mut app.state.file_picker);
}

pub fn file_picker_select_last(app: &mut App) {
    codetether_file_picker::file_picker_select_last(&mut app.state.file_picker);
}
