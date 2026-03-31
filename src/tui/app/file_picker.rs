//! File picker UI (scaffold — implementation pending).
//!
//! This module provides a placeholder for the file picker view.
//! The full implementation will support browsing and selecting files
//! from the workspace to attach to messages.

use std::path::PathBuf;

use crate::tui::app::state::App;

/// A single entry in the file picker listing.
#[derive(Debug, Clone)]
pub struct FilePickerEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub name: String,
}

/// Open the file picker at the given working directory.
pub fn open_file_picker(app: &mut App, cwd: &std::path::Path) {
    app.state.file_picker_active = true;
    app.state.file_picker_dir = cwd.to_path_buf();
    app.state.file_picker_entries.clear();
    app.state.file_picker_selected = 0;
    app.state.file_picker_filter.clear();
    app.state.view_mode = crate::tui::models::ViewMode::FilePicker;
}

/// Handle Enter key in file picker — select current entry.
pub fn file_picker_enter(app: &mut App, _cwd: &std::path::Path) {
    // TODO: implement file selection / directory navigation
    app.state.file_picker_active = false;
    app.state.view_mode = crate::tui::models::ViewMode::Chat;
}

/// Handle Backspace in the filter input.
pub fn file_picker_filter_backspace(app: &mut App) {
    app.state.file_picker_filter.pop();
}

/// Render the file picker view (placeholder).
pub fn render_file_picker(_f: &mut ratatui::Frame, _area: ratatui::prelude::Rect, _app: &App) {
    // Placeholder — full implementation pending
}
