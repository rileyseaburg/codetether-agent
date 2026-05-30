use crate::tui::app::state::App;

use super::types::FilePickerMode;

pub fn render_file_picker(f: &mut ratatui::Frame, area: ratatui::prelude::Rect, app: &App) {
    match app.state.file_picker.mode {
        FilePickerMode::Browse => {
            super::render_browser::render_browser(f, area, &app.state.file_picker);
        }
        FilePickerMode::Viewer => {
            super::render_viewer::render_viewer(f, area, &app.state.file_picker);
        }
    }
}
