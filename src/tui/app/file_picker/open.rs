//! Application view and status updates around shared picker constructors.
use crate::tui::{app::state::App, models::ViewMode};
use std::path::Path;
pub fn open_file_picker(app: &mut App, cwd: &Path) {
    app.state.file_picker = codetether_file_picker::open_file_picker(cwd);
    app.state.view_mode = ViewMode::FilePicker;
    app.state.status = "File browser: Enter opens, Alt+a attaches, Esc closes".to_string();
}
pub fn open_file_in_viewer(app: &mut App, workspace_dir: &Path, file: &Path) {
    app.state.file_picker = codetether_file_picker::open_file_in_viewer(workspace_dir, file);
    app.state.view_mode = ViewMode::FilePicker;
    app.state.status = format!("Viewing {}", file.display());
}
