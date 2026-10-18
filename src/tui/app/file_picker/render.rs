//! Render the shared picker from application state.
use crate::tui::app::state::App;
pub fn render_file_picker(f: &mut ratatui::Frame, area: ratatui::prelude::Rect, app: &App) {
    codetether_file_picker::render_file_picker(f, area, &app.state.file_picker);
}
