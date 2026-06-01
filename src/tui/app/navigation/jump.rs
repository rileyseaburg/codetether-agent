use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

pub fn handle_home(app: &mut App) {
    match app.state.view_mode {
        ViewMode::Chat => app.state.move_cursor_home(),
        ViewMode::FilePicker => crate::tui::app::file_picker::file_picker_select_first(app),
        _ => {}
    }
}

pub fn handle_end(app: &mut App) {
    match app.state.view_mode {
        ViewMode::Chat => app.state.move_cursor_end(),
        ViewMode::FilePicker => crate::tui::app::file_picker::file_picker_select_last(app),
        _ => {}
    }
}
