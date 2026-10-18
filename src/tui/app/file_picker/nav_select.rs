//! Application adapter for shared file picker navigation.
use crate::tui::app::state::App;

pub fn file_picker_select_prev(app: &mut App) {
    codetether_file_picker::file_picker_select_prev(&mut app.state.file_picker);
}

pub fn file_picker_select_next(app: &mut App) {
    codetether_file_picker::file_picker_select_next(&mut app.state.file_picker);
}

pub fn file_picker_page_up(app: &mut App) {
    codetether_file_picker::file_picker_page_up(&mut app.state.file_picker);
}

pub fn file_picker_page_down(app: &mut App) {
    codetether_file_picker::file_picker_page_down(&mut app.state.file_picker);
}
