//! Shared-file actions: show git diff, and open in the file viewer.

use std::path::Path;

use crate::tui::app::file_picker::open_file_in_viewer;
use crate::tui::app::state::App;
use crate::tui::git_diff::capture_file_diff;
use crate::tui::models::ViewMode;

use super::resolve::latest_shared_file;

/// Show the git diff for the most recently shared file in the `/git` webview.
pub fn show_shared_file_diff(app: &mut App, workspace_dir: &Path) {
    let Some(file) = latest_shared_file(app, workspace_dir) else {
        app.state.status = "No shared file to diff.".to_string();
        return;
    };
    app.state.git.diff_stat = capture_file_diff(workspace_dir, &file);
    app.state.git.captured_at = chrono::Local::now().format("%H:%M:%S").to_string();
    app.state.save_scroll_state();
    app.state.view_mode = ViewMode::Git;
    app.state.status = format!("Diff: {}", file.display());
}

/// Open the most recently shared file in the standard TUI file viewer.
pub fn goto_shared_file_definition(app: &mut App, workspace_dir: &Path) {
    let Some(file) = latest_shared_file(app, workspace_dir) else {
        app.state.status = "No shared file to open.".to_string();
        return;
    };
    if !file.is_file() {
        app.state.status = format!("File not found: {}", file.display());
        return;
    }
    app.state.save_scroll_state();
    open_file_in_viewer(app, workspace_dir, &file);
}
