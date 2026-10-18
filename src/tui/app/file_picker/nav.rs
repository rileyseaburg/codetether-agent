//! Application adapter for shared file picker navigation.
use crate::tui::app::state::App;

pub fn file_picker_enter(app: &mut App, _cwd: &std::path::Path) {
    codetether_file_picker::file_picker_enter(&mut app.state.file_picker, _cwd);
}

pub fn file_picker_escape(app: &mut App) -> bool {
    codetether_file_picker::file_picker_escape(&mut app.state.file_picker)
}
